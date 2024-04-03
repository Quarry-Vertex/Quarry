pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {ValidateSPVScript} from "bitcoin-spv/script/ValidateSPV.s.sol";
import {BTCUtilsScript} from "bitcoin-spv/script/BTCUtils.s.sol";
import "forge-std/console.sol";

contract BTCSPV is Test {
    /*
    ValidateSPVScript public instance;
    BTCUtilsScript public btcUtils;

    function setUp() public {
        instance = new ValidateSPVScript();
        btcUtils = new BTCUtilsScript();
    }

    function test_LibraryTests() public {

        bytes32 inputTxIdLE = hex"48e5a1a0e616d8fd92b4ef228c424e0c816799a256c6a90892195ccfc53300d6";
        bytes32 inputMerkleRootLE = hex"0296ef123ea96da5cf695f22bf7d94be87d49db1ad7ac371ac43c4da4161c8c2";
        bytes memory _proof = hex"e35a0d6de94b656694589964a252957e4673a9fb1d2f8b4a92e3f0a7bb654fddb94e5a1e6d7f7f499fd1be5dd30a73bf5584bf137da5fdd77cc21aeb95b9e35788894be019284bd4fbed6dd6118ac2cb6d26bc4be4e423f55a3a48f2874d8d02a65d9c87d07de21d4dfe7b0a9f4a23cc9a58373e9e6931fefdb5afade5df54c91104048df1ee999240617984e18b6f931e2373673d0195b8c6987d7ff7650d5ce53bcec46e13ab4f2da1146a7fc621ee672f62bc22742486392d75e55e67b09960c3386a0b49e75f1723d6ab28ac9a2028a0c72866e2111d79d4817b88e17c821937847768d92837bae3832bb8e5a4ab4434b97e00a6c10182f211f592409068d6f5652400d9a3d1cc150a7fb692e874cc42d76bdafc842f2fe0f835a7c24d2d60c109b187d64571efbaa8047be85821f8e67e0e85f2f5894bc63d00c2ed9d64";
        uint inputIndex = 281;
        assertTrue(instance.prove(inputTxIdLE, inputMerkleRootLE, _proof, inputIndex));
    }

    function extractMerkleProofFromHex(bytes memory hexProof) public pure returns (bytes memory) {
        // Assuming the first byte indicates the number of hashes
        uint256 numHashes = uint8(hexProof[0]);

        // Extract the hashes
        bytes memory proof = new bytes(numHashes * 32);
        for (uint256 i = 0; i < numHashes; i++) {
            for (uint256 j = 0; j < 32; j++) {
                proof[i * 32 + j] = hexProof[1 + i * 32 + j];
            }
        }

        return proof;
    }


    function test_ValidateTestnetTransaction() public {
        // construct proof paramters
        bytes32 _merkleRoot = hex"3bf0614fc39756da9bd31f28a0e9568e57893a28ee2be5777f163da2e330f8c2";
        bytes32 _txid = hex"0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba";
        bytes memory _proof = (hex"8b03a008b7afca6d15d20c08567d41c2ba50c8c99d1cc1848a61ce1de9c148d2af84747b762a9af9079db0f47f970e0f79925ae390e8671ad7a9789df41b2554a971a51f753284935a146f5254bdffe72d7edd9a39fb7f3717e872a05d83cb75307e5c6bff41a1cb91dfaef46eb7ac08d383f329a38b40709dd61613d551a5c13d92e46dc4bd7ed8776862f8c4d52e90bef0eb525e9a1f69acddd4d0b540de0222c1071b4e8d2a1ad9f30ac3452447b9ed090952f97a9bad9455d73f9f2fd0ea4fa662470eebb76a2c404d0981103401902c128e8c9af623e3c4a03f7b06a32ad15b60c7266ef3d05d729dfba4960f31245bbb42c6dbfe68efb3bb77a5b1412342b99ec7c030dacbe657c9104511703204f3eebf69f60b2f5b0f8db35c1c6ad48b9516f67914d886cbfebb8a59ed956b29a76b1d8f44b0cfb7a50d93af72a1bc");

        // try every index
        bool everTrue = false;
        bool proven = false;
        for (uint i = 0; i < 580; i++) {
            // try to prove transaction
            proven = instance.prove(_txid, _merkleRoot, _proof, i);
            if (proven) {
                everTrue = true;
                console.log("proven at");
                console.log(i);
            }
        }

        // see if it's ever passing
        if (!everTrue) {
            console.log("it's so over");
        } else {
            console.log("hell yea");
        }
        // different transaction in the same block
        //_txid = hex"0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba";
        //assertTrue(instance.prove(_txid, _merkleRoot, _proof, _index));
    }
    */
}
