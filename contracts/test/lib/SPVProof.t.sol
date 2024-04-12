pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {ValidateSPVScript} from "bitcoin-spv/script/ValidateSPV.s.sol";

contract SPVProofTest is Test {
    ValidateSPVScript public instance;

    struct ProveTest {
        bytes32 inputTxIdLE;
        bytes32 inputMerkleRootLE;
        bytes inputProof;
        uint inputIndex;
        bool output;
    }

    function setUp() public {
        instance = new ValidateSPVScript();
    }
    function test_ReturnsTrueIfProofIsValid() public {
        ProveTest[1] memory testCases = [
            ProveTest({
                // tx hash in le (blockstream shows in be)
                inputTxIdLE: hex"0913912193fdc55184d022bba8f5e95b4806acdb337683c613465ad5f53f176f",
                // merkle root hash in le (blockstream shows in be)
                inputMerkleRootLE: hex"c2f830e3a23d167f77e52bee283a89578e56e9a0281fd39bda5697c34f61f03b",
                // every merkle hash in le then combined together
                inputProof: hex"5207b021fe710f488c5354f748a35db95410b4927959b4899b7b480dee0fb48a12f92593a0e12253c318995fd4998bef84b412ff24f856a8ab5108fa119e29503dd1510cb0991dc29715f3995c284d797decbeeadb64034607ec0b5f8eaf82407c33b2f2c3bf9449ac9d66b9a44c8b23fa1a738cde9db2686c87867d039f98d5fd2565a1ce3d7fb9b29445d1dfdb6f122c6d600fb0303fe65a2014b9316d8e5ea88808de176911d9b51f89e87666ee28dbcd1e599f5046d231c3f6d194ae513fd98e6be8a12a62bfd69004caa503ad25a1db0b31b31b8322abe18887dedc9f2c163c7848d19ac2cac645c8b5e6411a327a4a0a412d2cfa0a0bda5ea67809215380af6848d202dadd03d22073af21736d813f64cdb58596397559dfa8049d41bbb8300a807bfaacd6512dc08065d7142cab058c9cd9c11c48a816ecd19e1c842d",
                // index from quicknode
                inputIndex: 0,
                // expected result
                output: true
            })
        ];

        for (uint i = 0; i < testCases.length; i++) {
            bool res = instance.prove(
                testCases[i].inputTxIdLE,
                testCases[i].inputMerkleRootLE,
                testCases[i].inputProof,
                testCases[i].inputIndex
            );
            assertEq(res, testCases[i].output);
        }
    }
}
