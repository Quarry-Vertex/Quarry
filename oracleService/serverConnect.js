const axios = require('axios');
const web3Pkg = require('web3');

// set the QuickNode endpoint
const endpoint = 'https://yolo-holy-sheet.btc-testnet.quiknode.pro/920ff3a045338aaeebaec4a4ea5ee278ee4e737e/';

// get the best block hash
const getBestHash = async () => {
  try {
    // connect via axios
    const chainRes = await axios.post(endpoint, {
      method: 'getbestblockhash'
    }, {
      headers: {
        'Content-Type': 'application/json'
      }
    });
    // get the hash
    const tipHash = chainRes.data.result;
    console.log('Best Block Hash:', tipHash);
    return tipHash;
  } catch (error) {
    console.error('Error getting tip hash:', error.message);
    throw error;
  }
}

// get the best block
const getBestBlock = async () => {
  try {
    const tipHash = await getBestHash();
    // connect via axios
    const blockRes = await axios.post(endpoint, {
      method: 'getblock',
      params: [tipHash] // set tip hash
    }, {
      headers: {
        'Content-Type': 'application/json'
      }
    });
    const result = blockRes.data.result;
    return {
      previousBlockHash: '0x' + result.previousblockhash,
      merkleRootHash: '0x' + result.merkleroot,
    };
  } catch (error) {
    console.error('Error getting block:', error.message);
    throw error;
  }
}

// const test = async () => {
//   try {
//     const tipBlock = await getBestBlock();
//     console.log(Object.keys(tipBlock));
//     console.log('Block Data:', tipBlock);
//   } catch (error) {
//     console.error('Test error:', error.message);
//   }
// }

// test();

// assuming localhost will need to be known when testing (using infura goerli for testing)
const web3 = new web3Pkg.Web3('https://goerli.infura.io/v3/531a76cd2d144d118c734b2bed4e3150');
// get abi from forge
const abi = require('../SharesPool/sharespool_foundry/out/SharesPool.sol/SharesPoolAbi.json');
// set recieving and sending addresses
const address = '0x2EcD1F8A8c1b4Ab15d6075010b57D68cc6cCe9bA'; // once deployed we will get the address
const oracleAddress = '0xc9d9d042b7BB36d95457395B61FaC29D724b4E35'; // need to either set or caclulate from SC when sending
const oraclePK = '0x' + 'bad28dc83de39b5f51a002986efd5a5222e3428e396e43261ae423659dacfc7e';
// instantiate sharespool contract
const sharesPoolContract = new web3.eth.Contract(abi, address);

// interact with the smart contract
const setContractTip = async (contract) => {
  try {
    // get the chainTip
    const chainBlock = await getBestBlock();
    // interact with oracle account
    const nonce = await web3.eth.getTransactionCount(oracleAddress);
    // prepare transaction
    const txData = {
      nonce: nonce,
      gasLimit: web3.utils.toHex(22005),
      gasPrice: web3.utils.toHex(web3.utils.toWei('10', 'gwei')),
      to: address,
      data: contract.methods.setChainTip(chainBlock).encodeABI()
    }
    // get the balance of the oracle (eth needed for gas)
    const balance = await web3.eth.getBalance(oracleAddress);
    console.log(`Balance: ${web3.utils.fromWei(balance, 'ether')} ETH`);

    // sign the transaction
    const signedTx = await web3.eth.accounts.signTransaction(txData, oraclePK);
    console.log('---Transaction Successfully Signed---')
    console.log('Transaction Signing Data:', signedTx.rawTransaction);
    // try sending the transaction
    const result = await web3.eth.sendSignedTransaction(signedTx.rawTransaction);
    // get successful transaction hash
    console.log(`Transaction Hash => ${result.transactionHash}`);
  } catch (err) {
    console.error(`ERROR \n ${err}`);
  }
};

setContractTip(sharesPoolContract);
