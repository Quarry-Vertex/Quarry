/*
 * to get working you need to generate a key used by *bitcoin.conf*
 * this is generated from running '<bitcoin_dir>/share/rpcauth/rpcauth.py <username>'
 */
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var bitcoin = require('bitcoin-core');
var client = new bitcoin({
    network: 'regtest', // testing network
    username: "cade", // defined in ~/.bitcoin/bitcoin.conf
    password: "c",
    host: "localhost", // running on local network
    port: 18443 // regtest port
});
var BlockData = /** @class */ (function () {
    function BlockData(data) {
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
    BlockData.prototype.toArray = function () {
        return [
            this.hash, // needed
            this.confirmations, // needed
            this.height, // needed
            this.version,
            this.versionHex,
            this.merkleroot, // needed
            this.time,
            this.mediantime,
            this.nonce, // needed
            this.bits,
            this.difficulty,
            this.chainwork,
            this.nTx,
            this.previousblockhash, // needed
            this.strippedsize,
            this.size,
            this.weight,
            this.tx,
        ];
    };
    return BlockData;
}());
function test() {
    return __awaiter(this, void 0, void 0, function () {
        var bh, blockRaw, block, err_1;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    _a.trys.push([0, 3, , 4]);
                    return [4 /*yield*/, client.getBestBlockHash()];
                case 1:
                    bh = _a.sent();
                    console.log(bh);
                    return [4 /*yield*/, client.getBlock(bh)];
                case 2:
                    blockRaw = _a.sent();
                    block = new BlockData(blockRaw);
                    console.log(block);
                    console.log(block.toArray());
                    return [3 /*break*/, 4];
                case 3:
                    err_1 = _a.sent();
                    console.error(err_1);
                    return [3 /*break*/, 4];
                case 4: return [2 /*return*/];
            }
        });
    });
}
test();
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
