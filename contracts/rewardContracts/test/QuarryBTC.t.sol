pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {QuarryBTC} from"../src/QuarryBTC.sol";
import {SharesPool} from"../src/SharesPool.sol";

import "forge-std/console.sol";

contract QuarryBTCTest is Test {
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);

    QuarryBTC public quarryBTC;
    address proxy;
    address proxyQuarryBTC;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyQuarryBTC = Upgrades.deployUUPSProxy(
            "QuarryBTC.sol",
            abi.encodeCall(QuarryBTC.initialize, ("Quarry", "QBTC", proxy))
        );

        quarryBTC = QuarryBTC(proxyQuarryBTC);
    }

    function test_mintQuarryBTC() public {
        vm.prank(proxy);
        quarryBTC.mintQuarryBTC(testAddress, 100);
        assertTrue(quarryBTC.getBalanceOf(testAddress) == 100, "Expected 100 QBTC to be minted");
    }

    function test_burnQuarryBTC() public {
        vm.startPrank(proxy);
        quarryBTC.mintQuarryBTC(proxy, 100);
        assertTrue(quarryBTC.getBalanceOf(proxy) == 100, "Expected 100 QBTC to be minted");

        quarryBTC.burnQuarryBTC(50);
        assertTrue(quarryBTC.getBalanceOf(proxy) == 50, "Expected 50 QBTC to be burned, leaving 50 left");
        vm.stopPrank();
    }

    function test_transferQuarryBTC() public {
        vm.startPrank(proxy);
        quarryBTC.mintQuarryBTC(proxy, 100);
        assertTrue(quarryBTC.getBalanceOf(proxy) == 100, "Expected 100 QBTC to be minted");

        quarryBTC.approve(proxy, 50);

        quarryBTC.transferFrom(proxy, testAddress, 50);
        assertTrue(quarryBTC.getBalanceOf(proxy) == 50, "Expected 50 QBTC to be transferred, leaving 50 left");
        assertTrue(quarryBTC.getBalanceOf(testAddress) == 50, "Expected 50 QBTC to be transferred to testAddress");
        vm.stopPrank();
    }

    function test_burnQuarryBTCInsufficientBalance() public {
        vm.prank(proxy);
        quarryBTC.mintQuarryBTC(proxy, 100);
        assertTrue(quarryBTC.getBalanceOf(proxy) == 100, "Expected 100 QBTC to be minted");

        vm.prank(proxy);
        vm.expectRevert("Insufficient balance in address");
        quarryBTC.burnQuarryBTC(150);
    }
}
