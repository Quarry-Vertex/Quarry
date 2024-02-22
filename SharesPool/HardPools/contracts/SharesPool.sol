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

    mapping(address => uint256) public syntheticBTCBalances; // balance of each user of credited synthetic BTC

    uint256 totalShares; // number of total shares, used to calculate how many rewards to distribute
    mapping(address => uint256) public sharesBalances; // tracks number of shares per user
    address[] users; // users with shares

    mapping(bytes32 => uint8) public confirmations; // tracks number of confirmations for each block hash
    bytes32[] blocks; // list of blocks from setChainTip

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

    function setChainTip(BitcoinBlock memory _chainTip) public onlyOwner {
        require(_chainTip.header.previousBlockHash == chainTip.header.merkleRootHash,
            "New tip previous block hash does not match current chain tip block hash");

        chainTip = _chainTip;

        // increment number of confirmations for each block
        for (uint256 i = 0; i < blocks.len; i++) {
            confirmations[blocks[i]]++;
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
    function submitBlockHeader(BitcoinHeader memory _blockHeader, bytes32[] memory _merklePath, address _account) public returns (bool success) {
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

        // Merkle proof that Coinbase tx is pointed to current peg in address of the mining pool (TODO)

        // All checks pass, credit user with share
        if (sharesBalances[_account] == 0) {
            users.push_back(_account);
        }

        sharesBalances[_account]++;
        totalShares++;

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
    // TODO: Make sure this function is only callable by the oracle contract or would this be from the mining pool??
    // TODO: We need to add queueing logic that Allard explained using PPLNS
    function distributeRewards(BitcoinBlock calldata _block) public returns (bool success) {
        // TODO: Verify that the block is a valid, won block and should probably double check that coinbase transaction points to peg in address
        // I think this involves translating the script field of the output of the coinbase transaction in some way to get the peg in address

        require(confirmations[_block.header.merkleRootHash] < 6, "Does not have 6+ confirmations");

        uint256 numShares = totalShares;
        totalShares = 0;
        uint8 blockReward = _block.transactions[0].outputs.value;
        for (uint256 i = 0; i < users.length; i++) {
            uint256 userShares = sharesBalances[users[i]];
            sharesBalances[users[i]] = 0;
            
            // distribute portion of rewards
            syntheticBTCBalances[users[i]] += (userShares / numShares) * blockReward;
        }

        // clear all pending share state
        users = new address[](0);

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
