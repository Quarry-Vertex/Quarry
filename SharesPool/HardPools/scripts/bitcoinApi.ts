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
  toArray(): any[] {
    return [
      this.hash,              // 0 needed
      this.confirmations,     // 1 needed
      this.height,            // 2 needed
      this.version,
      this.versionHex,
      this.merkleroot,        // 5 needed
      this.time,
      this.mediantime,
      this.nonce,             // 8 needed
      this.bits,
      this.difficulty,
      this.chainwork,
      this.nTx,
      this.previousblockhash, // 13 needed
      this.strippedsize,
      this.size,
      this.weight,
      this.tx,
    ];
  }
}

async function testChainTip() {
  try {
    const bh = await client.getBestBlockHash();
    console.log(bh);
    const blockRaw = await client.getBlock(bh);
    const block: BlockData = new BlockData(blockRaw)
    console.log(block);
    console.log(block.toArray());
  } catch (err) {
    console.error(err);
  }
}

testChainTip()

/* ~/.bitcoin/bitcoin.conf
regtest=1
rpcuser=cade
rpcpassword=c
# bottom part might not be needed
rpcauth=cade:cf8840c50066552acc04526de1c67ed7$c10878119565f1a0ea5032e2838c66f3015a8ae6f1e1f2ab61ece7f1b3e52c50
*/
