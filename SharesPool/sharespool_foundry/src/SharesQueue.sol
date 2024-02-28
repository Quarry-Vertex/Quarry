pragma solidity ^0.8.0;

import "./PoolShares.sol";

// Queue to track token ids for PPLNS
contract SharesQueue {
    uint256 private capacity;
    uint256 private front; // Front pointer
    uint256 private rear;  // Rear pointer
    uint256[] private queue; // contains tokenids, corresponding to the NFTs

    // Constructor to initialize the queue
    constructor(uint256 _capacity) {
        capacity = _capacity;
        front = 0;
        rear = 0;
        queue = new uint256[](capacity);
    }

    // Enqueue: Add an element to the rear of the queue
    function enqueue(uint256 value) public {
        require(rear < capacity, "Queue is full");
        queue[rear] = value;
        rear++;
    }

    // Dequeue: Remove and return the element from the front of the queue
    function dequeue() public returns (uint256) {
        require(front < rear, "Queue is empty");
        uint256 value = queue[front];
        delete queue[front];
        front++;
        return value;
    }

    // Get the front element without removing it
    function peek() public view returns (uint256) {
        require(front < rear, "Queue is empty");
        return queue[front];
    }

    // Check if the queue is empty
    function isEmpty() public view returns (bool) {
        return front >= rear;
    }

    // Check if the queue is full
    function isFull() public view returns (bool) {
        return rear >= capacity;
    }

    // Get the current size of the queue
    function size() public view returns (uint256) {
        return rear - front;
    }
}
