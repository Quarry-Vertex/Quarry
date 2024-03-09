pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {QuarryBTC} from"../src/QuarryBTC.sol";

import "forge-std/console.sol";

contract QuarryBTCTest is Test {
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));

    QuarryBTC public quarryBTC;
    address proxy;

    function setUp() public {
        // proxy = Upgrades.deployUUPSProxy(
        //     "QuarryBTC.sol",
        //     abi.encodeCall(QuarryBTC.initialize, ("Quarry", "QBTC"))
        // );
        quarryBTC = new QuarryBTC("Quarry", "QBTC");
    }

    function test_mintQuarryBTC() public {
        quarryBTC.mintQuarryBTC(testAddress, 100);
        assertTrue(quarryBTC.getBalanceOf(testAddress) == 100, "Expected 100 QBTC to be minted");

    }

    function test_burnQuarryBTC() public {
        quarryBTC.mintQuarryBTC(address(this), 100);
        assertTrue(quarryBTC.getBalanceOf(address(this)) == 100, "Expected 100 QBTC to be minted");

        vm.prank(address(this));
        quarryBTC.burnQuarryBTC(50);
        assertTrue(quarryBTC.getBalanceOf(address(this)) == 50, "Expected 50 QBTC to be burned, leaving 50 left");
    }

    function test_transferQuarryBTC() public {
        quarryBTC.mintQuarryBTC(address(this), 100);
        assertTrue(quarryBTC.getBalanceOf(address(this)) == 100, "Expected 100 QBTC to be minted");

        vm.prank(address(this));
        quarryBTC.approve(address(this), 50);

        quarryBTC.transferFrom(address(this), testAddress, 50);
        assertTrue(quarryBTC.getBalanceOf(address(this)) == 50, "Expected 50 QBTC to be transferred, leaving 50 left");
        assertTrue(quarryBTC.getBalanceOf(testAddress) == 50, "Expected 50 QBTC to be transferred to testAddress");
    }

    function test_burnQuarryBTCInsufficientBalance() public {
        quarryBTC.mintQuarryBTC(address(this), 100);
        assertTrue(quarryBTC.getBalanceOf(address(this)) == 100, "Expected 100 QBTC to be minted");

        vm.prank(address(this));
        vm.expectRevert("Insufficient balance in address");
        quarryBTC.burnQuarryBTC(150);
    }
}
