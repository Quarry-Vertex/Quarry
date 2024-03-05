pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {SPVProof} from"../../src/lib/SPVProof.sol";

contract SPVProofTest is Test {
    // declare contract object
    SPVProof public spvProof;

    // initialize contract object
    function setUp() public {
        spvProof = new SPVProof();
    }
    /*
              ROOT <- merkle root
             /    \
            AB     *CD
           /  \    /  \
        A(tx)  B* C    D
    */
    function test_ValidSPVProof() public {
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

        assertTrue(spvProof.spvProof(merklePath, root), "Valid SPV proof should pass");
    }

    function testFail_InvalidSPVProof() public {
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
        assertTrue(!spvProof.spvProof(merklePath, wrongRoot), "Invalid SPV proof should not pass");
    }

}
