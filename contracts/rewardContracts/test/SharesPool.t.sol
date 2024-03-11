pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {SharesPool} from"../src/SharesPool.sol";

import "forge-std/console.sol";

contract SharesPoolTest is Test {
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    SharesPool public sharesPool;
    address public proxy;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress))
        );
        sharesPool = SharesPool(proxy);
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

    /*
    function test_calculateDifficulty() public {
        vm.startPrank(oracleAddress);
        uint32 bits = 0x1d00ffff;
        uint256 calcDifficulty = sharesPool._calculateDifficulty(bits);
        vm.stopPrank();
        // different expected values
        // hex
        uint256 expected = 0x15a35c0000000000000000000000000000000000000000;
        // decimal
        uint256 expectedDecimal = 2072520395859657486634608572838975759381606196813234176;
        assertEq(calcDifficulty, expected);
    }

    function test_extractScriptPubKey() public {

    }

    function test_scriptPubKeyToAddress() public {

    }

    function test_distributeRewards() public {

    }
    */

    function test_submitBlock() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;
        merklePath[2] = txCD;

        // get the root
        bytes32 root = sha256(abi.encodePacked(sha256(abi.encodePacked(txAB, txCD))));

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");

        // // set chaing tip
        // SharesPool.ChainTip memory newTip;
        // newTip.previousBlockHash = txAB;
        // newTip.merkleRootHash = root;
        // sharesPool.setChainTip(newTip);
        
        // // create a block header
        // SharesPool.BlockHeader memory blockHeader;
        // blockHeader.version = 10001;
        // blockHeader.previousBlockHash = txAB;
        // blockHeader.merkleRootHash = root;
        // blockHeader.timestamp = 10001;
        // blockHeader.bits = 10001;
        // blockHeader.nonce = 10001;

        // // create a block
        // bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        // sharesPool.setPegInAddress(outputAddress);
        // bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));

        // SharesPool.BitcoinBlock memory curBlock;
        // curBlock.header = blockHeader;
        // curBlock.outputAddress = outputAddress;
        // curBlock.blockReward = blockReward;

        // // create address
        // address account = address(bytes20(keccak256(abi.encode(block.timestamp))));

        // assertTrue(sharesPool.submitBlock(curBlock, merklePath, account), "block successfully submitted");

        vm.stopPrank();
    }

}
