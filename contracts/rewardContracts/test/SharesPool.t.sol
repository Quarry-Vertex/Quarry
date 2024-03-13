pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {PoolShares} from"../src/PoolShares.sol";
import {QuarryBTC} from"../src/QuarryBTC.sol";
import {SharesPool} from"../src/SharesPool.sol";

import "forge-std/console.sol";

contract SharesPoolTest is Test {
    address oracleAddress = 0x5FbDB2315678afecb367f032d93F642f64180aa3; // random address for testing
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
    uint256 testHash = 0x0000000000000000000e3c2f6c0483de8bd2aefb4d3b5f9846ab8e21fb19bc7;

    SharesPool public sharesPool;
    PoolShares public poolShares;
    QuarryBTC public quarryBTC;

    address public proxy;
    address public proxyPoolShares;
    address public proxyQuarryBTC;

    function setUp() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 500))
        );

        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQuarryBTC = Upgrades.deployUUPSProxy(
          "QuarryBTC.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryBTC", "QBTC", proxy))
        );

        sharesPool = SharesPool(proxy);
        poolShares = PoolShares(proxyPoolShares);
        quarryBTC = QuarryBTC(proxyQuarryBTC);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        sharesPool.setQuarryBTCContract(proxyQuarryBTC);
    }

    function test_initialChainTip() public {
        vm.prank(oracleAddress);
        SharesPool.ChainTip memory chainTip = sharesPool.getChainTip();
        assertEq(chainTip.previousBlockHash, bytes32(0), "Initial previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32(0), "Initial merkle root hash should be 0");
    }

    function test_setChainTip() public {
        vm.startPrank(oracleAddress);
        SharesPool.ChainTip memory newTip;
        newTip.previousBlockHash = 0;
        newTip.merkleRootHash = "A";
        sharesPool.setChainTip(newTip);

        SharesPool.ChainTip memory chainTip = sharesPool.getChainTip();
        vm.stopPrank();

        assertEq(chainTip.previousBlockHash, bytes32(0), "Previous block hash should be 0");
        assertEq(chainTip.merkleRootHash, bytes32("A"), "Merkle root hash should be A");
    }

    function test_submitHash() public {
        sharesPool.submitHash(bytes32(testHash), testAddress);
        assertEq(sharesPool.getAddressForSubmittedHash(bytes32(testHash)), testAddress);
    }

    function test_calculateDifficulty() public {
        uint32 bits = 0x1b0404cb;
        uint256 difficulty = sharesPool._calculateDifficulty(bits);
        uint256 expectedDifficulty = 163074209349632;
        assertEq(difficulty, expectedDifficulty);
    }

    function createAndSetChainTip(
        SharesPool sharesPool,
        bytes32 previousBlockHash,
        bytes32 merkleRootHash
    ) public {
        SharesPool.ChainTip memory newTip;
        newTip.previousBlockHash = previousBlockHash;
        newTip.merkleRootHash = merkleRootHash;
        sharesPool.setChainTip(newTip);
    }
    function createTestBlock(
        uint32 version,
        bytes32 previousBlockHash,
        bytes32 merkleRootHash,
        uint32 timestamp,
        uint32 bits,
        uint32 nonce,
        bytes32 outputAddress,
        bytes8 blockReward
    ) public returns (SharesPool.BitcoinBlock memory) {
        // create a block header
        SharesPool.BlockHeader memory blockHeader;
        blockHeader.version = version;
        blockHeader.previousBlockHash = previousBlockHash;
        blockHeader.merkleRootHash = merkleRootHash;
        blockHeader.timestamp = timestamp;
        blockHeader.bits = bits;
        blockHeader.nonce = nonce;

        // create a block
        SharesPool.BitcoinBlock memory curBlock;
        curBlock.header = blockHeader;
        curBlock.outputAddress = outputAddress;
        curBlock.blockReward = blockReward;
        return curBlock;
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

        assertTrue(sharesPool.spvProof(merklePath, root), "Valid SPV proof should pass");

        // set initial chain tip
        createAndSetChainTip(sharesPool, 0, "C");
        // set current chain tip (for block)
        createAndSetChainTip(sharesPool, "C", "D");

        vm.stopPrank();

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));

        // instantiate a block
        SharesPool.BitcoinBlock memory curBlock = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        assertTrue(sharesPool.submitBlock(curBlock, merklePath, account), "block successfully submitted");
        assertEq(poolShares.getOwnerOfShare(0), account, "Owner of share with token id 0");
    }

    function test_submitOneBlockWrongRoot() public {
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
        bytes32 root = "incorrect";

        // set initial chain tip
        createAndSetChainTip(sharesPool, 0, "C");
        // set current chain tip (for block)
        createAndSetChainTip(sharesPool, "C", "D");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));

        // instantiate a block
        SharesPool.BitcoinBlock memory curBlock = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        vm.expectRevert("SPV proof failed");
        sharesPool.submitBlock(curBlock, merklePath, account);
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
        createAndSetChainTip(sharesPool, 0, "C");
        // set current chain tip (for block) wrong previous hash
        createAndSetChainTip(sharesPool, "C", "Wrong");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));

        // instantiate a block
        SharesPool.BitcoinBlock memory curBlock = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        vm.expectRevert("Submitted block is stale");
        sharesPool.submitBlock(curBlock, merklePath, account);
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
        createAndSetChainTip(sharesPool, 0, "C");
        // set current chain tip (for block)
        createAndSetChainTip(sharesPool, "C", "D");

        // create block params
        // wrong, different from peg in
        bytes32 outputAddress = bytes32(0x2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));

        // instantiate a block
        SharesPool.BitcoinBlock memory curBlock = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        // block should be stale
        vm.expectRevert("Coinbase transaction does not point to quarry peg in address");
        sharesPool.submitBlock(curBlock, merklePath, account);
    }

    function test_submitMultipleBlocksSameHash() public {
        sharesPool = SharesPool(proxy);
        poolShares = PoolShares(proxyPoolShares);
        quarryBTC = QuarryBTC(proxyQuarryBTC);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        sharesPool.setQuarryBTCContract(proxyQuarryBTC);
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
        createAndSetChainTip(sharesPool, 0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));
        // create address
        address account1 = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        address account2 = (0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC);

        // instantiate blocks
        SharesPool.BitcoinBlock memory block1 = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        createAndSetChainTip(sharesPool, "C", "D");
        sharesPool.submitBlock(block1, merklePath, account1);

        SharesPool.BitcoinBlock memory block2 = createTestBlock(
            10001,          // version
            "E",            // previousBlockHash
            root,           // merkleRootHash
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        createAndSetChainTip(sharesPool, "D", "E");
        vm.expectRevert("Block hash has already been submitted");
        sharesPool.submitBlock(block2, merklePath, account2);
    }

    function test_submitMultipleBlocks() public {
        proxy = Upgrades.deployUUPSProxy(
            "SharesPool.sol",
            abi.encodeCall(SharesPool.initialize, (oracleAddress, pegInAddress, 2))
        );

        proxyPoolShares = Upgrades.deployUUPSProxy(
          "PoolShares.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryShares", "QShare", proxy))
        );

        proxyQuarryBTC = Upgrades.deployUUPSProxy(
          "QuarryBTC.sol",
          abi.encodeCall(PoolShares.initialize, ("QuarryBTC", "QBTC", proxy))
        );

        sharesPool = SharesPool(proxy);
        poolShares = PoolShares(proxyPoolShares);
        quarryBTC = QuarryBTC(proxyQuarryBTC);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        sharesPool.setQuarryBTCContract(proxyQuarryBTC);
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
        createAndSetChainTip(sharesPool, 0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));
        // create address
        address account1 = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        address account2 = (0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC);
        address account3 = (0x90F79bf6EB2c4f870365E785982E1f101E93b906);

        // instantiate blocks
        SharesPool.BitcoinBlock memory block1 = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip(sharesPool, "C", "D");
        assertTrue(sharesPool.submitBlock(block1, merklePath, account1), "block 1 submitted");

        // create transaction hashes in merkle path
        bytes32 txC = "C";
        bytes32 txD = "D";
        bytes32 txCD = sha256(abi.encodePacked(sha256(abi.encodePacked(txC, txD))));

        // populate merkle path
        merklePath[0] = txC; // curhash (hash of the transaction)
        merklePath[1] = txD;

        // get the root
        root = txCD;

        SharesPool.BitcoinBlock memory block2 = createTestBlock(
            10001,          // version
            "E",            // previousBlockHash
            root,           // merkleRootHash
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip(sharesPool, "D", "E");
        assertTrue(sharesPool.submitBlock(block2, merklePath, account2), "block2 submitted");

        assertEq(poolShares.getOwnerOfShare(0), account1, "Owner of share with token id 0");
        assertEq(poolShares.getOwnerOfShare(1), account2, "Owner of share with token id 1");

        // create transaction hashes in merkle path
        bytes32 txE = "E";
        bytes32 txF = "F";
        bytes32 txEF = sha256(abi.encodePacked(sha256(abi.encodePacked(txE, txF))));

        // populate merkle path
        merklePath[0] = txE; // curhash (hash of the transaction)
        merklePath[1] = txF;

        // get the root
        root = txEF;

        SharesPool.BitcoinBlock memory block3 = createTestBlock(
            10001,          // version
            "F",            // previousBlockHash
            root,           // merkleRootHash (wrong)
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip(sharesPool, "E", "F");
        sharesPool.submitBlock(block3, merklePath, account3);

        vm.expectRevert();
        poolShares.getOwnerOfShare(0);

        vm.stopPrank();
    }

    function test_distributeRewards() public {
        sharesPool = SharesPool(proxy);
        poolShares = PoolShares(proxyPoolShares);
        quarryBTC = QuarryBTC(proxyQuarryBTC);
        sharesPool.setPoolSharesContract(proxyPoolShares);
        sharesPool.setQuarryBTCContract(proxyQuarryBTC);
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
        createAndSetChainTip(sharesPool, 0, "C");

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        bytes8 blockReward = bytes8(bytes32(uint256(0x12345678)));
        // create address
        address account1 = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        address account2 = (0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC);
        address account3 = (0x90F79bf6EB2c4f870365E785982E1f101E93b906);
        address account4 = (0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65);
        address account5 = (0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc);
        address account6 = (0x976EA74026E726554dB657fA54763abd0C3a0aa9);
        address account7 = (0x14dC79964da2C08b23698B3D3cc7Ca32193d9955);

        // block 1
        SharesPool.BitcoinBlock memory block1 = createTestBlock(
            10001,          // version
            "D",            // previousBlockHash
            root,           // merkleRootHash
            10001,          // timestamp
            0x1b0404cb,     // bits
            10001,          // nonce
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );
        createAndSetChainTip(sharesPool, "C", "D");
        assertTrue(sharesPool.submitBlock(block1, merklePath, account1), "block 1 submitted");

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
        SharesPool.BitcoinBlock memory block2 = createTestBlock(
            10001, "E", root, 10001, 0x1b0404cb, 10001, outputAddress, blockReward
        );
        createAndSetChainTip(sharesPool, "D", "E");
        assertTrue(sharesPool.submitBlock(block2, merklePath, account2), "block2 submitted");

        assertEq(poolShares.getOwnerOfShare(0), account1, "Owner of share with token id 0");
        assertEq(poolShares.getOwnerOfShare(1), account2, "Owner of share with token id 1");

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
        SharesPool.BitcoinBlock memory block3 = createTestBlock(
            10001, "F", root, 10001, 0x1b0404cb, 10001, outputAddress, blockReward
        );
        createAndSetChainTip(sharesPool, "E", "F");
        sharesPool.submitBlock(block3, merklePath, account3);

        // block 4
        bytes32 txG = "G";
        bytes32 txH = "H";
        bytes32 txGH = sha256(abi.encodePacked(sha256(abi.encodePacked(txG, txH))));
        merklePath[0] = txG;
        merklePath[1] = txH;
        root = txGH;
        SharesPool.BitcoinBlock memory block4 = createTestBlock(
            10001, "G", root, 10001, 0x1b0404cb, 10001, outputAddress, blockReward
        );
        createAndSetChainTip(sharesPool, "F", "G");
        assertTrue(sharesPool.submitBlock(block4, merklePath, account4), "block 4 submitted");

        // block 5
        bytes32 txI = "I";
        bytes32 txJ = "J";
        bytes32 txIJ = sha256(abi.encodePacked(sha256(abi.encodePacked(txI, txJ))));
        merklePath[0] = txI;
        merklePath[1] = txJ;
        root = txIJ;
        SharesPool.BitcoinBlock memory block5 = createTestBlock(
            10001, "H", root, 10001, 0x1b0404cb, 10001, outputAddress, blockReward
        );
        createAndSetChainTip(sharesPool, "G", "H");
        assertTrue(sharesPool.submitBlock(block5, merklePath, account5), "block 5 submitted");

        // block 6
        bytes32 txK = "K";
        bytes32 txL = "L";
        bytes32 txKL = sha256(abi.encodePacked(sha256(abi.encodePacked(txK, txL))));
        merklePath[0] = txK;
        merklePath[1] = txL;
        root = txKL;
        SharesPool.BitcoinBlock memory block6 = createTestBlock(
            10001, "I", root, 10001, 0x1b0404cb, 10001, outputAddress, blockReward
        );
        createAndSetChainTip(sharesPool, "H", "I");
        assertTrue(sharesPool.submitBlock(block6, merklePath, account6), "block 6 submitted");

        // block 7
        bytes32 txM = "M";
        bytes32 txN = "N";
        bytes32 txMN = sha256(abi.encodePacked(sha256(abi.encodePacked(txM, txN))));
        merklePath[0] = txM;
        merklePath[1] = txN;
        root = txMN;
        SharesPool.BitcoinBlock memory block7 = createTestBlock(
            10001, "J", root, 10001, 0x1b0404cb, 10001, outputAddress, blockReward
        );
        createAndSetChainTip(sharesPool, "I", "J");
        assertTrue(sharesPool.submitBlock(block7, merklePath, account7), "block 7 submitted");

        sharesPool.distributeRewards(block1);

        vm.stopPrank();
    }
}
