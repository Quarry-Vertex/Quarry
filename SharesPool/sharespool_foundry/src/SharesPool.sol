pragma solidity ^0.8.13;

import "./PoolShares.sol";
import "./QuarryBTC.sol";
import "./SharesRingBuffer.sol";

/*
General Design Diagram:

                    (submits blocks)                  (submits btc data)
Stratum Mining Pool     -->             SharesPool          <--             Blockchain Data Oracle/Service

                                            ^
                                            |  (redeem shares)
                                            |
                (request peg out)
        Miners       --->            Bridge Contract // this would need a btc address that can receive/send native BTC,
                                                     // potentially same wallet as the one that receives mining rewards?

*/

contract SharesPool is SharesRingBuffer {
    uint256 constant SHARES_RING_BUFFER_SIZE = 500; // TODO: Set sharesRingBuffer size to correct value

    address stratumPool;
    address chainTipOracle;
    address quarryPegInAddress;

    modifier onlyOracle() {
        require(msg.sender == chainTipOracle, "Only the chainTipOracle can call this method");
        _;
    }

    modifier onlyStratumPool() {
        require(msg.sender == stratumPool, "Only the stratumPool can call this method");
        _;
    }

    /*
        Difficulty Threshold Calculation:
            bitcoin_exahash = 10**18
            network_hash_rate_bitcoin = 500 * bitcoin_exahash  # Example: 500 Exahash/s for Bitcoin
            target_block_time_bitcoin = 600  # 10 minutes
            miner_hash_rate = 5 * bitcoin_exahash  # 5 Exahash/s
            Formula: difficulty = network_hash_rate_bitcoin / (target_block_time_bitcoin * miner_hash_rate) = 2*10^13
    */
    uint256 constant private DIFFICULTY_THRESHOLD = 20000000000000; // 2 * 10^13

    PoolShares shares; // Shares NFT instance
    uint256 sharesId = 0;

    QuarryBTC quarryBTC; // synthetic BTC

    mapping(bytes32 => uint8) public confirmations; // tracks number of confirmations for each block hash
    bytes32[] blocks; // list of block hashes from setChainTip

    mapping(bytes32 => address) public commits; // tracks the address that has committed a block hash

    mapping(bytes32 => bool) public usedBlockHashes; // tracks whether a block hash has already been used

    ChainTip public chainTip;

    event Transfer(
        address indexed from,
        address indexed to,
        uint256 value
    );

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );

    struct ChainTip {
        bytes32 previousBlockHash;
        bytes32 merkleRootHash;
    }

    struct BlockHeader {
        uint32 version;
        bytes32 previousBlockHash;
        bytes32 merkleRootHash;
        uint32 timestamp;
        uint32 bits;
        uint32 nonce;
    }

    // From https://en.bitcoin.it/wiki/Block + https://en.bitcoin.it/wiki/Transaction
    struct BitcoinBlock {
        BlockHeader header;

        // Flattened Transactions
        bytes8[][] outputValues;
        bytes32[][] outputScripts;
    }

    constructor(string memory _oracleAddress) SharesRingBuffer(SHARES_RING_BUFFER_SIZE) {
        chainTipOracle = address(bytes20(bytes32(uint256(keccak256(abi.encodePacked(_oracleAddress))))));
        shares = new PoolShares("Quarry", "QRY", ""); // TODO: Fill in baseTokenURI
        chainTip = ChainTip("", "");

        // TODO: need to set quarryPegInAddress and stratumPool
    }

    // To prevent work from being "stolen" (man in the middle attack) miners should reveal a HASH(Block hash + Destination
    // Quarry address) first then submits the rest of the block and the destination Quarry address to be credited with the pool share.
    function submitHash(bytes32 _blockHash, address _account) public {
        commits[_blockHash] = _account;
    }

    function _calculateDifficulty(uint32 _bits) private pure returns (uint256) {
        uint256 maxTarget = 0xFFFF * 256**(0x1D - 3);
        uint256 target = (_bits & 0xFFFFFF) * 256**(_bits >> 24 - 3);
        uint256 difficulty = maxTarget / target;
        return difficulty;
    }

    function setChainTip(ChainTip memory _chainTip) public onlyOracle {
        // check if the merkleRoot hasn't been populated the chain tip hasn't been set
        if (_chainTip.merkleRootHash != "") {
            require(_chainTip.previousBlockHash == chainTip.merkleRootHash,
                "New chain tip prev block hash does not match current chain tip block hash");
        }

        chainTip = _chainTip;

        // increment number of confirmations for each block
        for (uint256 i = 0; i < blocks.length; i++) {
            confirmations[blocks[i]]++;
        }

        blocks.push(_chainTip.merkleRootHash);
    }

    /*
    - Keep track of which addresses have how many shares (mapping of address to number of shares)
    - Checks should be:
        * Address must match the one that has been committed
        * The block hash has not been submitted to the pool before (thus all submitted block hashes should be kept in a hashmap/set data structure on chain
            - this hashmap can be cleared when a new Bitcoin block is found and the chain tip is updated)
        * The block header of the submitted block meets the pool difficulty (which will be fixed - for now let's just make it represent 5 Exahash of work
            - which means assuming the whole network is mining on Quarry there should be 100 new pool shares per second, since Bitcoin's hashrate is 500 Eh/s)
        * The previous block hash (written in the current block's block header) is the Bitcoin chain tip for the fork with the most accumulated PoW
        * A merkle proof (ie SPV proof) that the Coinbase transaction of the block is pointed to the current peg in address
    */
    function submitBlockHeader(BitcoinBlock memory _block, bytes32[] memory _merklePath, address _account) public returns (bool success) {
        bytes32 blockHash = _block.header.merkleRootHash;
        // Address must match the one that has been committed and block hash has not been submitted to pool before
        require(
            commits[blockHash] != _account,
            "Address doesn't match account"
        );

        require(
            !usedBlockHashes[blockHash],
            "Block hash has already been submitted"
        );

        uint256 difficulty = _calculateDifficulty(_block.header.bits);

        // double check units match up here
        require(difficulty < DIFFICULTY_THRESHOLD, "Pool difficulty not met");

        // check that previous block hash is the bitcoin chain tip for the fork with the most accumulated PoW
        bytes32 prevHash = _block.header.previousBlockHash;
        require(prevHash == chainTip.merkleRootHash, "Submitted block is stale");

        /*
         Let's say we have the following Merkle tree for four transactions (A, B, C, D):

                 ROOT <- merkle root
                /    \
               AB     *CD
              /  \    /  \
           A(tx)  B* C    D

            If we want to prove that transaction A is in the Merkle tree
            the Merkle path would be the hash of B (sibling of A) and CD (sibling of AB)
            represented as an array: [B, CD].
         */
        // Merkle proof that Coinbase tx is pointed to current peg in address of the mining pool
        require(scriptPubKeyToAddress(extractScriptPubKey(_block.outputScripts[0][0])) == quarryPegInAddress,
            "Coinbase transaction does not point to quarry peg in address");

        bytes32 curHash = _merklePath[0]; // this is wrong, need to get the tx hash
        for (uint256 i = 1; i < _merklePath.length; i++) { // walk the merkle path
            // get the current hash's sibling
            bytes32 sibling = _merklePath[i];
            // get the new current hash
            if (curHash < sibling) {
                curHash = sha256(abi.encodePacked(sha256(abi.encodePacked(curHash, sibling))));
            } else {
                curHash = sha256(abi.encodePacked(sha256(abi.encodePacked(sibling, curHash))));
            }
        }
        require(curHash == blockHash, "SPV proof failed");

        usedBlockHashes[blockHash] = true;

        // All checks pass, credit user with share
        uint256 newShareId = shares.awardShare(_account, sharesId++);
        if (ringBufferIsFull()) {
            uint256 burnTokenId = popFromRingBuffer();
            shares.burnShare(burnTokenId);
        }
        pushToRingBuffer(newShareId);

        return true;
    }

    // This method assumes Pay-to-Script-Hash (P2SH)
    function extractScriptPubKey(bytes32 script) public pure returns (bytes25) {
        require(script.length == 25, "Invalid script length");

        // Ensure the script follows the P2SH format
        require(script[0] == 0xa9 && script[script.length - 1] == 0x87, "Not a P2SH script");

        // Extract the scriptPubKey by removing OP_HASH160 and OP_EQUAL operations
        bytes25 scriptPubKey;
        assembly {
            // Point to the free memory slot
            let dest := add(scriptPubKey, 32)
            // Point to the source in script
            let src := add(script, 33)
            // Copy 24 bytes from src to dest
            for { let i := 0 } lt(i, 24) { i := add(i, 1) } {
                mstore8(add(dest, i), mload(add(src, i)))
            }
        }

        return scriptPubKey;
    }

    // This method assumes Pay-to-Script-Hash (P2SH)
    function scriptPubKeyToAddress(bytes25 scriptPubKey) internal pure returns (address) {
        bytes20 versionByteP2SH = hex"05"; // Version byte for P2SH addresses on mainnet

        require(scriptPubKey.length >= 25, "Invalid scriptPubKey length");

        if (scriptPubKey[0] == 0xa9 && scriptPubKey[scriptPubKey.length - 1] == 0x87) {
            // Check if it's a P2SH scriptPubKey

            // Extract the scriptHash
            bytes20 scriptHash;
            assembly {
                scriptHash := mload(add(add(scriptPubKey, 0x21), 0))
            }

            // Create the address by concatenating version byte and script hash
            bytes memory addressBytes = abi.encodePacked(versionByteP2SH, scriptHash);
            return address(uint160(uint256(keccak256(addressBytes))));
        } else {
            revert("Not a P2SH scriptPubKey");
        }
    }

    // Clears out all shares and distributes rewards prorata to addresses
    // This function should be called by the Stratum mining pool when blocks are won
    function distributeRewards(BitcoinBlock memory _block) public onlyStratumPool returns (bool success) {
        require(confirmations[_block.header.merkleRootHash] < 6, "Do not have 6+ confirmations");

        uint256 numShares = numSharesInRingBuffer();
        bytes8 blockReward = _block.outputValues[0][0];
        uint256 blockRewardPerShare = uint64(blockReward) / numShares;
        while (ringBufferIsEmpty()) {
            uint256 burnTokenId = popFromRingBuffer();
            address shareOwner = shares.getOwnerOfShare(burnTokenId);
            quarryBTC.mintQuarryBTC(shareOwner, blockRewardPerShare);
            shares.burnShare(burnTokenId);
        }

        return true;
    }
}
