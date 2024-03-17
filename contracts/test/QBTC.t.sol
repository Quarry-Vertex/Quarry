pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {QSAT} from"../src/QSAT.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract qsatTest is Test {
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);

    QSAT public qsat;
    address proxy;
    address proxyqsat;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyqsat = Upgrades.deployUUPSProxy(
            "QSAT.sol",
            abi.encodeCall(qsat.initialize, ("Quarry", "QSAT", proxy))
        );

        qsat = QSAT(proxyqsat);
    }

    function test_mint() public {
        vm.prank(proxy);
        qsat.mint(testAddress, 100);
        assertTrue(qsat.balanceOf(testAddress) == 100, "Expected 100 qsat to be minted");
    }

    function test_burn() public {
        vm.startPrank(proxy);
        qsat.mint(proxy, 100);
        assertTrue(qsat.balanceOf(proxy) == 100, "Expected 100 qsat to be minted");

        qsat.burn(50);
        assertTrue(qsat.balanceOf(proxy) == 50, "Expected 50 qsat to be burned, leaving 50 left");
        vm.stopPrank();
    }

    function test_transferqsat() public {
        vm.startPrank(proxy);
        qsat.mint(proxy, 100);
        assertTrue(qsat.balanceOf(proxy) == 100, "Expected 100 qsat to be minted");

        qsat.approve(proxy, 50);

        qsat.transferFrom(proxy, testAddress, 50);
        assertTrue(qsat.balanceOf(proxy) == 50, "Expected 50 qsat to be transferred, leaving 50 left");
        assertTrue(qsat.balanceOf(testAddress) == 50, "Expected 50 qsat to be transferred to testAddress");
        vm.stopPrank();
    }

    function test_burnInsufficientBalance() public {
        vm.prank(proxy);
        qsat.mint(proxy, 100);
        assertTrue(qsat.balanceOf(proxy) == 100, "Expected 100 qsat to be minted");

        vm.prank(proxy);
        vm.expectRevert("Insufficient balance in address");
        qsat.burn(150);
    }
}
