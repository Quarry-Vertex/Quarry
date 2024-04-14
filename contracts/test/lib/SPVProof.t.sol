pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {ValidateSPVScript} from "bitcoin-spv/script/ValidateSPV.s.sol";

/*
    Testing the library function ValidateSPVScript.prove() function
    Although the library has its own tests, there is no documentation. This made
    it almost impossible to get working with data from quicknode and other block
    explorers. Finally I was able to find the format of the hashes and proof.

        TxIdLE: You can find this by reversing the bytes of the transaction that
        you would find on a block explorer / from a local node. This represents
        the transaction that you are trying to validate.

        MerkleRootLE: You can find this by reversing the merkle root of the 
        block a transaction is from. This can be found with a block explorer or
        with quicknode / a local node.

        Proof: The proof is the combination of the hashes in the merkle path.
        Not the proof string that you would get out of a local node. Note that each
        hash must be revered from BE to LE before being combined.
 */
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
        ProveTest[3] memory testCases = [
            // testnet
            ProveTest({
                // tx hash in le (blockstream shows in be)
                inputTxIdLE: hex"59b6781bde30207aef00cc0745e80a635e24be3c3c17a3a013c1082ac7506a80",
                // merkle root hash in le (blockstream shows in be)
                inputMerkleRootLE: hex"9c7864c96ba028f6fea5e24be811615822a37b3de6241fec56bedbb19ba4838c",
                // every merkle hash in le then combined together
                inputProof: hex"5d6c736e20b9011945431ed5caae0563df3d0897f503cad7f5ea0a6e5d28828c8f040861d599ef6708e7e997c54ebcc655925c99a8003789687eed48f3ee56a2a0952fdf53dc6adc5aedf89ed3b21042414188a001ebf94abf5b80056a2a0dc76d609c2a29a30a458f144ddd49c530610c9f12ca8db105a7a988aaa92fff3ee71c52b32ae1d9af9ebdb3913d727e713fb2ba06c010cc2d2f3d63e563f17c685207240ce5d67518954c51d8e5ed008cd1784ec5b4770847f40c3647e83e1469a9bdc8d7349de09f89c3e59f298051378159dc46a49024bcfbb31fe522828e980ec8173b006500920dad06592d8574757821466377fa89420ac02736419655def3b5455d77b1ed25bf39eb2ceb811adfac86cce86e1bc06c0ff69d2e993cfb6a9bde0b3ec82bc8425dafc2d0db9a3c1c25e951b615b68ecea804d77d6ee8dd6fcb7e7e04b4adf838dfa17f5a4e6a95584c73b01b05ff8a68c20c6fd67b05e5df530b3e29a41f38830ff77dcbcb299bda98fa33e8c42bac7ab86160774f4724f677de9de7a31f25cc972dcaaf6818f598530fddbcbcda62d9afa11f4c5c2c24decd",
                // index from blockstream
                inputIndex: 6,
                // expected result
                output: true
            }),
            // testnet
            ProveTest({
                inputTxIdLE: hex"0913912193fdc55184d022bba8f5e95b4806acdb337683c613465ad5f53f176f",
                inputMerkleRootLE: hex"c2f830e3a23d167f77e52bee283a89578e56e9a0281fd39bda5697c34f61f03b",
                inputProof: hex"5207b021fe710f488c5354f748a35db95410b4927959b4899b7b480dee0fb48a12f92593a0e12253c318995fd4998bef84b412ff24f856a8ab5108fa119e29503dd1510cb0991dc29715f3995c284d797decbeeadb64034607ec0b5f8eaf82407c33b2f2c3bf9449ac9d66b9a44c8b23fa1a738cde9db2686c87867d039f98d5fd2565a1ce3d7fb9b29445d1dfdb6f122c6d600fb0303fe65a2014b9316d8e5ea88808de176911d9b51f89e87666ee28dbcd1e599f5046d231c3f6d194ae513fd98e6be8a12a62bfd69004caa503ad25a1db0b31b31b8322abe18887dedc9f2c163c7848d19ac2cac645c8b5e6411a327a4a0a412d2cfa0a0bda5ea67809215380af6848d202dadd03d22073af21736d813f64cdb58596397559dfa8049d41bbb8300a807bfaacd6512dc08065d7142cab058c9cd9c11c48a816ecd19e1c842d",
                inputIndex: 0,
                output: true
            }),
            // mainnet
            ProveTest({
                inputTxIdLE: hex"33fc61288fb55f28a328ca41544bd1ce3055ada53290de01899d7b6a17d1a5a8",
                inputMerkleRootLE: hex"98ba972861edf0da0f711fda06781571acec965462fd4f9a33830c63eec4a5d4",
                inputProof: hex"33e520709bec4b480e6ed27b8527d853b0b665323770e17ef74d11e80bc509bf5f52899ba08794b9caa5ef1f5a114e1b78d685b456fecd9d9978599e87e37214dceadbcf492be11fbe8f93bceee3bc74cc0da43704b81afb061a2b18794300c5d7c7c76e734701c002e41e65106c18a4c72ecf54bd9b8f30cc5c1d260ac8ed44e5a4311a0a30d56e579b9fd03dd6f60d64f2890956822d176a1da44ca38d726775692fa442ddb1c8444e9b56d75732771cf4c96b49453f5b8f948890225b4742193ff686bdc081fddd7c745dabdf07c81b6671403469c9e7c2290b961407f468faf60775deb2f3c811dbcd43ffe533737f2aaac5ac651fc5547e1da69807eb71453b47cf3d564a1820966ffc4b02332777846122248c23d7d7e2df307fe0aaf96e766b101cdea45f624c0feeffd25a5b957faab3c8f2df3290f6708fd0e388d2519279127431229a9d629bd2072bd9c4ec9807f8211e9cb0ece2519614ad09ce81c5001db330f6ff85ca70e23bed90d08f549cddf46f1b9ea985006e51662c0d",
                inputIndex: 1668,
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
