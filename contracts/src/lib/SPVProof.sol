// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

abstract contract SPVProof {
    /*
        Let's say we have the following Merkle tree for four transactions (A, B, C, D):

                ROOT <- merkle root
            /    \
            AB     *CD
            /  \    /  \
        A(tx)  B* C    D

        If we want to prove that transaction A is in the Merkle tree
        the Merkle path would be the hash of B (sibling of A) and CD (sibling of AB)
        represented as an array: [B, CD].
    */
    function spvProof(bytes32[] memory merklePath, bytes32 blockHash) public pure returns (bool success) {

        bytes32 curHash = merklePath[0]; // this is wrong, need to get the tx hash
        for (uint256 i = 1; i < merklePath.length; i++) { // walk the merkle path
            // get the current hash's sibling
            bytes32 sibling = merklePath[i];
            // get the new current hash
            if (curHash < sibling) {
                curHash = sha256(abi.encodePacked(sha256(abi.encodePacked(curHash, sibling))));
            } else {
                curHash = sha256(abi.encodePacked(sha256(abi.encodePacked(sibling, curHash))));
            }
        }
        require(curHash == blockHash, "SPV proof failed");

        return true;
    }
}
