pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {PoolShares} from"../src/PoolShares.sol";
import {QuarryBTC} from"../src/QuarryBTC.sol";
import {SharesPool} from"../src/SharesPool.sol";

import "forge-std/console.sol";

contract SharesPoolTest is Test {
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    SharesPool public sharesPool;
    PoolShares public poolShares;
    QuarryBTC public quarryBTC;

    address public proxy;
    address public proxyPoolShares;
    address public proxyQuarryBTC;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQuarryBTC = Upgrades.deployUUPSProxy(
          "QuarryBTC.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryBTC", "QBTC", proxy))
        );

        sharesPool = SharesPool(proxy);
        poolShares = PoolShares(proxyPoolShares);
        quarryBTC = QuarryBTC(proxyQuarryBTC);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        sharesPool.setQuarryBTCContract(proxyQuarryBTC);
    }

    function test_initialChainTip() public {
        vm.prank(oracleAddress);
        SharesPool.ChainTip memory chainTip = sharesPool.getChainTip();
        assertEq(chainTip.previousBlockHash, bytes32(0), "Initial previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32(0), "Initial merkle root hash should be 0");
    }

    function test_setChainTip() public {
        vm.startPrank(oracleAddress);
        SharesPool.ChainTip memory newTip;
        newTip.previousBlockHash = 0;
        newTip.merkleRootHash = "A";
        sharesPool.setChainTip(newTip);

        SharesPool.ChainTip memory chainTip = sharesPool.getChainTip();
        vm.stopPrank();

        assertEq(chainTip.previousBlockHash, bytes32(0), "Previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32("A"), "Merkle root hash should be A");
    }

    function test_submitHash() public {
        sharesPool.submitHash(bytes32(testHash), testAddress);
        assertEq(sharesPool.getAddressForSubmittedHash(bytes32(testHash)), testAddress);
    }

    function test_calculateDifficulty() public {
        uint32 bits = 0x1b0404cb;
        uint256 difficulty = sharesPool._calculateDifficulty(bits);
        uint256 expectedDifficulty = 163074209349632;
        assertEq(difficulty, expectedDifficulty);
    }

    function test_submitBlock() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // get the root
        bytes32 root = txAB;

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");

        // set initial chain tip
        SharesPool.ChainTip memory initialTip;
        initialTip.previousBlockHash = 0;
        initialTip.merkleRootHash = "C";
        sharesPool.setChainTip(initialTip);
        // create a new chain tip
        SharesPool.ChainTip memory newTip;
        newTip.previousBlockHash = "C";
        newTip.merkleRootHash = "D";
        sharesPool.setChainTip(newTip);

        vm.stopPrank();

        // create a block header
        SharesPool.BlockHeader memory blockHeader;
        blockHeader.version = 10001;
        blockHeader.previousBlockHash = "D";
        blockHeader.merkleRootHash = root;
        blockHeader.timestamp = 10001;
        blockHeader.bits = 0x1b0404cb;
        blockHeader.nonce = 10001;

        // create a block
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));

        SharesPool.BitcoinBlock memory curBlock;
        curBlock.header = blockHeader;
        curBlock.outputAddress = outputAddress;
        curBlock.blockReward = blockReward;

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        assertTrue(sharesPool.submitBlock(curBlock, merklePath, account), "block successfully submitted");
        assertEq(poolShares.getOwnerOfShare(0), account, "Owner of share with token id 0");
    }

    // function test_distributeRewards() public {

    // }
}
