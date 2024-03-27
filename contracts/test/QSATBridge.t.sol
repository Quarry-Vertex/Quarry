pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {QSAT} from "../src/QSAT.sol";
import {QSATBridge} from "../src/QSATBridge.sol";
import {Pool} from "../src/Pool.sol";

import "forge-std/console.sol";

contract QSATBridgeTest is Test {
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    bytes32 testBTCAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdfe);

    QSAT public qsat;
    QSATBridge public qsatBridge;
    Pool public pool;

    function setUp() public {
        pool = new Pool();
        pool.initialize(oracleAddress, pegInAddress, 500);

        qsatBridge = new QSATBridge();
        qsatBridge.initialize(oracleAddress, address(pool));

        qsat = new QSAT();
        qsat.initialize("Quarry", "QSAT", address(qsatBridge));

        qsatBridge.setQSATContract(address(qsat));
    }

    function test_bridgeInitialSupply() public view {
        assertEq(qsat.balanceOf(address(qsatBridge)), 21000000 * 100000000, "Expected 21000000 * 100000000 qsat to be initial supply");
    }

    function test_pegInQSAT() public {
        vm.expectEmit();
        emit QSATBridge.PegInQSATEvent(testAddress, 10000);

        vm.prank(oracleAddress);
        qsatBridge.pegInQSAT(testAddress, 10000);

        assertEq(qsat.balanceOf(testAddress), 10000, "Expected 10000 qsat to be pegged in");
        assertEq(qsat.balanceOf(address(qsatBridge)), (qsat.TOTAL_SUPPLY() - 10000), "Expected 10000 to be deducted from bridge contract");
    }

    function test_pegOutQSAT() public {
        vm.prank(oracleAddress);
        qsatBridge.pegInQSAT(testAddress, 10000);

        vm.prank(testAddress);
        qsat.approve(address(qsatBridge), 5000);

        vm.expectEmit();
        emit QSATBridge.PegOutQSATEvent(testBTCAddress, 1000);

        // test emitting events and peg out request array
        vm.prank(testAddress);
        qsatBridge.pegOutQSAT(testBTCAddress, 1000);    // 0
        vm.prank(testAddress);
        qsatBridge.pegOutQSAT(testBTCAddress, 100);     // 1
        vm.prank(testAddress);
        qsatBridge.pegOutQSAT(testBTCAddress, 300);     // 2
        vm.prank(testAddress);
        qsatBridge.pegOutQSAT(testBTCAddress, 400);     // 3

        // total peg out requests
        assertEq(qsatBridge.getTotalPegOutRequests(), 4);

        // 0
        QSATBridge.PegOutRequest memory req = qsatBridge.getPegOutRequest(0);
        assert(req.btcAddress == testBTCAddress);
        assert(req.amount == 1000);
        // 1
        req = qsatBridge.getPegOutRequest(1);
        assert(req.amount == 100);
        // 2
        req = qsatBridge.getPegOutRequest(2);
        assert(req.amount == 300);
        // 3
        req = qsatBridge.getPegOutRequest(3);
        assert(req.amount == 400);
    }
}
