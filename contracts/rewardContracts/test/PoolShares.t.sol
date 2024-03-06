pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {PoolShares} from"../src/PoolShares.sol";

contract PoolSharesTest is Test {
    address testAddress = address(bytes20(bytes32(uint256(keccak256(abi.encodePacked("0x5FbDB2315678afecb367f032d93F642f64180aa3"))))));

    PoolShares public poolShares;
    address proxy;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "PoolShares.sol",
            abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", ""))
        );
        poolShares = PoolShares(proxy);
    }

    function test_mintPoolShares() public {
        poolShares.awardShare(testAddress, 1);
        assertTrue(poolShares.getOwnerOfShare(1) == testAddress, "Expected testAddress to be owner of tokenId 1");
    }

    // function test_burnPoolShares() public {
    //     uint256 tokenId = poolShares.awardShare(testAddress, 2);
    //     assertTrue(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 2");

    //     poolShares.burnShare(tokenId);
    //     assertFalse(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected tokenId 2 to have been burned");
    // }

    // function test_transferPoolShares() public {
    //     uint256 tokenId = poolShares.awardShare(testAddress, 3);
    //     assertTrue(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected testAddress to be owner of tokenId 3");

    //     poolShares.approve(testAddress, tokenId);
    //     poolShares.transferFrom(testAddress, address(123), tokenId);
    //     assertFalse(poolShares.getOwnerOfShare(tokenId) == testAddress, "Expected tokenId 3 to have been transferred out of testAddress");
    //     assertTrue(poolShares.getOwnerOfShare(tokenId) == address(123), "Expected tokenId 3 to have been transferred to address 123");
    // }
}
