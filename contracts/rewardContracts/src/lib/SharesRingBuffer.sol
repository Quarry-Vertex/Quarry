pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

// Queue to track token ids for PPLNS
contract SharesRingBuffer is Initializable {
    uint256 public bufferSize;   // Maximum size of the ring buffer
    uint256 public currSize;     // Current number of elements in the buffer
    uint256 public index;        // Index of the oldest element
    uint256[] public buffer;     // Array to store the elements

    event RingBufferPush(
        uint256 value,
        uint256 position
    );

    event RingBufferPop(
        uint256 value,
        uint256 position
    );

    function initialize(uint256 _bufferSize) public onlyInitializing {
        bufferSize = _bufferSize;
        buffer = new uint256[](bufferSize);
        currSize = 0;
        index = 0;
    }

    // Function to add an element to the buffer
    function pushToRingBuffer(uint256 value) public {
        if (currSize >= bufferSize) { // wraps around
            index = 0;
        } else { // only increment size if buffer isn't already full
            currSize++;
        }

        buffer[index] = value;

        emit RingBufferPush(value, index);

        index++;
    }

    // Function to retrieve and remove the oldest element from the buffer
    function popFromRingBuffer() public returns (uint256) {
        require(currSize > 0, "Buffer is empty");

        uint256 value = buffer[index];

        emit RingBufferPush(value, index);

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
