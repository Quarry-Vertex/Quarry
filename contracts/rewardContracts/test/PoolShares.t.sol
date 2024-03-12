pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {PoolShares} from"../src/PoolShares.sol";
import {SharesPool} from"../src/SharesPool.sol";

import "forge-std/console.sol";

contract PoolSharesTest is Test {
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address testAddress2 = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));

    PoolShares public poolShares;
    address proxy;
    address proxyPoolShares;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", proxy))
        );

        poolShares = PoolShares(proxyPoolShares);
    }

    function test_mintPoolShares() public {
        vm.startPrank(proxy);
        poolShares.awardShare(testAddress, 1);
        assertTrue(poolShares.getOwnerOfShare(1) == testAddress, "Expected testAddress to be owner of tokenId 1");
        vm.stopPrank();
    }

    function test_burnPoolShares() public {
        vm.startPrank(proxy);
        uint256 tokenId = poolShares.awardShare(testAddress, 2);
        assertTrue(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 2");

        poolShares.burnShare(tokenId);
        assertFalse(poolShares.tokenExists(tokenId), "Expected tokenId 2 to have been burned");
        vm.stopPrank();
    }

    function test_transferPoolShares() public {
        vm.startPrank(proxy);
        uint256 tokenId = poolShares.awardShare(testAddress, 3);
        assertTrue(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 3");
        vm.stopPrank();

        vm.startPrank(testAddress);
        poolShares.approve(testAddress, tokenId);
        poolShares.transferFrom(testAddress, testAddress2, tokenId);
        vm.stopPrank();

        assertFalse(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected tokenId 3 to have been transferred out of testAddress");
        assertTrue(poolShares.getOwnerOfShare(tokenId) == testAddress2, "Expected tokenId 3 to have been transferred to testAddress2");
    }

    function test_burnShareDoesNotExist() public {
        vm.startPrank(proxy);
        uint256 tokenId = poolShares.awardShare(testAddress, 4);
        assertTrue(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 4");

        vm.expectRevert("Token does not exist");
        poolShares.burnShare(5);
    }

}
