pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Share} from"../src/Share.sol";
import {QSAT} from"../src/QSAT.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract PoolTest is Test {
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);

    Pool public pool;
    Share public share;
    QSAT public qsat;

    address public proxy;
    address public proxyShare;
    address public proxyQSAT;

    // Helper methods
    function createTestBlock(
        bytes32 previousBlockHash,
        bytes32 merkleRootHash,
        uint32 bits,
        bytes32 outputAddress,
        uint256 blockReward
    ) public pure returns (Pool.BitcoinBlock memory) {
        // create a block header
        Pool.BlockHeader memory blockHeader;
        blockHeader.previousBlockHash = previousBlockHash;
        blockHeader.merkleRootHash = merkleRootHash;
        blockHeader.bits = bits;

        // create a block
        Pool.BitcoinBlock memory curBlock;
        curBlock.header = blockHeader;
        curBlock.outputAddress = outputAddress;
        curBlock.blockReward = blockReward;
        return curBlock;
    }

    function createAndSetChainTip(
        bytes32 previousBlockHash,
        bytes32 merkleRootHash,
        uint32 bits,
        bytes32 outputAddress,
        uint256 blockReward
    ) public {
        Pool.BitcoinBlock memory newTip = createTestBlock(previousBlockHash, merkleRootHash, bits, outputAddress, blockReward);
        pool.setChainTip(newTip);
    }

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(share.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQSAT = Upgrades.deployUUPSProxy(
          "QSAT.sol",
          abi.encodeCall(share.initialize, ("QSAT", "QSAT", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qsat = QSAT(proxyQSAT);
        pool.setShareContract(proxyShare);
        pool.setQSATContract(proxyQSAT);
    }

    function test_initialChainTip() public {
        vm.prank(oracleAddress);
        Pool.BitcoinBlock memory chainTip = pool.getChainTip();
        assertEq(chainTip.header.previousBlockHash, bytes32(0), "Initial previous block hash should be 0");
        assertEq(chainTip.header.merkleRootHash, bytes32(0), "Initial merkle root hash should be 0");
    }

    function test_setChainTip() public {
        vm.startPrank(oracleAddress);
        Pool.BitcoinBlock memory newTip = createTestBlock(0, "A", 0, 0, 0);
        pool.setChainTip(newTip);

        Pool.BitcoinBlock memory chainTip = pool.getChainTip();
        vm.stopPrank();

        assertEq(chainTip.header.previousBlockHash, bytes32(0), "Previous block hash should be 0");
        assertEq(chainTip.header.merkleRootHash, bytes32("A"), "Merkle root hash should be A");
    }

    function test_submitHash() public {
        uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

        pool.submitHash(bytes32(testHash), testAddress);
        assertEq(pool.getAddressForSubmittedHash(bytes32(testHash)), testAddress,
            "Address for hash 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7 should be testAddress");
    }

    function test_calculateDifficulty() public {
        uint32 bits = 0x1b0404cb;
        uint256 difficulty = pool._calculateDifficulty(bits);
        uint256 expectedDifficulty = 163074209349632;
        assertEq(difficulty, expectedDifficulty, "Difficulty for bits 0x1b0404cb should be 163074209349632");
    }

    function test_submitOneBlock() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        assertTrue(pool.spvProof(merklePath, txAB), "Valid SPV proof should pass");

        // set initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        vm.stopPrank();

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb; 

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        assertTrue(pool.submitBlock(curBlock, merklePath, account), "Block was not successfully submitted");
        assertEq(share.ownerOf(0), account, "Owner share id 0 should be 0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    }

    function test_submitOneBlockWrongRoot() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // set initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb; 

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            "wrong",        // merkleRootHash (wrong)
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        vm.expectRevert("SPV proof failed");
        pool.submitBlock(curBlock, merklePath, account);
    }

    function test_submitOneBlockWrongTip() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // set initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "Wrong", 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb; 

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash (wrong)
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        vm.expectRevert("Submitted block is stale");
        pool.submitBlock(curBlock, merklePath, account);
    }

    function test_submitOneBlockWrongPegTip() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // set the initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        // create block params
        // wrong, different from peg in
        bytes32 outputAddress = bytes32(0x2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb; 

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash (wrong)
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        vm.expectRevert("Coinbase transaction does not point to quarry peg in address");
        pool.submitBlock(curBlock, merklePath, account);
    }

    function test_submitMultipleBlocksSameHash() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // set initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        // create addresses
        address account1 = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        address account2 = (0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC);
        uint32 bits = 0x1b0404cb; 

        /* Block 1 */

        // instantiate blocks
        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitBlock(block1, merklePath, account1);
        createAndSetChainTip("D", txAB, bits, outputAddress, blockReward);

        /* Block 2 */

        // correct merkle hash
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        Pool.BitcoinBlock memory block2 = createTestBlock(
            txAB,           // previousBlockHash
            txAB,           // merkleRootHash (wrong)
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        createAndSetChainTip(txAB, txCD, bits, outputAddress, blockReward);
        vm.expectRevert("Block hash has already been submitted");
        pool.submitBlock(block2, merklePath, account2);
    }

    function test_submitMultipleBlocks() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 2))
        );

        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(Share.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQSAT = Upgrades.deployUUPSProxy(
          "QSAT.sol",
          abi.encodeCall(Share.initialize, ("QSAT", "QSAT", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qsat = QSAT(proxyQSAT);
        pool.setShareContract(proxyShare);
        pool.setQSATContract(proxyQSAT);
        vm.startPrank(oracleAddress);

        // create addresses
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));

        // set some initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        /* Block 1 */

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb; 

        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        assertTrue(pool.submitBlock(block1, merklePath, account1), "Block 1 was not successfully submitted");
        createAndSetChainTip("D", txAB, bits, outputAddress, blockReward);

        /* Block 2 */

        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        merklePath[0] = txC;
        merklePath[1] = txD;

        Pool.BitcoinBlock memory block2 = createTestBlock(
            txAB, txCD, bits, outputAddress, blockReward
        );

        assertTrue(pool.submitBlock(block2, merklePath, account2), "Block 2 was not successfully submitted");
        createAndSetChainTip(txAB, txCD, bits, outputAddress, blockReward);

        assertEq(share.ownerOf(0), account1, "Owner of share id 0 should be account1");
        assertEq(share.ownerOf(1), account2, "Owner of share id 1 should be account2");

        /* Block 3 */

        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        merklePath[0] = txE;
        merklePath[1] = txF;

        Pool.BitcoinBlock memory block3 = createTestBlock(
            txCD, txEF, bits, outputAddress, blockReward
        );
        pool.submitBlock(block3, merklePath, account3);

        createAndSetChainTip(txCD, txEF, bits, outputAddress, blockReward);

        assertFalse(share.tokenExists(0), "Pool share id 0 should not exist");
        assertEq(share.ownerOf(1), account2, "Owner of share id 1 should be account2");
        assertEq(share.ownerOf(2), account3, "Owner of share id 2 should be account3");

        vm.stopPrank();
    }

    function test_distributeRewards() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 5))
        );
        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(Share.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQSAT = Upgrades.deployUUPSProxy(
          "QSAT.sol",
          abi.encodeCall(Share.initialize, ("QSAT", "QSAT", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qsat = QSAT(proxyQSAT);
        pool.setShareContract(proxyShare);
        pool.setQSATContract(proxyQSAT);
        vm.startPrank(oracleAddress);

        // create addresses
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 800))));
        address account7 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));

        // set some initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        /* Block 1 */

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);

        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        assertTrue(pool.submitBlock(block1, merklePath, account1), "Block 1 was not successfully submitted");
        createAndSetChainTip("D", txAB, bits, outputAddress, blockReward);

        /* Block 2 */

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        merklePath[0] = txC;
        merklePath[1] = txD;

        Pool.BitcoinBlock memory block2 = createTestBlock(
            txAB, txCD, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block2, merklePath, account2), "Block 2 was not successfully submitted");

        assertEq(share.ownerOf(0), account1, "Owner of share id 0 should be account1");
        assertEq(share.ownerOf(1), account2, "Owner of share id 2 should be account2");

        createAndSetChainTip(txAB, txCD, bits, outputAddress, blockReward);

        /* Block 3 */

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        merklePath[0] = txE;
        merklePath[1] = txF;

        Pool.BitcoinBlock memory block3 = createTestBlock(
            txCD, txEF, bits, outputAddress, blockReward
        );
        pool.submitBlock(block3, merklePath, account3);

        createAndSetChainTip(txCD, txEF, bits, outputAddress, blockReward);

        /* Block 4 */

        bytes32 txG = "G";
        bytes32 txH = "H";
        bytes32 txGH = sha256(abi.encodePacked(sha256(abi.encodePacked(txG, txH))));

        merklePath[0] = txG;
        merklePath[1] = txH;

        Pool.BitcoinBlock memory block4 = createTestBlock(
            txEF, txGH, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block4, merklePath, account4), "Block 4 was not successfully submitted");

        createAndSetChainTip(txEF, txGH, bits, outputAddress, blockReward);

        /* Block 5 */

        bytes32 txI = "I";
        bytes32 txJ = "J";
        bytes32 txIJ = sha256(abi.encodePacked(sha256(abi.encodePacked(txI, txJ))));

        merklePath[0] = txI;
        merklePath[1] = txJ;

        Pool.BitcoinBlock memory block5 = createTestBlock(
            txGH, txIJ, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block5, merklePath, account5), "Block 5 was not successfully submitted");

        createAndSetChainTip(txGH, txIJ, bits, outputAddress, blockReward);

        /* Block 6 */

        bytes32 txK = "K";
        bytes32 txL = "L";
        bytes32 txKL = sha256(abi.encodePacked(sha256(abi.encodePacked(txK, txL))));

        merklePath[0] = txK;
        merklePath[1] = txL;

        Pool.BitcoinBlock memory block6 = createTestBlock(
            txIJ, txKL, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block6, merklePath, account6), "Block 6 was not successfully submitted");

        createAndSetChainTip(txIJ, txKL, bits, outputAddress, blockReward);

        /* Block 7 */

        bytes32 txM = "M";
        bytes32 txN = "N";
        bytes32 txMN = sha256(abi.encodePacked(sha256(abi.encodePacked(txM, txN))));

        merklePath[0] = txM;
        merklePath[1] = txN;

        Pool.BitcoinBlock memory block7 = createTestBlock(
            txKL, txMN, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block7, merklePath, account7), "Block 7 was not successfully submitted");

        createAndSetChainTip(txKL, txMN, bits, outputAddress, blockReward);

        vm.stopPrank();

        // Expect 1000 to be distributed to addresses 3-7
        // because 1 and 2 got evicted from the ring buffer
        pool.distributeRewards(block1.header.merkleRootHash);

        assertEq(qsat.balanceOf(account1), 0, "address1 should not have been rewarded any QSAT");
        assertEq(qsat.balanceOf(account2), 0, "address2 should not have been rewarded any QSAT");

        assertEq(qsat.balanceOf(account3), 10000, "address3 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account4), 10000, "address4 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account5), 10000, "address5 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account6), 10000, "address6 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account7), 10000, "address7 should have been rewarded 10000 QSAT");

        assertFalse(share.tokenExists(0), "Token id 0 should have been burned");
        assertFalse(share.tokenExists(1), "Token id 1 should have been burned");
        assertFalse(share.tokenExists(2), "Token id 2 should have been burned");
        assertFalse(share.tokenExists(3), "Token id 3 should have been burned");
        assertFalse(share.tokenExists(4), "Token id 4 should have been burned");
        assertFalse(share.tokenExists(5), "Token id 5 should have been burned");
        assertFalse(share.tokenExists(6), "Token id 6 should have been burned");
    }

    function test_distributeRewardsMultSubmissions() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 5))
        );
        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(Share.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQSAT = Upgrades.deployUUPSProxy(
          "QSAT.sol",
          abi.encodeCall(Share.initialize, ("QSAT", "QSAT", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qsat = QSAT(proxyQSAT);
        pool.setShareContract(proxyShare);
        pool.setQSATContract(proxyQSAT);
        vm.startPrank(oracleAddress);

        // create address
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));

        // set some initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        /* Block 1 */

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);

        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        assertTrue(pool.submitBlock(block1, merklePath, account1), "Block 1 was not successfully submitted");
        createAndSetChainTip("D", txAB, bits, outputAddress, blockReward);

        /* Block 2 */

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        merklePath[0] = txC;
        merklePath[1] = txD;

        Pool.BitcoinBlock memory block2 = createTestBlock(
            txAB, txCD, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block2, merklePath, account2), "Block 2 was not successfully submitted");

        assertEq(share.ownerOf(0), account1, "Owner of share id 0 should be account1");
        assertEq(share.ownerOf(1), account2, "Owner of share id 2 should be account2");

        createAndSetChainTip(txAB, txCD, bits, outputAddress, blockReward);

        /* Block 3 */

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        merklePath[0] = txE;
        merklePath[1] = txF;

        Pool.BitcoinBlock memory block3 = createTestBlock(
            txCD, txEF, bits, outputAddress, blockReward
        );
        pool.submitBlock(block3, merklePath, account3);

        createAndSetChainTip(txCD, txEF, bits, outputAddress, blockReward);

        /* Block 4 */

        bytes32 txG = "G";
        bytes32 txH = "H";
        bytes32 txGH = sha256(abi.encodePacked(sha256(abi.encodePacked(txG, txH))));

        merklePath[0] = txG;
        merklePath[1] = txH;

        Pool.BitcoinBlock memory block4 = createTestBlock(
            txEF, txGH, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block4, merklePath, account4), "Block 4 was not successfully submitted");

        createAndSetChainTip(txEF, txGH, bits, outputAddress, blockReward);

        /* Block 5 */

        bytes32 txI = "I";
        bytes32 txJ = "J";
        bytes32 txIJ = sha256(abi.encodePacked(sha256(abi.encodePacked(txI, txJ))));

        merklePath[0] = txI;
        merklePath[1] = txJ;

        Pool.BitcoinBlock memory block5 = createTestBlock(
            txGH, txIJ, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block5, merklePath, account5), "Block 5 was not successfully submitted");

        createAndSetChainTip(txGH, txIJ, bits, outputAddress, blockReward);

        /* Block 6 */

        bytes32 txK = "K";
        bytes32 txL = "L";
        bytes32 txKL = sha256(abi.encodePacked(sha256(abi.encodePacked(txK, txL))));

        merklePath[0] = txK;
        merklePath[1] = txL;

        Pool.BitcoinBlock memory block6 = createTestBlock(
            txIJ, txKL, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block6, merklePath, account6), "Block 6 was not successfully submitted");

        createAndSetChainTip(txIJ, txKL, bits, outputAddress, blockReward);

        /* Block 7 */

        bytes32 txM = "M";
        bytes32 txN = "N";
        bytes32 txMN = sha256(abi.encodePacked(sha256(abi.encodePacked(txM, txN))));

        merklePath[0] = txM;
        merklePath[1] = txN;

        Pool.BitcoinBlock memory block7 = createTestBlock(
            txKL, txMN, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block7, merklePath, account3), "Block 7 was not successfully submitted");

        createAndSetChainTip(txKL, txMN, bits, outputAddress, blockReward);
        vm.stopPrank();

        // Expect 1000 to be distributed to addresses 3-7
        // because 1 and 2 got evicted from the ring buffer
        pool.distributeRewards(block1.header.merkleRootHash);

        assertEq(qsat.balanceOf(account1), 0, "address1 should not have been rewarded any QSAT");
        assertEq(qsat.balanceOf(account2), 0, "address2 should not have been rewarded any QSAT");

        assertEq(qsat.balanceOf(account3), 20000, "address3 should have been rewarded 20000 QSAT");
        assertEq(qsat.balanceOf(account4), 10000, "address4 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account5), 10000, "address5 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account6), 10000, "address6 should have been rewarded 10000 QSAT");

        assertFalse(share.tokenExists(0), "Token id 0 should have been burned");
        assertFalse(share.tokenExists(1), "Token id 1 should have been burned");
        assertFalse(share.tokenExists(2), "Token id 2 should have been burned");
        assertFalse(share.tokenExists(3), "Token id 3 should have been burned");
        assertFalse(share.tokenExists(4), "Token id 4 should have been burned");
        assertFalse(share.tokenExists(5), "Token id 5 should have been burned");
        assertFalse(share.tokenExists(6), "Token id 6 should have been burned");
    }

    function test_distributeRewardsLessThanN() public {
        proxy = Upgrades.deployUUPSProxy(
            "Pool.sol",
            abi.encodeCall(Pool.initialize, (oracleAddress, pegInAddress, 10))
        );
        proxyShare = Upgrades.deployUUPSProxy(
          "Share.sol",
          abi.encodeCall(Share.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQSAT = Upgrades.deployUUPSProxy(
          "QSAT.sol",
          abi.encodeCall(Share.initialize, ("QSAT", "QSAT", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qsat = QSAT(proxyQSAT);
        pool.setShareContract(proxyShare);
        pool.setQSATContract(proxyQSAT);
        vm.startPrank(oracleAddress);

        // create address
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));
        address account7 = address(bytes20(keccak256(abi.encode(block.timestamp + 800))));

        // set some initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        /* Block 1 */

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);

        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (70000);
        uint32 bits = 0x1b0404cb;

        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        assertTrue(pool.submitBlock(block1, merklePath, account1), "Block 1 was not successfully submitted");
        createAndSetChainTip("D", txAB, bits, outputAddress, blockReward);

        /* Block 2 */

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        merklePath[0] = txC;
        merklePath[1] = txD;

        Pool.BitcoinBlock memory block2 = createTestBlock(
            txAB, txCD, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block2, merklePath, account2), "Block 2 was not successfully submitted");

        assertEq(share.ownerOf(0), account1, "Owner of share id 0 should be account1");
        assertEq(share.ownerOf(1), account2, "Owner of share id 2 should be account2");

        createAndSetChainTip(txAB, txCD, bits, outputAddress, blockReward);

        /* Block 3 */

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        merklePath[0] = txE;
        merklePath[1] = txF;

        Pool.BitcoinBlock memory block3 = createTestBlock(
            txCD, txEF, bits, outputAddress, blockReward
        );
        pool.submitBlock(block3, merklePath, account3);

        createAndSetChainTip(txCD, txEF, bits, outputAddress, blockReward);

        /* Block 4 */

        bytes32 txG = "G";
        bytes32 txH = "H";
        bytes32 txGH = sha256(abi.encodePacked(sha256(abi.encodePacked(txG, txH))));

        merklePath[0] = txG;
        merklePath[1] = txH;

        Pool.BitcoinBlock memory block4 = createTestBlock(
            txEF, txGH, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block4, merklePath, account4), "Block 4 was not successfully submitted");

        createAndSetChainTip(txEF, txGH, bits, outputAddress, blockReward);

        /* Block 5 */

        bytes32 txI = "I";
        bytes32 txJ = "J";
        bytes32 txIJ = sha256(abi.encodePacked(sha256(abi.encodePacked(txI, txJ))));

        merklePath[0] = txI;
        merklePath[1] = txJ;

        Pool.BitcoinBlock memory block5 = createTestBlock(
            txGH, txIJ, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block5, merklePath, account5), "Block 5 was not successfully submitted");

        createAndSetChainTip(txGH, txIJ, bits, outputAddress, blockReward);

        /* Block 6 */

        bytes32 txK = "K";
        bytes32 txL = "L";
        bytes32 txKL = sha256(abi.encodePacked(sha256(abi.encodePacked(txK, txL))));

        merklePath[0] = txK;
        merklePath[1] = txL;

        Pool.BitcoinBlock memory block6 = createTestBlock(
            txIJ, txKL, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block6, merklePath, account6), "Block 6 was not successfully submitted");

        createAndSetChainTip(txIJ, txKL, bits, outputAddress, blockReward);

        /* Block 7 */

        bytes32 txM = "M";
        bytes32 txN = "N";
        bytes32 txMN = sha256(abi.encodePacked(sha256(abi.encodePacked(txM, txN))));

        merklePath[0] = txM;
        merklePath[1] = txN;

        Pool.BitcoinBlock memory block7 = createTestBlock(
            txKL, txMN, bits, outputAddress, blockReward
        );
        assertTrue(pool.submitBlock(block7, merklePath, account7), "Block 7 was not successfully submitted");

        createAndSetChainTip(txKL, txMN, bits, outputAddress, blockReward);
        vm.stopPrank();

        // Expect 1000 to be distributed to all addresses
        pool.distributeRewards(block1.header.merkleRootHash);

        assertEq(qsat.balanceOf(account1), 10000, "address1 should have been rewarded 20000 QSAT");
        assertEq(qsat.balanceOf(account2), 10000, "address2 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account3), 10000, "address3 should have been rewarded 20000 QSAT");
        assertEq(qsat.balanceOf(account4), 10000, "address4 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account5), 10000, "address5 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account6), 10000, "address6 should have been rewarded 10000 QSAT");
        assertEq(qsat.balanceOf(account6), 10000, "address7 should have been rewarded 10000 QSAT");

        assertFalse(share.tokenExists(0), "Token id 0 should have been burned");
        assertFalse(share.tokenExists(1), "Token id 1 should have been burned");
        assertFalse(share.tokenExists(2), "Token id 2 should have been burned");
        assertFalse(share.tokenExists(3), "Token id 3 should have been burned");
        assertFalse(share.tokenExists(4), "Token id 4 should have been burned");
        assertFalse(share.tokenExists(5), "Token id 5 should have been burned");
        assertFalse(share.tokenExists(6), "Token id 6 should have been burned");
    }

    function test_SubmitBlockWrongOutputAddress() public {
        vm.startPrank(oracleAddress);

        // Example of a valid Merkle path for transaction A in the Merkle tree
        bytes32[] memory merklePath = new bytes32[](2);
        // create transaction hashes in merkle path
        bytes32 txA = "A";
        bytes32 txB = "B";
        bytes32 txAB = sha256(abi.encodePacked(sha256(abi.encodePacked(txA, txB))));

        // populate merkle path
        merklePath[0] = txA; // curhash (hash of the transaction)
        merklePath[1] = txB;

        // set initial chain tips
        createAndSetChainTip(0, "C", 0, 0, 0);
        createAndSetChainTip("C", "D", 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes32 incorrectAddress = bytes32(0x1434567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = 50000;
        // create addresses
        address account1 = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        uint32 bits = 0x1b0404cb; 

        /* Block 1 */

        // instantiate blocks
        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            txAB,           // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitBlock(block1, merklePath, account1);
        // force the block to have the wrong peg in address
        block1.outputAddress = incorrectAddress;
        vm.expectRevert("Block's output address does not match quarry peg in address");
        pool.distributeRewards(block1.header.merkleRootHash);
    }
}
