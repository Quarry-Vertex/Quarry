pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Pool} from"../../src/Pool.sol";
import {Share} from"../../src/Share.sol";

import "forge-std/console.sol";

contract SharesRingBufferTest is Test {
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp)))); // random address for testing
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    Pool public pool;
    Share public share;

    function setUp() public {
        pool = new Pool();
        share = new Share();
    }

    function test_emptyBuffer() public {
        pool.initialize(oracleAddress, pegInAddress, 0);
        share.initialize("QuarryShares", "QShare", address(pool));

        pool.setShareContract(address(share));
        // 0 shares
        assertTrue(pool.numSharesInRingBuffer() == 0, "Ring buffer size is 0");
        // buffer is full
        assertTrue(pool.ringBufferIsFull(), "Ring buffer is full");
        // buffer is empty
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_populatedBuffer() public {
        pool.initialize(oracleAddress, pegInAddress, 10);
        share.initialize("QuarryShares", "QShare", address(pool));

        pool.setShareContract(address(share));
        // no pushed shares
        assertTrue(pool.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");

        // element to be added (1-10)
        uint16 i = 1;
        // add 10 shares
        while(!pool.ringBufferIsFull()) {
            pool.pushToRingBuffer(i);
            assertTrue(pool.numSharesInRingBuffer() == i, "correct share count after push");
            i++; // because it is incremented post push i will be 11 at the end
        }
        assertTrue(pool.numSharesInRingBuffer() == 10, "Fully populated buffer");
        assertTrue(pool.ringBufferIsFull(), "buffer knows it is full");

        // pop elements from buffer
        uint256 val;
        // buffer pops from start so should start at 1
        uint256 cur = 1;
        while(!pool.ringBufferIsEmpty()) {
            i--; // starts at 11
            assertTrue(pool.numSharesInRingBuffer() == i, "correct share count after pop");
            val = pool.popFromRingBuffer();
            assertTrue(val == cur, "correct value popped from the buffer");
            cur++; // ends at 11
        }
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_overflow() public {
        pool.initialize(oracleAddress, pegInAddress, 10);
        share.initialize("QuarryShares", "QShare", address(pool));

        pool.setShareContract(address(share));
        // no pushed shares
        assertTrue(pool.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");

        // element to be added (1-20)
        uint16 i = 1;
        // add 10 shares
        while(i <= 20) {
            pool.pushToRingBuffer(i);
            i++; // ends at 21
        }
        // if the last value pushed is 20 the ringbuff should be
        // value: 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
        // index:  0,  1,  2,  3,  4,  5,  6,  7,  8,  9 => 10 total shares
        assertTrue(i == 21, "i is at 21 so the last value pushed is 20");
        assertTrue(pool.numSharesInRingBuffer() == 10, "Fully populated buffer");
        assertTrue(pool.ringBufferIsFull(), "buffer knows it is full");

        // pop elements from buffer
        uint256 val;
        // buffer pops from start so should start at 11
        uint256 cur = 11;
        while(!pool.ringBufferIsEmpty()) {
            val = pool.popFromRingBuffer();
            assertTrue(val == cur, "correct value popped from the buffer");
            cur++; // ends at 21
        }
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");
    }

    function test_popFromEmpty() public {
        pool.initialize(oracleAddress, pegInAddress, 10);
        share.initialize("QuarryShares", "QShare", address(pool));

        pool.setShareContract(address(share));
        // element to be added (1-10)
        uint16 i = 1;
        // add 10 shares
        while(!pool.ringBufferIsFull()) {
            pool.pushToRingBuffer(i);
            i++; // because it is incremented post push i will be 11 at the end
        }
        assertTrue(pool.ringBufferIsFull(), "buffer knows it is full");

        while(!pool.ringBufferIsEmpty()) {
            pool.popFromRingBuffer();
        }
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");

        vm.expectRevert("Buffer is empty");
        pool.popFromRingBuffer();
    }

    function test_partiallyPopulatedBuffer() public {
        pool.initialize(oracleAddress, pegInAddress, 10);
        share.initialize("QuarryShares", "QShare", address(pool));

        pool.setShareContract(address(share));
        // no pushed shares
        assertTrue(pool.numSharesInRingBuffer() == 0, "Ring buffer size starts at 0");
        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");

        // add 5 shares
        for(uint64 i = 0; i < 5; i++) {
            pool.pushToRingBuffer(i);
        }
        assertTrue(pool.numSharesInRingBuffer() == 5, "5 elements added");

        // pop all added shares
        while(!pool.ringBufferIsEmpty()) {
            pool.popFromRingBuffer();
        }

        assertTrue(pool.ringBufferIsEmpty(), "Ring buffer is empty");
    }
}
