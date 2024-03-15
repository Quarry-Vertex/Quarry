pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Share} from"../src/Share.sol";
import {QBTC} from"../src/QBTC.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract PoolTest is Test {
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    Pool public pool;
    Share public share;
    QBTC public qbtc;

    address public proxy;
    address public proxyShare;
    address public proxyQBTC;

    // Helper methods
    function createAndSetChainTip(
        bytes32 previousBlockHash,
        bytes32 merkleRootHash
    ) public {
        Pool.ChainTip memory newTip;
        newTip.previousBlockHash = previousBlockHash;
        newTip.merkleRootHash = merkleRootHash;
        pool.setChainTip(newTip);
    }

    function createTestBlock(
        bytes32 previousBlockHash,
        bytes32 merkleRootHash,
        uint32 bits,
        bytes32 outputAddress,
        bytes8 blockReward
    ) public pure returns (Pool.BitcoinBlock memory) {
        // create a block header
        Pool.BlockHeader memory blockHeader;
        blockHeader.version = 10001;
        blockHeader.previousBlockHash = previousBlockHash;
        blockHeader.merkleRootHash = merkleRootHash;
        blockHeader.timestamp = 10001;
        blockHeader.bits = bits;
        blockHeader.nonce = 10001;

        // create a block
        Pool.BitcoinBlock memory curBlock;
        curBlock.header = blockHeader;
        curBlock.outputAddress = outputAddress;
        curBlock.blockReward = blockReward;
        return curBlock;
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

        proxyQBTC = Upgrades.deployUUPSProxy(
          "QBTC.sol",
          abi.encodeCall(share.initialize, ("QBTC", "QBTC", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qbtc = QBTC(proxyQBTC);
        pool.setShareContract(proxyShare);
        pool.setQBTCContract(proxyQBTC);
    }

    function test_initialChainTip() public {
        vm.prank(oracleAddress);
        Pool.ChainTip memory chainTip = pool.getChainTip();
        assertEq(chainTip.previousBlockHash, bytes32(0), "Initial previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32(0), "Initial merkle root hash should be 0");
    }

    function test_setChainTip() public {
        vm.startPrank(oracleAddress);
        Pool.ChainTip memory newTip;
        newTip.previousBlockHash = 0;
        newTip.merkleRootHash = "A";
        pool.setChainTip(newTip);

        Pool.ChainTip memory chainTip = pool.getChainTip();
        vm.stopPrank();

        assertEq(chainTip.previousBlockHash, bytes32(0), "Previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32("A"), "Merkle root hash should be A");
    }

    function test_submitHash() public {
        pool.submitHash(bytes32(testHash), testAddress);
        assertEq(pool.getAddressForSubmittedHash(bytes32(testHash)), testAddress);
    }

    function test_calculateDifficulty() public {
        uint32 bits = 0x1b0404cb;
        uint256 difficulty = pool._calculateDifficulty(bits);
        uint256 expectedDifficulty = 163074209349632;
        assertEq(difficulty, expectedDifficulty);
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

        // get the root
        bytes32 root = txAB;

        assertTrue(pool.spvProof(merklePath, root), "Valid SPV proof should pass");

        // set initial chain tip
        createAndSetChainTip(0, "C");
        // set current chain tip (for block)
        createAndSetChainTip("C", "D");

        vm.stopPrank();

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        assertTrue(pool.submitBlock(curBlock, merklePath, account), "block successfully submitted");
        assertEq(share.ownerOf(0), account, "Owner of share with token id 0");
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

        // get the root
        bytes32 root = "incorrect";

        // set initial chain tip
        createAndSetChainTip(0, "C");
        // set current chain tip (for block)
        createAndSetChainTip("C", "D");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            0x1b0404cb,     // bits
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

        // get the root
        bytes32 root = txAB;

        // set initial chain tip
        createAndSetChainTip(0, "C");
        // set current chain tip (for block) wrong previous hash
        createAndSetChainTip("C", "Wrong");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            0x1b0404cb,     // bits
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

        // get the root
        bytes32 root = txAB;

        // set initial chain tip
        createAndSetChainTip(0, "C");
        // set current chain tip (for block)
        createAndSetChainTip("C", "D");

        // create block params
        // wrong, different from peg in
        bytes32 outputAddress = bytes32(0x2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        // block should be stale
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

        // get the root
        bytes32 root = txAB;

        // set initial chain tip
        createAndSetChainTip(0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));
        // create address
        address account1 = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        address account2 = (0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC);

        // instantiate blocks
        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        createAndSetChainTip("C", "D");
        pool.submitBlock(block1, merklePath, account1);

        Pool.BitcoinBlock memory block2 = createTestBlock(
            "E",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        createAndSetChainTip("D", "E");
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

        proxyQBTC = Upgrades.deployUUPSProxy(
          "QBTC.sol",
          abi.encodeCall(Share.initialize, ("QBTC", "QBTC", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qbtc = QBTC(proxyQBTC);
        pool.setShareContract(proxyShare);
        pool.setQBTCContract(proxyQBTC);
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

        // get the root
        bytes32 root = txAB;

        // set initial chain tip
        createAndSetChainTip(0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));
        // create address
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));

        // instantiate blocks
        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip("C", "D");
        assertTrue(pool.submitBlock(block1, merklePath, account1), "block 1 submitted");

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        // populate merkle path
        merklePath[0] = txC; // curhash (hash of the transaction)
        merklePath[1] = txD;

        // get the root
        root = txCD;

        Pool.BitcoinBlock memory block2 = createTestBlock(
            "E",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip("D", "E");
        assertTrue(pool.submitBlock(block2, merklePath, account2), "block2 submitted");

        assertEq(share.ownerOf(0), account1, "Owner of share with token id 0");
        assertEq(share.ownerOf(1), account2, "Owner of share with token id 1");

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        // populate merkle path
        merklePath[0] = txE; // curhash (hash of the transaction)
        merklePath[1] = txF;

        // get the root
        root = txEF;

        Pool.BitcoinBlock memory block3 = createTestBlock(
            "F",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip("E", "F");
        pool.submitBlock(block3, merklePath, account3);

        assertFalse(share.tokenExists(0), "Pool share with token id 0 was overwritten");
        assertEq(share.ownerOf(1), account2, "Owner of share with token id 1");
        assertEq(share.ownerOf(2), account3, "Owner of share with token id 2");

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

        proxyQBTC = Upgrades.deployUUPSProxy(
          "QBTC.sol",
          abi.encodeCall(Share.initialize, ("QBTC", "QBTC", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qbtc = QBTC(proxyQBTC);
        pool.setShareContract(proxyShare);
        pool.setQBTCContract(proxyQBTC);
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

        // get the root
        bytes32 root = txAB;

        // set initial chain tip
        createAndSetChainTip(0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));
        // create address
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 800))));
        address account7 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));

        // block 1
        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip("C", "D");
        assertTrue(pool.submitBlock(block1, merklePath, account1), "block 1 submitted");

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        // populate merkle path
        merklePath[0] = txC; // curhash (hash of the transaction)
        merklePath[1] = txD;

        // get the root
        root = txCD;

        // block 2
        Pool.BitcoinBlock memory block2 = createTestBlock(
            "E", root,  0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("D", "E");
        assertTrue(pool.submitBlock(block2, merklePath, account2), "block2 submitted");

        assertEq(share.ownerOf(0), account1, "Owner of share with token id 0");
        assertEq(share.ownerOf(1), account2, "Owner of share with token id 1");

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        // populate merkle path
        merklePath[0] = txE; // curhash (hash of the transaction)
        merklePath[1] = txF;

        // get the root
        root = txEF;

        // block 3
        Pool.BitcoinBlock memory block3 = createTestBlock(
            "F", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("E", "F");
        pool.submitBlock(block3, merklePath, account3);

        // block 4
        bytes32 txG = "G";
        bytes32 txH = "H";
        bytes32 txGH = sha256(abi.encodePacked(sha256(abi.encodePacked(txG, txH))));
        merklePath[0] = txG;
        merklePath[1] = txH;
        root = txGH;
        Pool.BitcoinBlock memory block4 = createTestBlock(
            "G", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("F", "G");
        assertTrue(pool.submitBlock(block4, merklePath, account4), "block 4 submitted");

        // block 5
        bytes32 txI = "I";
        bytes32 txJ = "J";
        bytes32 txIJ = sha256(abi.encodePacked(sha256(abi.encodePacked(txI, txJ))));
        merklePath[0] = txI;
        merklePath[1] = txJ;
        root = txIJ;
        Pool.BitcoinBlock memory block5 = createTestBlock(
            "H", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("G", "H");
        assertTrue(pool.submitBlock(block5, merklePath, account5), "block 5 submitted");

        // block 6
        bytes32 txK = "K";
        bytes32 txL = "L";
        bytes32 txKL = sha256(abi.encodePacked(sha256(abi.encodePacked(txK, txL))));
        merklePath[0] = txK;
        merklePath[1] = txL;
        root = txKL;
        Pool.BitcoinBlock memory block6 = createTestBlock(
            "I", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("H", "I");
        assertTrue(pool.submitBlock(block6, merklePath, account6), "block 6 submitted");

        // block 7
        bytes32 txM = "M";
        bytes32 txN = "N";
        bytes32 txMN = sha256(abi.encodePacked(sha256(abi.encodePacked(txM, txN))));
        merklePath[0] = txM;
        merklePath[1] = txN;
        root = txMN;
        Pool.BitcoinBlock memory block7 = createTestBlock(
            "J", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("I", "J");
        assertTrue(pool.submitBlock(block7, merklePath, account7), "block 7 submitted");

        vm.stopPrank();

        // Expect 1000 to be distributed to addresses 3-7
        // because 1 and 2 got evicted from the ring buffer
        pool.distributeRewards(block1);

        assertEq(qbtc.balanceOf(account1), 0, "address1 should not have been rewarded any QBTC");
        assertEq(qbtc.balanceOf(account2), 0, "address2 should not have been rewarded any QBTC");

        assertEq(qbtc.balanceOf(account3), 10000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account4), 10000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account5), 10000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account6), 10000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account7), 10000, "address3 should have been rewarded 1000 QBTC");

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

        proxyQBTC = Upgrades.deployUUPSProxy(
          "QBTC.sol",
          abi.encodeCall(Share.initialize, ("QBTC", "QBTC", proxy))
        );

        pool = Pool(proxy);
        share = Share(proxyShare);
        qbtc = QBTC(proxyQBTC);
        pool.setShareContract(proxyShare);
        pool.setQBTCContract(proxyQBTC);
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

        // get the root
        bytes32 root = txAB;

        // set initial chain tip
        createAndSetChainTip(0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(uint64(50000));
        // create address
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));

        // block 1
        Pool.BitcoinBlock memory block1 = createTestBlock(
            "D",            // previousBlockHash
            root,           // merkleRootHash
            0x1b0404cb,     // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip("C", "D");
        assertTrue(pool.submitBlock(block1, merklePath, account1), "block 1 submitted");

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        // populate merkle path
        merklePath[0] = txC; // curhash (hash of the transaction)
        merklePath[1] = txD;

        // get the root
        root = txCD;

        // block 2
        Pool.BitcoinBlock memory block2 = createTestBlock(
            "E", root,  0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("D", "E");
        assertTrue(pool.submitBlock(block2, merklePath, account2), "block2 submitted");

        assertEq(share.ownerOf(0), account1, "Owner of share with token id 0");
        assertEq(share.ownerOf(1), account2, "Owner of share with token id 1");

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        // populate merkle path
        merklePath[0] = txE; // curhash (hash of the transaction)
        merklePath[1] = txF;

        // get the root
        root = txEF;

        // block 3
        Pool.BitcoinBlock memory block3 = createTestBlock(
            "F", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("E", "F");
        pool.submitBlock(block3, merklePath, account3);

        // block 4
        bytes32 txG = "G";
        bytes32 txH = "H";
        bytes32 txGH = sha256(abi.encodePacked(sha256(abi.encodePacked(txG, txH))));
        merklePath[0] = txG;
        merklePath[1] = txH;
        root = txGH;
        Pool.BitcoinBlock memory block4 = createTestBlock(
            "G", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("F", "G");
        assertTrue(pool.submitBlock(block4, merklePath, account4), "block 4 submitted");

        // block 5
        bytes32 txI = "I";
        bytes32 txJ = "J";
        bytes32 txIJ = sha256(abi.encodePacked(sha256(abi.encodePacked(txI, txJ))));
        merklePath[0] = txI;
        merklePath[1] = txJ;
        root = txIJ;
        Pool.BitcoinBlock memory block5 = createTestBlock(
            "H", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("G", "H");
        assertTrue(pool.submitBlock(block5, merklePath, account5), "block 5 submitted");

        // block 6
        bytes32 txK = "K";
        bytes32 txL = "L";
        bytes32 txKL = sha256(abi.encodePacked(sha256(abi.encodePacked(txK, txL))));
        merklePath[0] = txK;
        merklePath[1] = txL;
        root = txKL;
        Pool.BitcoinBlock memory block6 = createTestBlock(
            "I", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("H", "I");
        assertTrue(pool.submitBlock(block6, merklePath, account6), "block 6 submitted");

        // block 7
        bytes32 txM = "M";
        bytes32 txN = "N";
        bytes32 txMN = sha256(abi.encodePacked(sha256(abi.encodePacked(txM, txN))));
        merklePath[0] = txM;
        merklePath[1] = txN;
        root = txMN;
        Pool.BitcoinBlock memory block7 = createTestBlock(
            "J", root, 0x1b0404cb, outputAddress, blockReward
        );
        createAndSetChainTip("I", "J");
        assertTrue(pool.submitBlock(block7, merklePath, account3), "block 7 submitted");

        vm.stopPrank();

        // Expect 1000 to be distributed to addresses 3-7
        // because 1 and 2 got evicted from the ring buffer
        pool.distributeRewards(block1);

        assertEq(qbtc.balanceOf(account1), 0, "address1 should not have been rewarded any QBTC");
        assertEq(qbtc.balanceOf(account2), 0, "address2 should not have been rewarded any QBTC");

        assertEq(qbtc.balanceOf(account3), 20000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account4), 10000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account5), 10000, "address3 should have been rewarded 1000 QBTC");
        assertEq(qbtc.balanceOf(account6), 10000, "address3 should have been rewarded 1000 QBTC");

        assertFalse(share.tokenExists(0), "Token id 0 should have been burned");
        assertFalse(share.tokenExists(1), "Token id 1 should have been burned");
        assertFalse(share.tokenExists(2), "Token id 2 should have been burned");
        assertFalse(share.tokenExists(3), "Token id 3 should have been burned");
        assertFalse(share.tokenExists(4), "Token id 4 should have been burned");
        assertFalse(share.tokenExists(5), "Token id 5 should have been burned");
        assertFalse(share.tokenExists(6), "Token id 6 should have been burned");
    }
}
