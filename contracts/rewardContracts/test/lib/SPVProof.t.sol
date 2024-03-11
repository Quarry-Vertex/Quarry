pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {SharesPool} from"../../src/SharesPool.sol";
import "forge-std/console.sol";

contract SPVProofTest is Test {
    /*
        Visualization of tree used for testing
              ROOT <- merkle root
             /    \
            AB     *CD
           /  \    /  \
        A(tx)  B* C    D
    */

    // initialize contract object
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    SharesPool public sharesPool;
    address public proxy;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress))
        );
        sharesPool = SharesPool(proxy);
    }

    /*
        Test getting the correct merkle root from every node in the merkle tree
    */

    // Starting at Transaction A
    function test_FromAPasses() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;
        merklePath[2] = txCD;

        // get the root
        bytes32 root = sha256(abi.encodePacked(sha256(abi.encodePacked(txAB, txCD))));

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");
    }

    // Starting at Transaction B
    function test_FromBPasses() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txB; // curhash (hash of the transaction)
        merklePath[1] = txA;
        merklePath[2] = txCD;

        // get the root
        bytes32 root = sha256(abi.encodePacked(sha256(abi.encodePacked(txAB, txCD))));

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");
    }

    // Starting at Transaction C
    function test_FromCPasses() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txC; // curhash (hash of the transaction)
        merklePath[1] = txD;
        merklePath[2] = txAB;

        // get the root
        bytes32 root = sha256(abi.encodePacked(sha256(abi.encodePacked(txAB, txCD))));

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");
    }

    // Starting at Transaction D
    function test_FromDPasses() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txD; // curhash (hash of the transaction)
        merklePath[1] = txC;
        merklePath[2] = txAB;

        // get the root
        bytes32 root = sha256(abi.encodePacked(sha256(abi.encodePacked(txAB, txCD))));

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");
    }

    /*
        Failing proofs due to incorrect roots
    */

    function test_FromAFails() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;
        merklePath[2] = txCD;

        // get the root (order matters)
        bytes32 wrongRoot = sha256(abi.encodePacked(sha256(abi.encodePacked(txCD, txAB))));
        vm.expectRevert("SPV proof failed");
        sharesPool.spvProof(merklePath, wrongRoot);
    }

    function test_FromCFails() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](3);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txC; // curhash (hash of the transaction)
        merklePath[1] = txD;
        merklePath[2] = txAB;

        // get the root (order matters)
        bytes32 wrongRoot = sha256(abi.encodePacked(sha256(abi.encodePacked(txCD, txAB))));
        vm.expectRevert("SPV proof failed");
        sharesPool.spvProof(merklePath, wrongRoot);
    }

}
