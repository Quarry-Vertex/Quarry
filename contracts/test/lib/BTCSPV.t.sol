pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {ValidateSPVScript} from "bitcoin-spv/script/ValidateSPV.s.sol";
import "forge-std/console.sol";

contract BTCSPV is Test {
    ValidateSPVScript public instance;

    function setUp() public {
        instance = new ValidateSPVScript();
    }

    function test_LibraryTests() public {

        bytes32 inputTxIdLE = hex"48e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6";
        bytes32 inputMerkleRootLE = hex"0296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c2";
        bytes memory _proof = hex"e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64";
        uint inputIndex = 281;
        assertTrue(instance.prove(inputTxIdLE, inputMerkleRootLE, _proof, inputIndex));
    }

    function test_ValidateTestnetTransaction() public {
        // construct proof paramters
        // merkle root for block transaction is part of
        bytes32 _merkleRoot = hex"3bf0614fc39756da9bd31f28a0e9568e57893a28ee2be5777f163da2e330f8c2";
        // transaction id
        bytes32 _txid = hex"6f173ff5d55a4613c6837633dbac06485be9f5a8bb22d08451c5fd9321911309";
        uint _index = 33554432;
        // proof
        // bytes memory _proof = hex"00e0ff3ffa92350f47e83163e051fceb3efdd02998d5606e0c01691cf64f05a200000000c2f830e3a23d167f77e52bee283a89578e56e9a0281fd39bda5697c34f61f03b137dff65ffff001d85dc0ab62e0200000b0913912193fdc55184d022bba8f5e95b4806acdb337683c613465ad5f53f176f5207b021fe710f488c5354f748a35db95410b4927959b4899b7b480dee0fb48a12f92593a0e12253c318995fd4998bef84b412ff24f856a8ab5108fa119e29503dd1510cb0991dc29715f3995c284d797decbeeadb64034607ec0b5f8eaf82407c33b2f2c3bf9449ac9d66b9a44c8b23fa1a738cde9db2686c87867d039f98d5fd2565a1ce3d7fb9b29445d1dfdb6f122c6d600fb0303fe65a2014b9316d8e5ea88808de176911d9b51f89e87666ee28dbcd1e599f5046d231c3f6d194ae513fd98e6be8a12a62bfd69004";
        bytes memory _proof = hex"00e0ff3ffa92350f47e83163e051fceb3efdd02998d5606e0c01691cf64f05a200000000c2f830e3a23d167f77e52bee283a89578e56e9a0281fd39bda5697c34f61f03b137dff65ffff001d85dc0ab62e0200000bfa4847b767a2a99f70d90b4ff779e0f09729a53e098e76a17d9a87d94fb152459a175af157234839a541f62545dbff7ed2e7dda993bff773718e270ad538bc5703e7c5b6ff141abc19fdea4fe67bca803d383f923ab80407d96d61315d155a1c221c70b1e4d8a2a19d3fa03c5442749bde9090259fa7b9da49557df3f9f20dae1db5067c62e63f0dd527d9bf4a69f01342b5bb246cbdef86fe3bbb775a1b1432249be97c0c03adbc6e759c0154110723403feefb966fb0f2b5f0d83bc5c1a64db859616f97418d68bcefbba895de59b6927ab6d1f8440bfc7b5ad039fa271acbba25ab121aa5925d28467028c616396652e710e0a799f02d7e570d003f732a0ff46a2674e0be7ba6c204d4901801431009c221e8c8a96f323e4c0af3b7603aa2d3294ed64cdbe78d7786268f4c5de209eb0fbe25e5a9f196cadd4d0d5b04ed20b8300a807bfaacd6512dc08065d7142cab058c9cd9c11c48a816ecd19e1c842d03abad02";
        assertTrue(instance.prove(_txid, _merkleRoot, _proof, _index));
        // index of transaction in the block
        //bool everTrue = false;
        //bool proven = false;
        //for (uint i = 0; i < 580; i++) {
        //    // try to prove transaction
        //    proven = instance.prove(_txid, _merkleRoot, _proof, i);
        //    if (proven) {
        //        everTrue = true;
        //        console.log("proven at");
        //        console.log(i);
        //    }
        //}
        //if (!everTrue) {
        //    console.log("it's so over");
        //} else {
        //    console.log("hell yea");
        //}
        // different transaction in the same block
        _txid = hex"0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba";
        assertTrue(instance.prove(_txid, _merkleRoot, _proof, _index));
    }
}
