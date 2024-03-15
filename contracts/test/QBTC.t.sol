pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {QBTC} from"../src/QBTC.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract qbtcTest is Test {
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);

    QBTC public qbtc;
    address proxy;
    address proxyqbtc;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyqbtc = Upgrades.deployUUPSProxy(
            "QBTC.sol",
            abi.encodeCall(qbtc.initialize, ("Quarry", "QBTC", proxy))
        );

        qbtc = QBTC(proxyqbtc);
    }

    function test_mint() public {
        vm.prank(proxy);
        qbtc.mint(testAddress, 100);
        assertTrue(qbtc.balanceOf(testAddress) == 100, "Expected 100 qbtc to be minted");
    }

    function test_burn() public {
        vm.startPrank(proxy);
        qbtc.mint(proxy, 100);
        assertTrue(qbtc.balanceOf(proxy) == 100, "Expected 100 qbtc to be minted");

        qbtc.burn(50);
        assertTrue(qbtc.balanceOf(proxy) == 50, "Expected 50 qbtc to be burned, leaving 50 left");
        vm.stopPrank();
    }

    function test_transferqbtc() public {
        vm.startPrank(proxy);
        qbtc.mint(proxy, 100);
        assertTrue(qbtc.balanceOf(proxy) == 100, "Expected 100 qbtc to be minted");

        qbtc.approve(proxy, 50);

        qbtc.transferFrom(proxy, testAddress, 50);
        assertTrue(qbtc.balanceOf(proxy) == 50, "Expected 50 qbtc to be transferred, leaving 50 left");
        assertTrue(qbtc.balanceOf(testAddress) == 50, "Expected 50 qbtc to be transferred to testAddress");
        vm.stopPrank();
    }

    function test_burnInsufficientBalance() public {
        vm.prank(proxy);
        qbtc.mint(proxy, 100);
        assertTrue(qbtc.balanceOf(proxy) == 100, "Expected 100 qbtc to be minted");

        vm.prank(proxy);
        vm.expectRevert("Insufficient balance in address");
        qbtc.burn(150);
    }
}
