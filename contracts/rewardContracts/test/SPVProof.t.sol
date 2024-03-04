pragma solidity ^0.8.13;

import {SPVProof} from"../src/lib/SPVProof.sol"};

contract SPVProofTest is Test {
    // declare contract object
    SPVProof public spvProof;

    // initialize contract object
    function setUp() public {
        spvProof = new SPVProof();
        spvProof.initialize();
    }
    /*
              ROOT <- merkle root
             /    \
            AB     *CD
           /  \    /  \
        A(tx)  B* C    D
    */
    function testValidSPVProof() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = sha256("A");
        bytes32 txB = sha256("B");
        bytes32 txCD = sha256(abi.encodePacked(sha256("C"), sha256("D")));
        merklePath[0] = txB;
        merklePath[1] = txCD;

        bytes32 root = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB)), txCD));

        assertTrue(spvProof.spvProof(merklePath, root), "Valid SPV proof should pass");
    }
}
