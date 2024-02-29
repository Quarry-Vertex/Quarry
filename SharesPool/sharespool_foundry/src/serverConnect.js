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
      previousBlockHash: result.previousblockhash,
      merkleRootHash: result.merkleroot,
    };
  } catch (error) {
    console.error('Error getting block:', error.message);
    throw error;
  }
}

const test = async () => {
  try {
    const tipBlock = await getBestBlock();
    console.log(Object.keys(tipBlock));
    console.log('Block Data:', tipBlock);
  } catch (error) {
    console.error('Test error:', error.message);
  }
}

test();

// assuming localhost will need to be known when testing
const web3 = new web3Pkg.Web3('http://localhost:7545');
const abi = require('../out/SharesPool.sol/SharesPoolAbi.json');
const address = ''; // once deployed we will get the address
const oracleAddress = ''; // need to either set or caclulate from SC when sending

const sharesPoolContract = new web3.eth.Contract(abi, address);

const setContractTip = async (contract) => {
  try {
    const chainBlock = await getBestBlock();
    const result = await contract.methods.setChainTip(chainBlock).send({
      from: oracleAddress,
    });
    console.log(`Transaction Hash => ${result}`);
  } catch (err) {
    console.error(err);
  }
}

// hash: '00000000000000095842e5e6bfcafaaa8e3e111ee8889fa4d4f37bd3a55347f6',
// confirmations: 1,
// height: 2579863,
// version: 536887296,
// versionHex: '20004000',
// merkleroot: '762b0d43016fc6d267a147d0d66194ffebfa44430c43a211d613da88dc2df374',
// time: 1709170959,
// mediantime: 1709167600,
// nonce: 3783370653,
// bits: '1927fe18',
// difficulty: 107392535.896636,
// chainwork: '000000000000000000000000000000000000000000000cfab2fba695bd7c4c3a',
// nTx: 860,
// previousblockhash: '000000000000001ac3af6dc98cdbeab1834c4c0e70a9508ea62a15bd2938f3ab',
// strippedsize: 99576,
// size: 263111,