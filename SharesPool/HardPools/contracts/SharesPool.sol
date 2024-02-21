pragma solidity >=0.6.0 <0.9.0;

contract SharesPool {
    /*
        Formula: difficulty = network_hash_rate / (target_block_time * miner_hash_rate) = 2*10^13
        bitcoin_exahash = 10**18
        network_hash_rate_bitcoin = 500 * bitcoin_exahash  # Example: 500 Exahash/s for Bitcoin
        target_block_time_bitcoin = 600  # 10 minutes
        miner_hash_rate = 5 * bitcoin_exahash  # 5 Exahash/s
    */

    address chainTipOracle;

    modifier onlyOwner() {
        require(msg.sender == chainTipOracle);
        _;
    }

    uint256 constant private DIFFICULTY_THRESHOLD = 20000000000000; // 2 * 10^13

    mapping(address => uint256) public syntheticBTCBalances; // balance of each user of credited synthetic BTC, TODO: I don't think solidity supports floats? Satoshis? 

    uint256 totalShares; // number of total shares, used to calculate how many rewards to distribute
    mapping(address => uint256) public sharesBalances; // tracks number of shares per user (confirmed)

    address[] public pendingUsers; // corresponding array to pending balances mapping
    bytes32[] public pendingBlocks; // submitted blocks that are pending 6+ confirmations
    mapping(bytes32 => uint8) public pendingBlocksConf; // submitted blocks, tracks number of confirmations
    mapping(bytes32 => address) public pendingBlocksSubmitter; // submitted blocks, tracks who submitted them

    mapping(bytes32 => address) public commits; // tracks the address that has committed a block hash

    mapping(address => mapping(address => uint256)) public allowance; // tracks approvals for transfering shares

    mapping(bytes32 => bool) public usedBlockHashes; // tracks whether a block hash has already been used

    BitcoinBlock public chainTip;

    /*
    full node for **quarry needs to be aware of the btc blockchain**
    that awareness has to be engineered into the **L2**
    for now assume you have an API or feed that gives the btc state
    make a dummy function that gives you the state
    pool smartcontract has to be aware of BTC. on EVM chains that isn't the case
    trivial (mandate every full Q node is running a BTC fullnode and get tip from that node)
    make a plugin function
    */

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

    // From https://en.bitcoin.it/wiki/Transaction

    struct Input {
        bytes32 outpointHash;
        bytes4 outpointIndex;
        bytes9 scriptLength;
        bytes32 scriptSignature; // Information required to spend the output -- not sure what is actually is yet, need to do more research (type might be wrong)
        bytes4 sequenceNum;
    }

    struct Output {
        bytes8 value; // The monetary value of the output in satoshis, I believe this is the value of the rewards?
        bytes9 scriptLength;
        bytes32 script; // A calculation which future transactions need to solve in order to spend it -- not sure what is actually is yet, need to do more research (type might be wrong)
    }

    // There is a list of these in a transaction, need to figure out what it is
    // https://en.bitcoin.it/wiki/Transaction#Witness
    struct Witness {

    }

    // Online Resource:
    // https://en.bitcoin.it/wiki/Transaction
    struct Transaction {
        uint32 version;
        bytes[] flag;
        bytes9 inputsCounter;
        Input[] inputs;
        bytes9 outputsCounter;
        Output[] outputs;
        Witness[] witnesses;
        bytes32 lockTime; 
    }

    // Online Resource:
    // https://en.bitcoin.it/wiki/Block
    struct BitcoinBlock {
        uint32 magic;
        uint32 blockSize;
        BlockHeader header;
        uint256 transactionCounter;
        Transaction[] transactions;
    }

    // handling coinbase transactions

    // not sure if we should create this 'touple' type or use maps to get the right structure
    struct AddressAmountPair {
        address rewardee;
        uint216 amount;
    }

    struct CoinbaseReward {
        // pairs array
        AddressAmountPair[] outputPairs;
        // or mapping
        mapping(address => uint216) outputMap;
        string message;
    }

    constructor() public {
        owner = msg.sender;
    }

    // Submits _account to be credited for the work of _blockHash
    function submitHash(bytes32 _blockHash, address _account) public returns (bool success) {
        commits[_blockHash] = _account;
        return true;
    }

    function _calculateDifficulty(uint32 _bits) private pure returns (uint256) {
        uint256 maxTarget = 0xFFFF * 256**(0x1D - 3);
        uint256 target = (_bits & 0xFFFFFF) * 256**(bits >> 24 - 3);
        uint256 difficulty = maxTarget / target;
        return difficulty;
    }

    function setChainTipHash(BitcoinBlock _chainTip) public onlyOwner {
        require(_chainTip.header.previousBlockHash == chainTip.header.merkleRootHash,
            "New tip previous block hash does not match current chain tip block hash");

        chainTip = _chainTip;

        // update all confirmations
        for (uint256 i = 0; i < pendingBlocks.length; i++) {
            pendingBlocksConf[pendingBlocks[i]]++;

            // if it hit 6, credit the address with the share and remove the block
            if (pendingBlocksConf[pendingBlocks[i]] >= 6) {
                // (TODO) Need some way to removing block from pending blocks list
                bytes32 pendingBlock = pendingBlocks[i];
                pendingBlocks[i] = pendingBlocks[pendingBlocks.length - 1];
                pendingBlocks.pop();

                // Otherwise we will be crediting multiple shares as long as entry is >= 6
                sharesBalances[pendingBlocksSubmitter[pendingBlock]]++;
            }
        }
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
    // 'calldata' is used to store values during function execution. read only.
    function submitBlock(BitcoinBlock calldata _block, address _account) public returns (bool success) {
        bytes32 blockHash = _block.header.merkleRootHash;
        // Address must match the one that has been committed and block hash has not been submitted to pool before
        require(
            commits[blockHash] != _account,
            "Address Doesn't Match account"
        );

        require(
            !usedBlockHashes[blockHash],
            "Block hash has already been submitted"
        );

        uint256 difficulty = _calculateDifficulty(_block.header.bits);

        // double check units match up here
        require(difficulty < DIFFICULTY_THRESHOLD, "difficulty not met");

        // check that previous block hash is the bitcoin chain tip for the fork with the most accumucated PoW (TODO)
        // chain tip is known by btc Node
        // should be passed into function
        bytes32 prevHash = _block.header.previousBlockHash;
        require(prevHash == chainTipHash, "submitted block is stale"); // chainTipHash doesn't exist we need to determine how to extract (TODO)

        // Merkle proof that Coinbase tx is pointed to current peg in address of the mining pool (TODO)

        pendingUsers.push(_account);
        pendingBlocks.push(blockHash);
        pendingBlocksConf[blockHash] = 0;
        pendingBlocksSubmitter[blockHash] = _account;

        usedBlockHashes[blockHash] = true;

        return true;
    }

    // Transfers _numShares from sender to _it
    function transfer(address _to, uint256 _numShares) public returns (bool success) {
        require(sharesBalances[msg.sender] >= _numShares, "Do not have enough shares");

        sharesBalances[msg.sender] -= _numShares;
        sharesBalances[_to] += _numShares;

        emit Transfer(msg.sender, _to, _numShares);

        return true;
    }

    // Approves _numShares to be transferred from _spender's account
    function approve(address _spender, uint256 _numShares) public returns (bool success) {
        allowance[msg.sender][_spender] = _numShares;

        emit Approval(msg.sender, _spender, _numShares);

        return true;
    }

    // Transfers _numShares from _from to _to, provided >= _numShares have been approved from the spender
    function transferFrom(address _from, address _to, uint256 _numShares) public returns (bool success) {
        require(_numShares <= sharesBalances[_from]);
        require(_numShares <= allowance[_from][msg.sender]);

        sharesBalances[_from] -= _numShares;
        sharesBalances[_to] += _numShares;

        allowance[_from][msg.sender] -= _numShares;

        emit Transfer(_from, _to, _numShares);

        return true;
    }

    // This function should be called by the oracle when rewards are won
    function distributeRewards(BitcoinBlock calldata _block) public returns (bool success) {
        // check if there has been 6+ confirmations on the block (TODO)

        Transaction calldata coinbaseTx = _block.transactions[0];
        for (uint256 i = 0; i < users.length; i++) {
            sharesBalances[users[i]] = 0;
            // distribute portion of rewards
            

            // mint some synthetic BTC, amount should be equal to the amount on the block that was submitted (TODO)
            // need to think through some way to ultimately redeem these tokens for the BTC in the peg in address (TODO)
        }

        return true;
    }
}

/*
General Design:

submitHash(address, hash)
- To prevent work from being "stolen" (man in the middle attack) we should implement a "commit-reveal" scheme in which
someone reveals a HASH(Block hash + Destination Quarry address) first and then submits the rest of the block and the
destination Quarry address to be credited with the pool share.

submitBlock(address, block)
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
