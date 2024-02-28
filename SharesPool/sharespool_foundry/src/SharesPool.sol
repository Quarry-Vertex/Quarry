pragma solidity ^0.8.13;

import "./SharesQueue.sol";

contract SharesPool {
    address chainTipOracle;
    address quarryPegInAddress;

    modifier onlyOracle() {
        require(msg.sender == chainTipOracle);
        _;
    }

    // bitcoin_exahash = 10**18
    // network_hash_rate_bitcoin = 500 * bitcoin_exahash  # Example: 500 Exahash/s for Bitcoin
    // target_block_time_bitcoin = 600  # 10 minutes
    // miner_hash_rate = 5 * bitcoin_exahash  # 5 Exahash/s
    // Formula: difficulty = network_hash_rate_bitcoin / (target_block_time_bitcoin * miner_hash_rate) = 2*10^13
    uint256 constant private DIFFICULTY_THRESHOLD = 20000000000000; // 2 * 10^13

    mapping(address => uint256) public syntheticBTCBalances; // balance of each user of credited synthetic BTC

    uint256 constant SHARES_QUEUE_CAPACITY = 500; // TODO: Set SharesQueue capacity to correct value
    PoolShares shares; // Shares NFT instance
    SharesQueue sharesQueue; // tracks the PPLNS shares to be credited if a block is found
    uint256 sharesId = 0;

    mapping(bytes32 => uint8) public confirmations; // tracks number of confirmations for each block hash
    bytes32[] blocks; // list of blocks from setChainTip

    mapping(bytes32 => address) public commits; // tracks the address that has committed a block hash

    mapping(address => mapping(address => uint256)) public allowedSharesTransfer; // tracks approvals for transfering shares
    mapping(address => mapping(address => uint256)) public allowedBTCTransfer; // tracks approvals for transfering synthetic BTC

    mapping(bytes32 => bool) public usedBlockHashes; // tracks whether a block hash has already been used

    BitcoinBlock public chainTip;

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

    uint32 constant magic = 0xD9B4BEF9;

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
        uint32 magic;
        uint32 blockSize;
        BlockHeader header;
        uint256 transactionCounter;

        // Flattened Transaction structure
        uint32[] transactionVersions;
        bytes[][] transactionFlags;
        bytes9[] transactionInputsCounters;
        bytes32[][] inputOutpointHashes;
        bytes4[][] inputOutpointIndices;
        bytes9[][] inputScriptLengths;
        bytes32[][] inputScriptSignatures;
        bytes4[][] inputSequenceNums;
        bytes9[] transactionOutputsCounters;
        bytes8[][] outputValues;
        bytes9[][] outputScriptLengths;
        bytes32[][] outputScripts;
        bytes32[] transactionLockTimes;
    }

    constructor() {
        sharesQueue = new SharesQueue(SHARES_QUEUE_CAPACITY);

        // TODO: need to set quarryPegInAddress
        // TODO: need to set chainTipOracle, but not sure what this should be as this isn't a smart contract
    }

    // Submits _account to be credited for the work of _blockHash
    function submitHash(bytes32 _blockHash, address _account) public returns (bool success) {
        commits[_blockHash] = _account;
        return true;
    }

    function _calculateDifficulty(uint32 _bits) private pure returns (uint256) {
        uint256 maxTarget = 0xFFFF * 256**(0x1D - 3);
        uint256 target = (_bits & 0xFFFFFF) * 256**(_bits >> 24 - 3);
        uint256 difficulty = maxTarget / target;
        return difficulty;
    }

    // What do we need to submit to know what a block was won by us?
    // Is this that the coinbase transaction points to the peg in address
    function setChainTip(BitcoinBlock memory _chainTip) public onlyOracle {
        require(_chainTip.header.previousBlockHash == chainTip.header.merkleRootHash,
            "New tip previous block hash does not match current chain tip block hash");

        chainTip = _chainTip;

        // increment number of confirmations for each block
        for (uint256 i = 0; i < blocks.length; i++) {
            confirmations[blocks[i]]++;
        }

        // If block is won by quarry mining pool (coinbase transaction points to peg in address), call distributeRewards
        bytes25 scriptPubKey = extractScriptPubKey(_chainTip.outputScripts[0][0]);
        address coinbaseTxAddress = scriptPubKeyToAddress(scriptPubKey);
        if (coinbaseTxAddress == quarryPegInAddress)
            distributeRewards(_chainTip);
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
    function submitBlockHeader(BlockHeader memory _blockHeader, bytes32[] memory _merklePath, address _account) public returns (bool success) {
        bytes32 blockHash = _blockHeader.merkleRootHash;
        // Address must match the one that has been committed and block hash has not been submitted to pool before
        require(
            commits[blockHash] != _account,
            "Address Doesn't Match account"
        );

        require(
            !usedBlockHashes[blockHash],
            "Block hash has already been submitted"
        );

        uint256 difficulty = _calculateDifficulty(_blockHeader.bits);

        // double check units match up here
        require(difficulty < DIFFICULTY_THRESHOLD, "difficulty not met");

        // check that previous block hash is the bitcoin chain tip for the fork with the most accumulated PoW
        bytes32 prevHash = _blockHeader.previousBlockHash;
        require(prevHash == chainTip.header.merkleRootHash, "submitted block is stale");

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
        require(curHash == blockHash, "spv proof failed");

        usedBlockHashes[blockHash] = true;

        // All checks pass, credit user with share
        uint256 newShareId = shares.awardShare(_account, sharesId++);
        if (sharesQueue.isFull()) {
            uint256 burnTokenId = sharesQueue.dequeue();
            shares.burnShare(burnTokenId);
        }
        sharesQueue.enqueue(newShareId);

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

    // This function should be called by the oracle when rewards are won
    function distributeRewards(BitcoinBlock memory _block) public returns (bool success) {
        // TODO: Verify that the block is a valid, won block and should probably double check that coinbase transaction points to peg in address
        // I think this involves translating the script field of the output of the coinbase transaction in some way to get the peg in address

        require(confirmations[_block.header.merkleRootHash] < 6, "Does not have 6+ confirmations");

        uint256 numShares = sharesQueue.size();
        // Transaction[] memory blockTransactions = _block.transactions;
        // bytes8 blockReward = blockTransactions[0].outputs[0].value;
        uint256 blockReward = 0xFFFFFFFFFFFFFFFF; // TODO: Figure out how to pull out block reward
        uint256 blockRewardPerShare = blockReward / numShares;
        while (!sharesQueue.isEmpty()) {
            uint256 burnTokenId = sharesQueue.dequeue();
            address shareOwner = shares.getOwnerOfShare(burnTokenId);
            syntheticBTCBalances[shareOwner] += blockRewardPerShare;
            shares.burnShare(burnTokenId);
        }

        return true;
    }

    /* Next 3 methods allow for the approval and transfer of synthetic BTC */

    // Transfers _numSats from sender to _to
    function transferBTC(address _to, uint256 _numSats) public returns (bool success) {
        require(syntheticBTCBalances[msg.sender] >= _numSats, "Do not have enough synthetic BTC");

        syntheticBTCBalances[msg.sender] -= _numSats;
        syntheticBTCBalances[_to] += _numSats;

        emit Transfer(msg.sender, _to, _numSats);

        return true;
    }

    // Approves _numSats to be transferred from _spender's account
    function approveBTCTransfer(address _spender, uint256 _numSats) public returns (bool success) {
        allowedBTCTransfer[msg.sender][_spender] = _numSats;

        emit Approval(msg.sender, _spender, _numSats);

        return true;
    }

    // Transfers _numSats from _from to _to, provided >= _numSats have been approved from the spender
    function transferBTCFrom(address _from, address _to, uint256 _numSats) public returns (bool success) {
        require(_numSats <= syntheticBTCBalances[_from], "Do not have enough synthetic BTC");
        require(_numSats <= allowedBTCTransfer[_from][msg.sender], "Not enough synthetic BTC was approved for transfer");

        syntheticBTCBalances[_from] -= _numSats;
        syntheticBTCBalances[_to] += _numSats;

        allowedBTCTransfer[_from][msg.sender] -= _numSats;

        emit Transfer(_from, _to, _numSats);

        return true;
    }
}

/*
General Design:

submitHash(address, hash)
- To prevent work from being "stolen" (man in the middle attack) we should implement a "commit-reveal" scheme in which
someone reveals a HASH(Block hash + Destination Quarry address) first and then submits the rest of the block and the
destination Quarry address to be credited with the pool share.

submitBlock(address, block header, merkle branch)
- Keep track of which addresses have how many shares (mapping of address to number of shares)
- Translator proxy has to communicate to this (but other participants can as well)
- Must reject stale shares
- Checks should be:
    * Address must match the one that has been committed
    * A merkle proof (ie SPV proof) that the Coinbase transaction of the block is pointed to the current peg in address
    * The block header of the submitted block meets the pool difficulty (which will be fixed - for now let's just make it represent 5 Exahash of work
       - which means assuming the whole network is mining on Quarry there should be 100 new pool shares per second, since Bitcoin's hashrate is 500 Eh/s)
    * The previous block hash (written in the current block's block header) is the Bitcoin chain tip for the fork with the most accumulated PoW
    * The block hash has not been submitted to the pool before (thus all submitted block hashes should be kept in a hashmap/ set data structure on chain
       - this hashmap can be cleared when a new Bitcoin block is found and the chain tip is updated)

transferShares(addressToTransferTo, numShares)

distributeRewards(bitcoin block)
- Clears out all shares and distributes rewards prorata to addresses (make sure to delete shares before distributing
- Probably called from the smart contract that was verifying work (if it gets a block that meets the difficulty of Bitcoin,
  it should broadcast the block while alerting the shares tracker smart contract, after 6 confirmations)


The work verifier smart contract only has to check for SPV proofs, but it should have access to the whole block that was hashed.
Because if the block is actually a winning Bitcoin block, we need to be able to broadcast that to the Bitcoin network.

                    (submits blocks)                  (submits btc data)
Stratum Mining Pool     -->             SharesPool          <--             Blockchain Data Oracle/Service

                                            ^
                                            |  (redeem shares)
                                            |
                (request peg out)
        Miners       --->            Bridge Contract // this would need a btc address to send btc to


*/
