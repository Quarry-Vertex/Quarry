// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "./Share.sol";
import "./QBTC.sol";
import "./lib/RingBuffer.sol";
import "./lib/SPVProof.sol";

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "forge-std/console.sol";

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

contract Pool is Initializable, OwnableUpgradeable, SPVProof, RingBuffer {
    /*
        Difficulty Threshold Calculation:
        https://bitcoin.stackexchange.com/questions/5556/relationship-between-hash-rate-and-difficulty
        5 EH/s = 5000000000000000000 H/s
        difficulty = hashrate / (2^256 / max_target / intended_time_per_block)
            = hashrate / (2^256 / (2^208*65535) / 600)
            = hashrate / (2^48 / 65535 / 600)
            = hashrate / 7158388.055
            = 5000000000000000000 / 7158388.055
            = ~698481272821
    */
    uint256 DIFFICULTY_THRESHOLD;
    uint256 DIFFICULTY_SCALING;

    address stratumPoolAddress;
    address chainTipOracle;
    bytes32 quarryPegInAddress;

    modifier onlyOracle() {
        require(msg.sender == chainTipOracle, "Only the chainTipOracle can call this method");
        _;
    }

    modifier onlyStratumPool() {
        require(msg.sender == stratumPoolAddress, "Only the stratumPool can call this method");
        _;
    }

    Share shares; // Shares NFT instance
    uint256 sharesId;

    QBTC qbtc; // synthetic BTC

    mapping(bytes32 => uint8) public confirmations; // tracks number of confirmations for each block hash
    bytes32[] blocks; // list of block hashes from setChainTip

    mapping(bytes32 => address) public commits; // tracks the address that has committed a block hash

    mapping(bytes32 => bool) public usedBlockHashes; // tracks whether a block hash has already been used

    ChainTip public chainTip;

    event ChainTipSet(
        bytes32 merkleRootHash
    );

    event HashCommitted(
        bytes32 blockHash,
        address account
    );

    event BlockRevealed(
        bytes32 blockHash,
        address account
    );

    event RewardsDistributed(
        bytes32 blockHash
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

        // Transactions
        bytes32 outputAddress;
        bytes8 blockReward;
    }

    function getOneHundred() public view returns (uint256) {
        return 100;
    }

    function setShareContract(address _poolSharesAddress) public onlyOwner {
        shares = Share(_poolSharesAddress);
    }

    function setQBTCContract(address _qbtcAddress) public onlyOwner {
        qbtc = QBTC(_qbtcAddress);
    }

    function initialize(address _oracleAddress, address _stratumPoolAddress, bytes32 _pegInAddress, uint256 _ringBufferSize) public initializer {
        // Used to limit setting token contract addresses to deploying address
        __Ownable_init(msg.sender);

        DIFFICULTY_SCALING = 10**10;
        DIFFICULTY_THRESHOLD = 698481272821 * DIFFICULTY_SCALING;

        RingBuffer.initialize(_ringBufferSize);
        SPVProof.initialize();

        chainTipOracle = _oracleAddress;
        stratumPoolAddress = _stratumPoolAddress;
        quarryPegInAddress = _pegInAddress;

        chainTip = ChainTip("", "");
        sharesId = 0;

    }

    function setChainTip(ChainTip memory _chainTip) public onlyOracle {
        // check if the merkleRoot hasn't been populated the chain tip hasn't been set
        if (_chainTip.merkleRootHash != "") {
            require(_chainTip.previousBlockHash == chainTip.merkleRootHash,
                "New chain tip prev block hash does not match current chain tip block hash");
        }

        chainTip = _chainTip;

        emit ChainTipSet(_chainTip.merkleRootHash);

        // increment number of confirmations for each block
        for (uint256 i = 0; i < blocks.length; i++) {
            confirmations[blocks[i]]++;
        }

        blocks.push(_chainTip.merkleRootHash);
    }

    function getChainTip() public onlyOracle view returns (ChainTip memory tip) {
        return chainTip;
    }

    // To prevent work from being "stolen" (man in the middle attack) miners should reveal a HASH(Block hash + Destination
    // Quarry address) first then submits the rest of the block and the destination Quarry address to be credited with the pool share.
    function submitHash(bytes32 _blockHash, address _account) public {
        commits[_blockHash] = _account;
        emit HashCommitted(_blockHash, _account);
    }

    function getAddressForSubmittedHash(bytes32 _blockHash) public view returns (address account) {
        return commits[_blockHash];
    }

    function _ldexp(uint256 val, uint256 exp) private pure returns (uint256) {
        return val * (2**exp);
    }

    // convert bits to difficulty
    function _calculateDifficulty(uint32 _bits) public view returns (uint256) {
        uint256 exponent_diff = 8 * (0x1D - ((_bits >> 24) & 0xFF));
        uint256 significand = _bits & 0xFFFFFF;
        uint256 difficulty = _ldexp((0x00FFFF*DIFFICULTY_SCALING) / significand, exponent_diff);
        return difficulty;
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
    function submitBlock(BitcoinBlock memory _block, bytes32[] memory _merklePath, address _account) public returns (bool success) {
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

        // Difficulty of block must be less than the threshold
        uint256 difficulty = _calculateDifficulty(_block.header.bits);
        require(difficulty < DIFFICULTY_THRESHOLD * DIFFICULTY_SCALING, "Pool difficulty not met");

        // check that previous block hash is the bitcoin chain tip for the fork with the most accumulated PoW
        bytes32 prevHash = _block.header.previousBlockHash;
        require(prevHash == chainTip.merkleRootHash, "Submitted block is stale");

        // Check that the Coinbase tx is pointed to current peg in address of the mining pool
        require(_block.outputAddress == quarryPegInAddress,
            "Coinbase transaction does not point to quarry peg in address");

        // SPV Proof TODO: Confirm this logic is correct
        spvProof(_merklePath, blockHash);

        usedBlockHashes[blockHash] = true;

        // All checks pass, credit user with share
        uint256 newShareId = shares.mint(_account, sharesId++);
        if (ringBufferIsFull()) {
            uint256 burnTokenId = popFromRingBuffer();
            shares.burn(burnTokenId); // fails
        }
        pushToRingBuffer(newShareId);

        confirmations[blockHash] = 0;

        emit BlockRevealed(_block.header.merkleRootHash, _account);

        return true;
    }

    // Clears out all shares and distributes rewards prorata to addresses
    // This function should be called by the Stratum mining pool when blocks are won
    function distributeRewards(BitcoinBlock memory _block) public onlyStratumPool returns (bool success) {
        require(confirmations[_block.header.merkleRootHash] < 6, "Do not have 6+ confirmations");

        uint256 numShares = numSharesInRingBuffer();
        bytes8 blockReward = _block.blockReward;
        uint256 blockRewardPerShare = uint64(blockReward) / numShares;
        while (!ringBufferIsEmpty()) {
            uint256 burnTokenId = popFromRingBuffer();
            address shareOwner = shares.ownerOf(burnTokenId);
            qbtc.mint(shareOwner, blockRewardPerShare);
            shares.burn(burnTokenId);
        }

        emit RewardsDistributed(_block.header.merkleRootHash);

        return true;
    }
}
