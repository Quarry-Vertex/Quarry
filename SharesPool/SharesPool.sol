pragma solidity >=0.6.0 <0.9.0;

// contract SharesToken {
//     string public name;
//     mapping(address => uint256) public balances;

//     constructor(string memory _name) public {
//         name = _name;
//     }

//     function mint() public {
//         balances[tx.origin]++;
//     }

//     function burn() public {

//     }
// }

// contract SharesPool is SharesToken {
//     string public symbol;
//     address[] public owners;
//     uint256 public ownerCount;

//     constructor(
//         string memory _name,
//         string memory _symbol
//     )
//         ERC20Token(_name)
//     public {
//         symbol = _symbol;
//     }

//     function mint() public {
//         super.mint();
//         ownerCount ++;
//         owners.push(msg.sender);
//     }

// }

contract SharesPool {
    mapping(address => uint256) public balances; // tracks number of shares per user
    address[] public users; // corresponding array to balances mapping, used to clear balances

    mapping(bytes32 => address) public commits; // tracks the address that has committed a block hash

    mapping(address => mapping(address => uint256)) public allowance; // tracks approvals for transfering shares

    uint256 public totalSupply;

    event Transfer(
        address indexed _from,
        address indexed _to,
        uint256 _value
    );

    event Approval(
        address indexed _owner,
        address indexed _spender,
        uint256 _value
    );

    uint32 constant magic = 0xD9B4BEF9;

    struct BlockHeader {
        uint32 version;
        bytes32 previousBlockHash;
        bytes32 merkleRoot;
        uint32 timestamp;
        uint32 bits;
        uint32 nonce;
    }

    struct Transaction {

    }

    struct BitcoinBlock {
        uint32 magic;
        uint32 blockSize;
        BlockHeader header;
        uint256 transactionCounter;
        Transaction[] transactions;
    }

    // Submits _account to be credited for the work of _blockHash
    function submitHash(bytes32 _blockHash, address _account) public returns (bool success) {
        commits[_blockHash] = _account;
    }

    /*

    */
    function submitBlock(bytes32 _blockHash, address _account) public returns (bool success) {
        // Validate the block hash

        balances[_account]++;
    }

    // Transfers _numShares from sender to _it
    function transfer(address _to, uint256 _numShares) public returns (bool success) {
        require(balances[msg.sender] >= _numShares);

        balances[msg.sender] -= _numShares;
        balances[_to] += _numShares;

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
        require(_numShares <= balances[_from]);
        require(_numShares <= allowance[_from][msg.sender]);

        balances[_from] -= _numShares;
        balances[_to] += _numShares;

        allowance[_from][msg.sender] -= _numShares;

        emit Transfer(_from, _to, _numShares);

        return true;
    }

    function distributeRewards(BitcoinBlock _block) public returns (bool success) {
        // check if there has been 6+ confirmations on the block
        for (uint256 i = 0; i < users.length; i++) {
            uint256 shares = balances[users[i]];
            balances[users[i]] = 0;
            // distribute portion of rewards
            // mint some synthetic BTC, amount should be equal to the amount on the block that was submitted
        }
    }
}

/*
submitHash(address, hash)
- To prevent work from being "stolen" (man in the middle attack) we should implement a "commit-reveal" scheme in which someone reveals a HASH(Block hash + Destination Quarry address) first and then submits the rest of the block and the destination Quarry address to be credited with the pool share.
- Needs to store 

submitBlock(address, block)
- Keep track of which addresses have how many shares (mapping of address to number of shares)
- Translator proxy has to communicate to this (but other participants can as well)
- Must reject stale shares
- Checks should be:
    * Address must match the one that has been committed
    * A merkle proof (ie SPV proof) that the Coinbase transaction of the block is pointed to the current peg in address
    * The block header of the submitted block meets the pool difficulty (which will be fixed - for now let's just make it represent 5 Exahash of work - which means assuming the whole network is mining on Quarry there should be 100 new pool shares per second, since Bitcoin's hashrate is 500 Eh/s)
    * The previous block hash (written in the current block's block header) is the Bitcoin chain tip for the fork with the most accumulated PoW
    * The block hash has not been submitted to the pool before (thus all submitted block hashes should be kept in a hashmap/ set data structure on chain - this hashmap can be cleared when a new Bitcoin block is found and the chain tip is updated)

transferShares(addressToTransferTo, numShares)

distributeRewards(bitcoin block)
- Clears out all shares and distributes rewards prorata to addresses (make sure to delete shares before distributing
- Probably called from the smart contract that was verifying work (if it gets a block that meets the difficulty of Bitcoin, it should broadcast the block while alerting the shares tracker smart contract, after 6 confirmations)


The work verifier smart contract only has to check for SPV proofs, but it should have access to the whole block that was hashed. Because if the block is actually a winning Bitcoin block, we need to be able to broadcast that to the Bitcoin network.
*/