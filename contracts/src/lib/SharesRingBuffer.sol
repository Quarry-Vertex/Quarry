// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

// Queue to track token ids for PPLNS
abstract contract SharesRingBuffer is Initializable {
    uint256 public bufferSize;   // Maximum size of the ring buffer
    uint256 public currSize;     // Current number of elements in the buffer
    uint256 public start;        // Index of first element
    uint256 public end;          // Index of last element
    uint256[] public buffer;     // Array to store the elements

    event RingBufferPush(
        uint256 value,
        uint256 position
    );

    event RingBufferPop(
        uint256 value,
        uint256 position
    );

    function initialize(uint256 _bufferSize) public initializer {//onlyInitializing {
        bufferSize = _bufferSize;
        buffer = new uint256[](bufferSize);
        currSize = 0;
        start = 0;
        end = 0;
    }

    // Function to add an element to the buffer
    function pushToRingBuffer(uint256 value) public {
        if (currSize == bufferSize) {
            popFromRingBuffer();
        }

        buffer[end] = value;
        emit RingBufferPush(value, end);

        end = (end + 1) % bufferSize;
        currSize++;
    }

    // Function to retrieve and remove the oldest element from the buffer
    function popFromRingBuffer() public returns (uint256) {
        require(currSize > 0, "Buffer is empty");

        uint256 value = buffer[start];
        emit RingBufferPop(value, start);

        start = (start + 1) % bufferSize;
        currSize--;

        return value;
    }

    function ringBufferIsEmpty() public view returns (bool) {
        return currSize == 0;
    }

    function ringBufferIsFull() public view returns (bool) {
        return currSize == bufferSize;
    }

    function numSharesInRingBuffer() public view returns (uint256) {
        return currSize;
    }
}
