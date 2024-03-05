pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {SharesRingBuffer} from"../src/lib/SharesRingBuffer.sol";

contract SharesRingBufferTest is Test {
    SharesRingBuffer public ringbuf;

    function setUp() public {
        address proxy = Upgrades.deployUUPSProxy(
            "SharesRingBuffer.sol",
            abi.encodeCall(SharesRingBuffer.initialize, (0))
        );
        ringbuf = SharesRingBuffer(proxy);
        // ringbuf = new SharesRingBuffer();
        // ringbuf.initialize(0);
    }

    function test_emptybuf() public {
        assertTrue(ringbuf.numSharesInRingBuffer() == 0, "Ring buffer size is 0");
        // assertTrue(true, "Ring buffer is full");
        assertTrue(ringbuf.ringBufferIsEmpty(), "Ring buffer is empty");
    }
}
