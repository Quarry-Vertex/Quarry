pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Share} from"../src/Share.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract ShareTest is Test {
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address testAddress2 = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));

    Share public share;
    address proxy;
    address proxyShare;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(Share.initialize, ("QuarryShares", "QShare", proxy))
        );

        share = Share(proxyShare);
    }

    function test_mintShare() public {
        vm.startPrank(proxy);
        share.mint(testAddress, 1);
        assertTrue(share.ownerOf(1) == testAddress, "Expected testAddress to be owner of tokenId 1");
        vm.stopPrank();
    }

    function test_burn() public {
        vm.startPrank(proxy);
        uint256 tokenId = share.mint(testAddress, 2);
        assertTrue(share.ownerOf(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 2");

        share.burn(tokenId);
        assertFalse(share.tokenExists(tokenId), "Expected tokenId 2 to have been burned");
        vm.stopPrank();
    }

    function test_transferShare() public {
        vm.startPrank(proxy);
        uint256 tokenId = share.mint(testAddress, 3);
        assertTrue(share.ownerOf(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 3");
        vm.stopPrank();

        vm.startPrank(testAddress);
        share.approve(testAddress, tokenId);
        share.transferFrom(testAddress, testAddress2, tokenId);
        vm.stopPrank();

        assertFalse(share.ownerOf(tokenId) == testAddress, "Expected tokenId 3 to have been transferred out of testAddress");
        assertTrue(share.ownerOf(tokenId) == testAddress2, "Expected tokenId 3 to have been transferred to testAddress2");
    }

    function test_burnDoesNotExist() public {
        vm.startPrank(proxy);
        uint256 tokenId = share.mint(testAddress, 4);
        assertTrue(share.ownerOf(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 4");

        vm.expectRevert("Token does not exist");
        share.burn(5);
    }

}
