pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {QuarryBTC} from"../src/QuarryBTC.sol";

contract QuarryBTCTest is Test {
    address testAddress = address(bytes20(bytes32(uint256(keccak256(abi.encodePacked("0x5FbDB2315678afecb367f032d93F642f64180aa3"))))));

    QuarryBTC public quarryBTC;
    address proxy;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "QuarryBTC.sol",
            abi.encodeCall(QuarryBTC.initialize, ("Quarry", "QBTC"))
        );
        quarryBTC = QuarryBTC(proxy);
    }

    function test_mintQuarryBTC() public {
        quarryBTC.mintQuarryBTC(testAddress, 100);
        assertTrue(quarryBTC.getBalanceOf(testAddress) == 100, "Expected 100 QBTC to be minted");

    }

    // function test_burnQuarryBTC() public {
    //     quarryBTC.mintQuarryBTC(address(quarryBTC), 100);
    //     assertTrue(quarryBTC.getBalanceOf(address(quarryBTC)) == 100, "Expected 100 QBTC to be minted");

    //     quarryBTC.burnQuarryBTC(50);
    //     assertTrue(quarryBTC.getBalanceOf(address(quarryBTC)) == 50, "Expected 50 QBTC to be burned, leaving 50 left");
    // }
}
