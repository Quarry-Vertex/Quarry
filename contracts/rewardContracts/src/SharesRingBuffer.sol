pragma solidity ^0.8.0;

// Queue to track token ids for PPLNS
contract SharesRingBuffer {
    uint256 public bufferSize;   // Maximum size of the ring buffer
    uint256 public currSize;     // Current number of elements in the buffer
    uint256 public index;        // Index of the oldest element
    uint256[] public buffer;     // Array to store the elements

    // Constructor to initialize the ring buffer
    constructor(uint256 _bufferSize) {
        bufferSize = _bufferSize;
        buffer = new uint256[](bufferSize);
        currSize = 0;
        index = 0;
    }

    // Function to add an element to the buffer
    function pushToRingBuffer(uint256 value) public {
        if (currSize < bufferSize) { // wraps around
            index = 0;
        } else { // only increment size if buffer isn't already full
            currSize++;
        }

        buffer[index] = value;
        index++;
    }

    // Function to retrieve and remove the oldest element from the buffer
    function popFromRingBuffer() public returns (uint256) {
        require(currSize > 0, "Buffer is empty");

        uint256 value = buffer[index];
        currSize--;
        if (index == 0) { // wraps around
            index = bufferSize - 1;
        } else {
            index--;
        }

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
