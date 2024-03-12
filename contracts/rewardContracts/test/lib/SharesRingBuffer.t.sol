pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {SharesPool} from"../../src/SharesPool.sol";
import {PoolShares} from"../../src/PoolShares.sol";

contract SharesRingBufferTest is Test {
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    SharesPool public sharesPool;
    address public proxy;
    address public proxyPoolShares;

    function setUp() public {}

    function test_emptyBuffer() public {
        // initialize with size 0
        uint256 size = 0;
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, size))
        );
        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", "", proxy))
        );
        sharesPool = SharesPool(proxy);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        // 0 shares
        assertTrue(sharesPool.numSharesInRingBuffer() == 0, "Ring buffer size is 0");
        // buffer is full
        assertTrue(sharesPool.ringBufferIsFull(), "Ring buffer is full");
        // buffer is empty
        assertTrue(sharesPool.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_populatedBuffer() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 10))
        );
        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", "", proxy))
        );
        sharesPool = SharesPool(proxy);
        sharesPool.setPoolSharesContract(proxyPoolShares);

        // no pushed shares
        assertTrue(sharesPool.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(sharesPool.ringBufferIsEmpty(), "Ring buffer is empty");

        // element to be added (1-10)
        uint16 i = 1;
        // add 10 shares
        while(!sharesPool.ringBufferIsFull()) {
            sharesPool.pushToRingBuffer(i);
            assertTrue(sharesPool.numSharesInRingBuffer() == i, "correct share count after push");
            i++; // because it is incremented post push i will be 11 at the end
        }
        assertTrue(sharesPool.numSharesInRingBuffer() == 10, "Fully populated buffer");
        assertTrue(sharesPool.ringBufferIsFull(), "buffer knows it is full");

        // pop elements from buffer
        uint256 val;
        // buffer pops from start so should start at 1
        uint256 cur = 1;
        while(!sharesPool.ringBufferIsEmpty()) {
            i--; // starts at 11
            assertTrue(sharesPool.numSharesInRingBuffer() == i, "correct share count after pop");
            val = sharesPool.popFromRingBuffer();
            assertTrue(val == cur, "correct value popped from the buffer");
            cur++; // ends at 11
        }
        assertTrue(sharesPool.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_overflow() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 10))
        );
        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", "", proxy))
        );
        sharesPool = SharesPool(proxy);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        // no pushed shares
        assertTrue(sharesPool.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(sharesPool.ringBufferIsEmpty(), "Ring buffer is empty");

        // element to be added (1-20)
        uint16 i = 1;
        // add 10 shares
        while(i <= 20) {
            sharesPool.pushToRingBuffer(i);
            i++; // ends at 21
        }
        // if the last value pushed is 20 the ringbuff should be
        // value: 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
        // index:  0,  1,  2,  3,  4,  5,  6,  7,  8,  9 => 10 total shares
        assertTrue(i == 21, "i is at 21 so the last value pushed is 20");
        assertTrue(sharesPool.numSharesInRingBuffer() == 10, "Fully populated buffer");
        assertTrue(sharesPool.ringBufferIsFull(), "buffer knows it is full");

        // pop elements from buffer
        uint256 val;
        // buffer pops from start so should start at 11
        uint256 cur = 11;
        while(!sharesPool.ringBufferIsEmpty()) {
            val = sharesPool.popFromRingBuffer();
            assertTrue(val == cur, "correct value popped from the buffer");
            cur++; // ends at 21
        }
        assertTrue(sharesPool.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_popFromEmpty() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 10))
        );
        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", "", proxy))
        );
        sharesPool = SharesPool(proxy);
        sharesPool.setPoolSharesContract(proxyPoolShares);

        // element to be added (1-10)
        uint16 i = 1;
        // add 10 shares
        while(!sharesPool.ringBufferIsFull()) {
            sharesPool.pushToRingBuffer(i);
            i++; // because it is incremented post push i will be 11 at the end
        }
        assertTrue(sharesPool.ringBufferIsFull(), "buffer knows it is full");

        while(!sharesPool.ringBufferIsEmpty()) {
            sharesPool.popFromRingBuffer();
        }
        assertTrue(sharesPool.ringBufferIsEmpty(), "Ring buffer is empty");

        vm.expectRevert("Buffer is empty");
        sharesPool.popFromRingBuffer();
    }
}
