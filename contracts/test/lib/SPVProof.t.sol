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
        ProveTest[2] memory testCases = [
            ProveTest({
                // tx hash in le (blockstream shows in be)
                inputTxIdLE: hex"59b6781bde30207aef00cc0745e80a635e24be3c3c17a3a013c1082ac7506a80",
                // merkle root hash in le (blockstream shows in be)
                inputMerkleRootLE: hex"9c7864c96ba028f6fea5e24be811615822a37b3de6241fec56bedbb19ba4838c",
                // every merkle hash in le then combined together
                inputProof: hex"5d6c736e20b9011945431ed5caae0563df3d0897f503cad7f5ea0a6e5d28828c8f040861d599ef6708e7e997c54ebcc655925c99a8003789687eed48f3ee56a2a0952fdf53dc6adc5aedf89ed3b21042414188a001ebf94abf5b80056a2a0dc76d609c2a29a30a458f144ddd49c530610c9f12ca8db105a7a988aaa92fff3ee71c52b32ae1d9af9ebdb3913d727e713fb2ba06c010cc2d2f3d63e563f17c685207240ce5d67518954c51d8e5ed008cd1784ec5b4770847f40c3647e83e1469a9bdc8d7349de09f89c3e59f298051378159dc46a49024bcfbb31fe522828e980ec8173b006500920dad06592d8574757821466377fa89420ac02736419655def3b5455d77b1ed25bf39eb2ceb811adfac86cce86e1bc06c0ff69d2e993cfb6a9bde0b3ec82bc8425dafc2d0db9a3c1c25e951b615b68ecea804d77d6ee8dd6fcb7e7e04b4adf838dfa17f5a4e6a95584c73b01b05ff8a68c20c6fd67b05e5df530b3e29a41f38830ff77dcbcb299bda98fa33e8c42bac7ab86160774f4724f677de9de7a31f25cc972dcaaf6818f598530fddbcbcda62d9afa11f4c5c2c24decd",
                // index from quicknode
                inputIndex: 6,
                // expected result
                output: true
            }),
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
