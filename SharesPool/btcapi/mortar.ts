// interop with Bitcoin Block Chain
const bitcoin = require('bitcoin-core');
// interop with Smart Contract
const Web3 = require('web3');

// sample block header class to capture data from 'bitcoin-core' funcs that return lates block
// need to define a class as the lib is in pure js
/*
    const curBlockHash = await client.getBlockHash('latest');
    const curBlockHeader: BlockHeader = await client.getBlockHeader(curBlockHash);
*/
// class BlockHeader {
//     version: number;
//     previousBlockHash: string;
//     merkleRoot: string;
//     timestamp: number;
//     bits: number;
//     nonce: number;

//     constructor(version: number, previousBlockHash: string, merkleRoot: string, timestamp: number, bits: number, nonce: number) {
//         this.version = version;
//         this.previousBlockHash = previousBlockHash;
//         this.merkleRoot = merkleRoot;
//         this.timestamp = timestamp;
//         this.bits = bits;
//         this.nonce = nonce;
//     }
// }

// Bitcoin node interactions
const client = new bitcoin({ network: 'regtest'})

async function getLongestChainHash(): Promise<string> {
    try {
        const longestChainHash: string = await client.getBestBlockHash();
        return longestChainHash;
    } catch (err) {
        console.error(`rError => ${err}`);
        return err
    }
}
//--------------------------

// Smart Contract interactions
const fs = require('fs');

// connect to ETH node
const node_address: string = ''; // needs to be filled in
const web3 = new Web3(node_address);

// constant files / addresses
const abi_file: string = ''; // needs to be filled in
const contractAddress: string = ''; // needs to be filled in
const senderAddress: string = ''; // needs to be filled in

const contractABI = JSON.parse(fs.readFileSync(abi_file, 'utf8'));
const contract = new web3.eth.Contract(contractABI, contractAddress);
//----------------------------

async function smartContractInterop () {
    try {
        const longestChainHash: string = await getLongestChainHash();
        // pass chain into smart contract
        contract.methods.checkHashTip(longestChainHash)
            .send({ from: senderAddress })
            .on('receipt', (rec) => {
                // do something
                console.log(rec);
            })
            .on('error', (err) => {
                console.error(`Error with CheckHashTip ${err}`);
            });

    } catch (err) {
        console.error(`Error interacting with Smart Contract => ${err}`);
        return err
    }
}
