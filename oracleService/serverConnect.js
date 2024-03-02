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
const web3 = new web3Pkg.Web3('http://localhost:8545');
// get abi from forge
const abi = require('../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json');
// set recieving and sending addresses
const address = '0x5FbDB2315678afecb367f032d93F642f64180aa3'; // once deployed we will get the address
const oracleAddress = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266'; // need to either set or caclulate from SC when sending
const oraclePK = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80';
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
      gasLimit: web3.utils.toHex(1000000),
      gasPrice: web3.utils.toHex(web3.utils.toWei('10000', 'wei')),
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
    console.log(result);
  } catch (err) {
    console.error(`ERROR \n ${err}`);
  }
};

setContractTip(sharesPoolContract);
