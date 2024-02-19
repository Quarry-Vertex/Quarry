/*
 * to get working you need to generate a key used by *bitcoin.conf*
 * this is generated from running '<bitcoin_dir>/share/rpcauth/rpcauth.py <username>'
 */

const bitcoin = require('bitcoin-core');

const client = new bitcoin({
  network: 'regtest',     // testing network
  username: "cade",       // defined in ~/.bitcoin/bitcoin.conf
  password: "c",
  host: "localhost",      // running on local network
  port: 18443             // regtest port
})

class BlockData {
  hash: string;
  confirmations: number;
  height: number;
  version: number;
  versionHex: string;
  merkleroot: string;
  time: number;
  mediantime: number;
  nonce: number;
  bits: string;
  difficulty: string;
  chainwork: string;
  nTx: number;
  previousblockhash: string;
  strippedsize: number;
  size: number;
  weight: number;
  tx: string[];

  constructor(data: any) {
    this.hash = data.hash;
    this.confirmations = data.confirmations;
    this.height = data.height;
    this.version = data.version;
    this.versionHex = data.versionHex;
    this.merkleroot = data.merkleroot;
    this.time = data.time;
    this.mediantime = data.mediantime;
    this.nonce = data.nonce;
    this.bits = data.bits;
    this.difficulty = data.difficulty;
    this.chainwork = data.chainwork;
    this.nTx = data.nTx;
    this.previousblockhash = data.previousblockhash;
    this.strippedsize = data.strippedsize;
    this.size = data.size;
    this.weight = data.weight;
    this.tx = data.tx;
  }
}

async function test() {
  try {
    const bh = await client.getBestBlockHash();
    console.log(bh);
    const blockRaw = await client.getBlock(bh);
    const block: BlockData = new BlockData(blockRaw)
    console.log(block);
  } catch (err) {
    console.error(err);
  }
}

test()

/* ~/.bitcoin/bitcoin.conf
regtest=1
rpcuser=cade
rpcpassword=c
# bottom part might not be needed
rpcauth=cade:cf8840c50066552acc04526de1c67ed7$c10878119565f1a0ea5032e2838c66f3015a8ae6f1e1f2ab61ece7f1b3e52c50
*/

// structure of bitcoin block
/*
[Object: null prototype] {
  hash: '52f2891d524907be0cb9432cf11c65a012e3c723758d5fa9882465144e32dd78',
  confirmations: 1,
  height: 20,
  version: 536870912,
  versionHex: '20000000',
  merkleroot: '12894cc447e1aecf295319daf19d1013c8e0b46bb8375f8932273eb57f384025'
,
  time: 1708293047,
  mediantime: 1708293046,
  nonce: 1,
  bits: '207fffff',
  difficulty: '4.656542373906925e-10',
  chainwork: '000000000000000000000000000000000000000000000000000000000000002a',
  nTx: 1,
  previousblockhash: '51bbdd192552df24e95dde13cac5cb665f4035a776b561ee3906ba7f96
6cfbf5',
  strippedsize: 213,
  size: 249,
  weight: 888,
  tx: [
    '12894cc447e1aecf295319daf19d1013c8e0b46bb8375f8932273eb57f384025'
  ]
}
*/
