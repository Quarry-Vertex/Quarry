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
const web3 = new web3Pkg.Web3('https://eth-sepolia.g.alchemy.com/v2/LcYLSe1fjqMF0g_p4tWPxMIFfyWJb1jK');
// get abi from forge
const abi = require('../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json');
// set recieving and sending addresses
const address = '0x6f844b60b005339703FA23c1885D85Dd4e277F60'; // once deployed we will get the address
const oracleAddress = '0xcCaCB37A575EF02C7108d23F8579732204CB4030'; // need to either set or caclulate from SC when sending
const oraclePK = '0x' + '6d2e414bff5bdc7dc6705eb5bcd858bf311106d4030599e8ede25d41927d0bf3';
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
  } catch (err) {
    console.error(`ERROR \n ${err}`);
  }
};

setContractTip(sharesPoolContract);
