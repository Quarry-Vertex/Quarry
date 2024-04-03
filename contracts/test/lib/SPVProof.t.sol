pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Pool} from"../../src/Pool.sol";
import {Share} from"../../src/Share.sol";
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

    /*
    // initialize contract object
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 200))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    Pool public pool;
    address public proxy;
    address public proxyShare;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 500))
        );
        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(Share.initialize, ("QuarryShares", "QShare", proxy))
        );
        pool = Pool(proxy);
        pool.setShareContract(proxyShare);
    }

    // Test getting the correct merkle root from every node in the merkle tree

    function reverseBytes32(bytes32 input) public pure returns (bytes32) {
        bytes32 reversed;
        for (uint i = 0; i < 32; i++) {
            reversed |= (input[i] & 0xFF) << (31 - i) * 8;
        }
        return reversed;
    }
    // Starting at Transaction A
    function test_FromAPasses() public {
        // Example of a valid Merkle path for transaction A in the Merkle tree
        // bytes32[] memory merklePath = new bytes32[](3);
        bytes32[] memory merklePath = new bytes32[](10);

        // create transaction hashes in merkle path (all reversed)
        merklePath[0] = reverseBytes32(hex"cb1a27fa39d05a7bfc0b44f8d1b67a92b659de95a8bbefbc688d41976f6159b8");
        merklePath[1] = reverseBytes32(hex"4da6c1c53bd8f0b5f2b06f96fbee3f4023071154019c756ebcad030c7ce99b24");
        merklePath[2] = reverseBytes32(hex"32141b5a77bb3bfe86efbd6c24bbb54213f0694abfd927d50d3fe6627c06b51d");
        merklePath[3] = reverseBytes32(hex"a23a60b7f30a4c3e326fa9c8e821c2091043011890d404c2a67bbee074266af4");
        merklePath[4] = reverseBytes32(hex"ae0df2f9f37d5549dab9a79f259090de9b7442543ca03f9da1a2d8e4b1701c22");
        merklePath[5] = reverseBytes32(hex"20ed045b0d4dddca96f1a9e525be0feb09e25d4c8f2686778de7db4cd64e29d3");
        merklePath[6] = reverseBytes32(hex"1c5a155d31616dd90704b83a923f383d80ca7be64feafd19bc1a14ffb6c5e703");
        merklePath[7] = reverseBytes32(hex"57bc38d50a278e7173f7bf93a9dde7d27effdb4525f641a539482357f15a179a");
        merklePath[8] = reverseBytes32(hex"4552b14fd9879a7da1768e093ea52997f0e079f74f0bd9709fa9a267b74748fa");
        merklePath[9] = reverseBytes32(hex"2d841c9ed1ec16a8481cc1d99c8c05ab2c14d76580c02d51d6acfa7b800a30b8");

        bytes32 root = hex"3bf0614fc39756da9bd31f28a0e9568e57893a28ee2be5777f163da2e330f8c2";
        bytes32 reversedRoot = reverseBytes32(root);
        bytes32 txHash = hex"0f2a733f000d577e2df099a7e010e752663916c6287046285d92a51a12ab25ba";
        bytes32 reversedTxHash = reverseBytes32(txHash);

        assertTrue(pool.spvProof1(merklePath, reversedRoot, reversedTxHash, 471), "Valid SPV proof should pass");
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

        assertTrue(pool.spvProof(merklePath, root), "Valid SPV proof should pass");
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

        assertTrue(pool.spvProof(merklePath, root), "Valid SPV proof should pass");
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

        assertTrue(pool.spvProof(merklePath, root), "Valid SPV proof should pass");
    }

    // Failing proofs due to incorrect roots

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
        pool.spvProof(merklePath, wrongRoot);
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
        pool.spvProof(merklePath, wrongRoot);
    }
    */

}
