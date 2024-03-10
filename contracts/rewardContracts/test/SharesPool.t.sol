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

    // function test_calculateDifficulty() public {

    // }

    // function test_extractScriptPubKey() public {

    // }

    // function test_scriptPubKeyToAddress() public {
        
    // }

    // function test_submitBlock() public {
        
    // }

    // function test_distributeRewards() public {

    // }
}