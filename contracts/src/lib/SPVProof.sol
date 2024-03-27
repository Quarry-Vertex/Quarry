// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "forge-std/console.sol";

abstract contract SPVProof {
    function bytes32ToString(bytes32 _bytes32) public returns (string memory) {
        bytes memory bytesArray = new bytes(32);
        for (uint256 i; i < 32; i++) {
            bytesArray[i] = _bytes32[i];
        }
        return string(bytesArray);
    }

    function reverseBytes32(bytes32 input) public pure returns (bytes32) {
        bytes32 reversed;
        for (uint i = 0; i < 32; i++) {
            reversed |= (input[i] & 0xFF) << (31 - i) * 8;
        }
        return reversed;
    }
    /*
    function doubleSha256(bytes memory data) public pure returns (bytes32) {
        return sha256(abi.encodePacked(sha256(data)));
    }

    function spvProof1(bytes32[] memory proof, bytes32 merkleRootHash, bytes32 txHash, uint256 index) public returns (bool success) {
        bytes32 hash = txHash;

        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofHash = proof[i];

            if (index % 2 == 0) {
                hash = doubleSha256(abi.encodePacked(hash, proofHash));
            } else {
                hash = doubleSha256(abi.encodePacked(proofHash, hash));
            }
            console.log(bytes32ToString(hash));
            index = index / 2;
        }

        return hash == merkleRootHash;
    }
    */

    function doubleSha256(bytes memory data) public pure returns (bytes32) {
        return sha256(abi.encodePacked(sha256(data)));
    }

    function spvProof1(bytes32[] memory proof, bytes32 merkleRootHash, bytes32 txHash, uint256 index) public returns (bool success) {
        bytes32 hash = (txHash);

        for (uint256 i = 0; i < proof.length; i++) {
            bytes32 proofHash = proof[i];

            if (index % 2 == 0) {
                hash = (doubleSha256(abi.encodePacked(hash, proofHash)));
            } else {
                hash = (doubleSha256(abi.encodePacked(proofHash, hash)));
            }
            index = index >> 1;
        }

        return hash == merkleRootHash;
    }



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
    function spvProof(bytes32[] memory merklePath, bytes32 merkleRootHash) public pure returns (bool success) {

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
        require(curHash == merkleRootHash, "SPV proof failed");

        return true;
    }
}
