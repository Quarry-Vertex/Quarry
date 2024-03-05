pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {SharesRingBuffer} from"../../src/lib/SharesRingBuffer.sol";

contract SharesRingBufferTest is Test {
    SharesRingBuffer public ringbuf;

    function setUp() public {}

    function test_emptyBuffer() public {
        // initialize with size 0
        uint256 size = 0;
        address proxy = Upgrades.deployUUPSProxy(
          "SharesRingBuffer.sol",
          abi.encodeCall(SharesRingBuffer.initialize, (size))
        );
        ringbuf = SharesRingBuffer(proxy);
        // initialize with no open 'slots'
        // ringbuf = new SharesRingBuffer();
        // ringbuf.initialize(0);

        // 0 shares
        assertTrue(ringbuf.numSharesInRingBuffer() == 0, "Ring buffer size is 0");
        // buffer is full
        assertTrue(ringbuf.ringBufferIsFull(), "Ring buffer is full");
        // buffer is empty
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_populatedBuffer() public {
        address proxy = Upgrades.deployUUPSProxy(
          "SharesRingBuffer.sol",
          // initialize with 10 open 'slots'
          abi.encodeCall(SharesRingBuffer.initialize, (10))
        );
        ringbuf = SharesRingBuffer(proxy);

        // initialize with 10 open 'slots'
        // ringbuf = new SharesRingBuffer();
        // ringbuf.initialize(10);

        // no pushed shares
        assertTrue(ringbuf.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");

        // element to be added (1-10)
        uint16 i = 1;
        // add 10 shares
        while(!ringbuf.ringBufferIsFull()) {
            ringbuf.pushToRingBuffer(i);
            assertTrue(ringbuf.numSharesInRingBuffer() == i, "correct share count after push");
            i++; // because it is incremented post push i will be 11 at the end
        }
        assertTrue(ringbuf.numSharesInRingBuffer() == 10, "Fully populated buffer");
        assertTrue(ringbuf.ringBufferIsFull(), "buffer knows it is full");

        // pop elements from buffer
        uint256 val;
        // buffer pops from start so should start at 1
        uint256 cur = 1;
        while(!ringbuf.ringBufferIsEmpty()) {
            i--; // starts at 11
            assertTrue(ringbuf.numSharesInRingBuffer() == i, "correct share count after pop");
            val = ringbuf.popFromRingBuffer();
            assertTrue(val == cur, "correct value popped from the buffer");
            cur++; // ends at 11
        }
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_overflow() public {
        address proxy = Upgrades.deployUUPSProxy(
          "SharesRingBuffer.sol",
          // initialize with 10 open 'slots'
          abi.encodeCall(SharesRingBuffer.initialize, (10))
        );
        ringbuf = SharesRingBuffer(proxy);
        // ringbuf = new SharesRingBuffer();
        // ringbuf.initialize(10);

        // no pushed shares
        assertTrue(ringbuf.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");

        // element to be added (1-20)
        uint16 i = 1;
        // add 10 shares
        while(i <= 20) {
            ringbuf.pushToRingBuffer(i);
            i++; // ends at 21
        }
        // if the last value pushed is 20 the ringbuff should be 
        // value: 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
        // index:  0,  1,  2,  3,  4,  5,  6,  7,  8,  9 => 10 total shares
        assertTrue(i == 21, "i is at 21 so the last value pushed is 20");
        assertTrue(ringbuf.numSharesInRingBuffer() == 10, "Fully populated buffer");
        assertTrue(ringbuf.ringBufferIsFull(), "buffer knows it is full");

        // pop elements from buffer
        uint256 val;
        // buffer pops from start so should start at 11
        uint256 cur = 11;
        while(!ringbuf.ringBufferIsEmpty()) {
            val = ringbuf.popFromRingBuffer();
            assertTrue(val == cur, "correct value popped from the buffer");
            cur++; // ends at 21
        }
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function testFail_popFromEmpty() public {
        address proxy = Upgrades.deployUUPSProxy(
          "SharesRingBuffer.sol",
          // initialize with 10 open 'slots'
          abi.encodeCall(SharesRingBuffer.initialize, (10))
        );
        ringbuf = SharesRingBuffer(proxy);
        // initialize with 10 open 'slots'
        // ringbuf = new SharesRingBuffer();
        // ringbuf.initialize(10);

        // element to be added (1-10)
        uint16 i = 1;
        // add 10 shares
        while(!ringbuf.ringBufferIsFull()) {
            ringbuf.pushToRingBuffer(i);
            i++; // because it is incremented post push i will be 11 at the end
        }
        assertTrue(ringbuf.ringBufferIsFull(), "buffer knows it is full");

        while(!ringbuf.ringBufferIsEmpty()) {
            ringbuf.popFromRingBuffer();
        }
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");

        // should cause revert, cannot pop from empty buffer
        ringbuf.popFromRingBuffer();
    }
}
