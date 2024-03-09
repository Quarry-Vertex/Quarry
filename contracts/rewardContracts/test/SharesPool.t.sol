pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {SharesPool} from"../src/SharesPool.sol";

contract SharesPoolTest is Test {
    SharesPool public sharesPool;
    address public proxy;

    function setUp() public {
        address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // replace with actual oracle address
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress))
        );
        sharesPool = SharesPool(proxy);
    }

    function test_initialChainTip() public {
        // Test the initial chain tip values
        SharesPool.ChainTip memory chainTip = sharesPool.getChainTip();
        assertEq(chainTip.previousBlockHash, bytes32(0), "Initial previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32(0), "Initial merkle root hash should be 0");
    }

    // Add more test functions as needed
}
