pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {QSAT} from"../src/QSAT.sol";
import {QSATBridge} from"../src/QSATBridge.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract QSATBridgeTest is Test {
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    bytes32 testBTCAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdfe);

    QSAT public qsat;
    QSATBridge public qsatBridge;

    address proxy;
    address proxyQSATBridge;
    address proxyQSAT;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyQSATBridge = Upgrades.deployUUPSProxy(
            "QSATBridge.sol",
            abi.encodeCall(QSATBridge.initialize, (oracleAddress, proxy))
        );

        proxyQSAT = Upgrades.deployUUPSProxy(
            "QSAT.sol",
            abi.encodeCall(QSAT.initialize, ("Quarry", "QSAT", proxyQSATBridge))
        );

        qsat = QSAT(proxyQSAT);
        qsatBridge = QSATBridge(proxyQSATBridge);

        qsatBridge.setQSATContract(proxyQSAT);
    }

    function test_bridgeInitialSupply() public {
        assertEq(qsat.balanceOf(proxyQSATBridge), 21000000 * 100000000, "Expected 21000000 * 100000000 qsat to be initial supply");
    }

    function test_pegInQSAT() public {
        vm.expectEmit();
        emit QSATBridge.PegInQSATEvent(testAddress, 10000);

        vm.prank(oracleAddress);
        qsatBridge.pegInQSAT(testAddress, 10000);

        assertEq(qsat.balanceOf(testAddress), 10000, "Expected 10000 qsat to be pegged in");
        assertEq(qsat.balanceOf(proxyQSATBridge), (qsat.TOTAL_SUPPLY() - 10000), "Expected 10000 to be deducted from bridge contract");
    }

    function test_pegOutQSAT() public {
        vm.prank(proxy);
        qsatBridge.pegInQSAT(testAddress, 10000);

        vm.prank(testAddress);
        qsat.approve(proxyQSATBridge, 5000);

        vm.expectEmit();
        emit QSATBridge.PegOutQSATEvent(testBTCAddress, 5000);

        vm.prank(testAddress);
        qsatBridge.pegOutQSAT(testBTCAddress, 5000);
    }
}
