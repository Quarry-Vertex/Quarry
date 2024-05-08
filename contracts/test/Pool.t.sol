pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Share} from"../src/Share.sol";
import {QSAT} from"../src/QSAT.sol";
import {QSATBridge} from"../src/QSATBridge.sol";
import {Pool} from"../src/Pool.sol";

import "forge-std/console.sol";

contract PoolTest is Test {
    address oracleAddress = address(bytes20(keccak256(abi.encode(block.timestamp))));
    address testAddress = address(bytes20(keccak256(abi.encode(block.timestamp + 100))));
    bytes32 pegInAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);

    Pool public pool;
    Share public share;
    QSAT public qsat;
    QSATBridge public qsatBridge;

    // Helper methods
    function createTestBlock(
        bytes32 blockHash,
        bytes32 previousBlockHash,
        bytes32 merkleRootHash,
        uint32 bits,
        bytes32 outputAddress,
        uint256 blockReward
    ) public pure returns (Pool.BitcoinBlock memory) {
        // create a block header
        Pool.BlockHeader memory blockHeader;
        blockHeader.blockHash = blockHash;
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
        bytes32 blockHash,
        bytes32 previousBlockHash,
        bytes32 merkleRootHash,
        uint32 bits,
        bytes32 outputAddress,
        uint256 blockReward
    ) public {
        Pool.BitcoinBlock memory newTip = createTestBlock(blockHash, previousBlockHash, merkleRootHash, bits, outputAddress, blockReward);
        pool.setChainTip(newTip);
    }

    function setUp() public {
        pool = new Pool();
        pool.initialize(oracleAddress, pegInAddress, 500);

        qsatBridge = new QSATBridge();
        qsatBridge.initialize(oracleAddress, address(pool));

        qsat = new QSAT();
        qsat.initialize("QSAT", "QSAT", address(qsatBridge));

        share = new Share();
        share.initialize("QuarryShares", "QShare", address(pool));

        qsatBridge.setQSATContract(address(qsat));
        pool.setShareContract(address(share));
        pool.setQSATBridgeContract(address(qsatBridge));
    }

    function test_initialChainTip() public {
        vm.prank(oracleAddress);
        Pool.BitcoinBlock memory chainTip = pool.getChainTip();
        assertEq(chainTip.header.blockHash, bytes32(0), "Initial block hash should be 0");
        assertEq(chainTip.header.previousBlockHash, bytes32(0), "Initial previous block hash should be 0");
        assertEq(chainTip.header.merkleRootHash, bytes32(0), "Initial merkle root hash should be 0");
    }

    function test_setChainTip() public {
        vm.startPrank(oracleAddress);
        Pool.BitcoinBlock memory newTip = createTestBlock("block 0", "block 1", "A", 0, 0, 0);
        pool.setChainTip(newTip);

        Pool.BitcoinBlock memory chainTip = pool.getChainTip();
        vm.stopPrank();

        assertEq(chainTip.header.blockHash, bytes32("block 0"), "Current block hash should be 1");
        assertEq(chainTip.header.previousBlockHash, bytes32("block 1"), "Previous block hash should be 2");
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

        // initial block
        // height 841770
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        vm.stopPrank();

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 txHash = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            blockHash,      // block hash
            rootTip,        // previousBlockHash
            merkleRoot,     // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        pool.submitHash(curBlock.header.blockHash, account);
        assertTrue(pool.submitBlock(curBlock, txHash, proof, account), "Block was not successfully submitted");
        assertEq(share.ownerOf(0), account, "Owner share id 0 should be 0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    }

    function test_submitOneBlockWrongRoot() public {
        vm.startPrank(oracleAddress);

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        vm.stopPrank();

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        // changed one number
        bytes32 merkleRootWrong = hex"76db6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 txHash = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";
        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            blockHash,       // block hash
            rootTip,         // previousBlockHash
            merkleRootWrong, // merkleRootHash
            bits,            // bits
            outputAddress,   // outputAddress
            blockReward      // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        pool.submitHash(curBlock.header.blockHash, account);

        vm.expectRevert("SPV proof failed");
        pool.submitBlock(curBlock, txHash, proof, account);
    }

    function test_submitOneBlockWrongTip() public {
        vm.startPrank(oracleAddress);

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        vm.stopPrank();

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 txHash = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            blockHash,       // block hash
            rootPrev,        // previousBlockHash
            merkleRoot,      // Merkle root
            bits,            // bits
            outputAddress,   // outputAddress
            blockReward      // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        pool.submitHash(curBlock.header.blockHash, account);

        vm.expectRevert("Submitted block is stale");
        pool.submitBlock(curBlock, txHash, proof, account);
    }

    function test_submitOneBlockWrongPegTip() public {
        vm.startPrank(oracleAddress);

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        vm.stopPrank();

        // create block params
        // wrong peg in
        bytes32 outputAddress = bytes32(0x2234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 txHash = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory curBlock = createTestBlock(
            blockHash,      // block hash
            rootTip,        // previousBlockHash
            merkleRoot,     // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        pool.submitHash(curBlock.header.blockHash, account);

        vm.expectRevert("Coinbase transaction does not point to quarry peg in address");
        pool.submitBlock(curBlock, txHash, proof, account);
    }

    function test_submitMultipleBlocksSameHash() public {
        vm.startPrank(oracleAddress);

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 txHash = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory block1 = createTestBlock(
            blockHash,      // block hash
            rootTip,        // previousBlockHash
            merkleRoot,     // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        pool.submitHash(block1.header.blockHash, account);

        pool.submitBlock(block1, txHash, proof, account);
        createAndSetChainTip(blockHash, rootTip, txHash, bits, outputAddress, blockReward);

        // Block 2

        bytes32 merkleRoot2 = hex"1c71abf07208944cf0f5236086f88174491100440631258b4b0c57f840aa369f";
        bytes32 tx2 = hex"cec7bf4feb8fbe9f78e38e1b9ebaf1f4cb7df80a6c87ffc481d4fd130340d333";
        bytes memory proof2 = hex"975ce494bf811025a383d03b20f35e5887ef1c5f8c7651dd187ff93013938480aea4529abe4c6ac9846167d4ed151bb9385479a61f1e7fc3db3e7068e91e93397bbc726060857a84fb1cbf8e338594673e7a071073153f54651dc3c380eee9762faee26991ce83c841afe70ed65eb6a26bee2a07a41b9629b976c72318b5ead296300e1f72d7b8f4985c9ad7639a9f3696cef2a810f8b1aa1ddc2060d075b54365ba022b9523e2da289270a7d47944809eac23e91c976cab287ff45e6eec9ca3e4552efa855a984a09f4ca476c7b11aa75a3ee0e84f6a770d5abbe4d73dae744d88aca07848d24fcb22270ebd91bf2120074737cd58de6bc58cb9881bd090c756ab5d3904ab9577ae5c42aaccbae773e8d33e05364bf7f4be774484abf313642d6cc5b7fba9d71a4d5999e0d1862d9937c416ecf2f30630fae8d327ab8520e89cc6be5d6ae920abe03a907653575243292d8f052221d1a97d7af0ca4ec7061c08c37d2e3b1db8aebc723397e5c209476dd3282ed7a2bb1db78588c48034a2b3e";

        Pool.BitcoinBlock memory block2 = createTestBlock(
            blockHash,      // block hash (same as block 1)
            blockHash,      // previousBlockHash
            merkleRoot2,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block2.header.blockHash, account2);

        createAndSetChainTip(blockHash, blockHash, tx2, bits, outputAddress, blockReward);
        vm.expectRevert("Block hash has already been submitted");
        pool.submitBlock(block2, tx2, proof2, account2);
    }

    function test_submitMultipleBlocks() public {
        pool = new Pool();
        pool.initialize(oracleAddress, pegInAddress, 2);

        qsatBridge = new QSATBridge();
        qsatBridge.initialize(oracleAddress, address(pool));

        qsat = new QSAT();
        qsat.initialize("QSAT", "QSAT", address(qsatBridge));

        share = new Share();
        share.initialize("QuarryShares", "QShare", address(pool));

        qsatBridge.setQSATContract(address(qsat));
        pool.setShareContract(address(share));
        pool.setQSATBridgeContract(address(qsatBridge));

        vm.startPrank(oracleAddress);

        // create addresses
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash1 = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot1 = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 tx1 = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof1 = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory block1 = createTestBlock(
            blockHash1,     // block hash
            rootTip,        // previousBlockHash
            merkleRoot1,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block1.header.blockHash, account1);
        pool.submitBlock(block1, tx1, proof1, account1);
        createAndSetChainTip(blockHash1, rootTip, tx1, bits, outputAddress, blockReward);

        // Block 2 (4)
        bytes32 blockHash2 = hex"0000000000000000000013e54d657504fb9a5640ec5c04d388313f0b6b15614c";
        bytes32 merkleRoot2 = hex"1c71abf07208944cf0f5236086f88174491100440631258b4b0c57f840aa369f";
        bytes32 tx2 = hex"cec7bf4feb8fbe9f78e38e1b9ebaf1f4cb7df80a6c87ffc481d4fd130340d333";
        bytes memory proof2 = hex"975ce494bf811025a383d03b20f35e5887ef1c5f8c7651dd187ff93013938480aea4529abe4c6ac9846167d4ed151bb9385479a61f1e7fc3db3e7068e91e93397bbc726060857a84fb1cbf8e338594673e7a071073153f54651dc3c380eee9762faee26991ce83c841afe70ed65eb6a26bee2a07a41b9629b976c72318b5ead296300e1f72d7b8f4985c9ad7639a9f3696cef2a810f8b1aa1ddc2060d075b54365ba022b9523e2da289270a7d47944809eac23e91c976cab287ff45e6eec9ca3e4552efa855a984a09f4ca476c7b11aa75a3ee0e84f6a770d5abbe4d73dae744d88aca07848d24fcb22270ebd91bf2120074737cd58de6bc58cb9881bd090c756ab5d3904ab9577ae5c42aaccbae773e8d33e05364bf7f4be774484abf313642d6cc5b7fba9d71a4d5999e0d1862d9937c416ecf2f30630fae8d327ab8520e89cc6be5d6ae920abe03a907653575243292d8f052221d1a97d7af0ca4ec7061c08c37d2e3b1db8aebc723397e5c209476dd3282ed7a2bb1db78588c48034a2b3e";

        Pool.BitcoinBlock memory block2 = createTestBlock(
            blockHash2,     // block hash
            blockHash1,     // previousBlockHash
            merkleRoot2,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block2.header.blockHash, account2);
        assertTrue(pool.submitBlock(block2, tx2, proof2, account2), "Block 2 was not successfully submitted");
        createAndSetChainTip(blockHash2, blockHash1, tx2, bits, outputAddress, blockReward);

        // Block 3
        bytes32 blockHash3 = hex"00000000000000000000b18d8b9e899ec37c07838fcd05bb102e8c64a13a7033";
        bytes32 merkleRoot3 = hex"bf114b680c042e27ee669f2e39311145b12a518dde8c06cf9c38f57aa6c23207";
        bytes32 tx3 = hex"be23073c650f9bfbbee15dc605b74b7881a9e404ed392739eeb78b5adf18fffc";
        bytes memory proof3 = hex"0b8140177031e07f2e4920955b97c94041d2c2f8d94ed122720be9fd0c31cf6dabe2b446a0c0b829f3799380f45da0418ea84b68e1b1ac77ae79afe82fed4586613edaf22e424f5454643a8d847724d1d2d8bce5103d02785a36c520787d4bc337bbc0953073f1a3b968f1d2c0e6fc337202bc51b942e073d27755e544d0f863ddba205918f71a45021f5a6c17db368ddcc35c169bde5a63b3dfc762bc2b45315613ea381620c39c814ebf4ea32c6496042deb61ff1ae8f866df1a2a6a0200123bbf075d3e04fcc32312b00935e3998a2ad2509560ea1c221bdaab9327ef9a4725eb79f06405282dd9c45d2a4625b866d1d07d89c711642bbe9cb289702015ca8767b93cfb9167b104768d8e0664a277c98d806e32f59315eefbcc3ab374648be65afb941db3a558a3d5c0d8fab0dc5834c9c3f5825e1484d7f0dbd0219124edb002807ab225ca18ca1d01a74c6a958dea09dc8fd1ee48750a507eaf4c0e54bb6651318984c7c796912c33fa875143bf2cdd27964e4e0cebbbdd12b4b3482145d7581b1a7eb389d76936ff2d01b3be4b0e5a8eb0e9e7cbc36e3d92d06c1f4a50";

        Pool.BitcoinBlock memory block3 = createTestBlock(
            blockHash3,     // block hash
            blockHash2,     // previousBlockHash
            merkleRoot3,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block3.header.blockHash, account3);
        assertTrue(pool.submitBlock(block3, tx3, proof3, account3), "Block 3 was not successfully submitted");
        createAndSetChainTip(blockHash3, blockHash2, tx3, bits, outputAddress, blockReward);

        assertFalse(share.tokenExists(0), "Pool share id 0 should not exist");
        assertEq(share.ownerOf(1), account2, "Owner of share id 1 should be account2");
        assertEq(share.ownerOf(2), account3, "Owner of share id 2 should be account3");

        vm.stopPrank();
    }

    function test_distributeRewards() public {
        pool = new Pool();
        pool.initialize(oracleAddress, pegInAddress, 5);

        qsatBridge = new QSATBridge();
        qsatBridge.initialize(oracleAddress, address(pool));

        qsat = new QSAT();
        qsat.initialize("QSAT", "QSAT", address(qsatBridge));

        share = new Share();
        share.initialize("QuarryShares", "QShare", address(pool));

        qsatBridge.setQSATContract(address(qsat));
        pool.setShareContract(address(share));
        pool.setQSATBridgeContract(address(qsatBridge));

        vm.startPrank(oracleAddress);

        // create addresses
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 800))));
        address account7 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash1 = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot1 = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 tx1 = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof1 = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory block1 = createTestBlock(
            blockHash1,     // block hash
            rootTip,        // previousBlockHash
            merkleRoot1,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block1.header.blockHash, account1);
        pool.submitBlock(block1, tx1, proof1, account1);
        createAndSetChainTip(blockHash1, rootTip, tx1, bits, outputAddress, blockReward);

        // Block 2 (4)
        bytes32 blockHash2 = hex"0000000000000000000013e54d657504fb9a5640ec5c04d388313f0b6b15614c";
        bytes32 merkleRoot2 = hex"1c71abf07208944cf0f5236086f88174491100440631258b4b0c57f840aa369f";
        bytes32 tx2 = hex"cec7bf4feb8fbe9f78e38e1b9ebaf1f4cb7df80a6c87ffc481d4fd130340d333";
        bytes memory proof2 = hex"975ce494bf811025a383d03b20f35e5887ef1c5f8c7651dd187ff93013938480aea4529abe4c6ac9846167d4ed151bb9385479a61f1e7fc3db3e7068e91e93397bbc726060857a84fb1cbf8e338594673e7a071073153f54651dc3c380eee9762faee26991ce83c841afe70ed65eb6a26bee2a07a41b9629b976c72318b5ead296300e1f72d7b8f4985c9ad7639a9f3696cef2a810f8b1aa1ddc2060d075b54365ba022b9523e2da289270a7d47944809eac23e91c976cab287ff45e6eec9ca3e4552efa855a984a09f4ca476c7b11aa75a3ee0e84f6a770d5abbe4d73dae744d88aca07848d24fcb22270ebd91bf2120074737cd58de6bc58cb9881bd090c756ab5d3904ab9577ae5c42aaccbae773e8d33e05364bf7f4be774484abf313642d6cc5b7fba9d71a4d5999e0d1862d9937c416ecf2f30630fae8d327ab8520e89cc6be5d6ae920abe03a907653575243292d8f052221d1a97d7af0ca4ec7061c08c37d2e3b1db8aebc723397e5c209476dd3282ed7a2bb1db78588c48034a2b3e";

        Pool.BitcoinBlock memory block2 = createTestBlock(
            blockHash2,     // block hash
            blockHash1,     // previousBlockHash
            merkleRoot2,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block2.header.blockHash, account2);
        assertTrue(pool.submitBlock(block2, tx2, proof2, account2), "Block 2 was not successfully submitted");
        createAndSetChainTip(blockHash2, blockHash1, tx2, bits, outputAddress, blockReward);

        // Block 3
        bytes32 blockHash3 = hex"00000000000000000000b18d8b9e899ec37c07838fcd05bb102e8c64a13a7033";
        bytes32 merkleRoot3 = hex"bf114b680c042e27ee669f2e39311145b12a518dde8c06cf9c38f57aa6c23207";
        bytes32 tx3 = hex"be23073c650f9bfbbee15dc605b74b7881a9e404ed392739eeb78b5adf18fffc";
        bytes memory proof3 = hex"0b8140177031e07f2e4920955b97c94041d2c2f8d94ed122720be9fd0c31cf6dabe2b446a0c0b829f3799380f45da0418ea84b68e1b1ac77ae79afe82fed4586613edaf22e424f5454643a8d847724d1d2d8bce5103d02785a36c520787d4bc337bbc0953073f1a3b968f1d2c0e6fc337202bc51b942e073d27755e544d0f863ddba205918f71a45021f5a6c17db368ddcc35c169bde5a63b3dfc762bc2b45315613ea381620c39c814ebf4ea32c6496042deb61ff1ae8f866df1a2a6a0200123bbf075d3e04fcc32312b00935e3998a2ad2509560ea1c221bdaab9327ef9a4725eb79f06405282dd9c45d2a4625b866d1d07d89c711642bbe9cb289702015ca8767b93cfb9167b104768d8e0664a277c98d806e32f59315eefbcc3ab374648be65afb941db3a558a3d5c0d8fab0dc5834c9c3f5825e1484d7f0dbd0219124edb002807ab225ca18ca1d01a74c6a958dea09dc8fd1ee48750a507eaf4c0e54bb6651318984c7c796912c33fa875143bf2cdd27964e4e0cebbbdd12b4b3482145d7581b1a7eb389d76936ff2d01b3be4b0e5a8eb0e9e7cbc36e3d92d06c1f4a50";

        Pool.BitcoinBlock memory block3 = createTestBlock(
            blockHash3,     // block hash
            blockHash2,     // previousBlockHash
            merkleRoot3,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block3.header.blockHash, account3);
        pool.submitBlock(block3, tx3, proof3, account3);
        createAndSetChainTip(blockHash3, blockHash2, tx3, bits, outputAddress, blockReward);

        // Block 4
        bytes32 blockHash4 = hex"00000000000000000002639cc859cf01486b6eb732f72204993e56cff78ce7b5";
        bytes32 merkleRoot4 = hex"1f963e48e6bebc6b3a26a42e39945ecd278268f0d6b655c0076d17706c71eaee";
        bytes32 tx4 = hex"2438fdba446b2333be44382df8927e6cc639bb2eae607bd87fca02141819459a";
        bytes memory proof4 = hex"48f1a63706c32f1ea889ce97f130d27cb9ea85e71e974f7ede0dc36a2962bfc4955d20e8fe83f223eeb697c06377aed2d5de83eef8710be89497477ed731af5511284f5f75eb51c627c4ab647063135eb6a6326e4ce1503f65ef1203b0b26b68e8f6f34ab52bea497002afc2e149f06c963793b70fd95b43de2653c8f6b44f0754494b6c2f7ba03ce52c2abc5a741764fc554bcbd860d2a7e19ee5f58b486f797835fb5a21e562dbf1299b9dd59ba89713489369c0b5eeae52d6c629365fc78727d8c81917ef75726ccee99bf38782ac4b179617b5b0ed783157d219f2ed473c7e77107979a6fa0727c5fbac6bfcfc0c0974389b1ecc3c8a9554f4b15d1826b5cb1c1cc34a805e45e9f8324de4813a9bf72dfb3f2d9807237dda15cde78f891a7eb344dcbad46665c800463a5d4c49ee6ab109e17ea0fab3415aaca43214e5139d714768c15e6010b2e00536aa5de9111ba9d3cf156d6427d5f733302af396581592fd10879716e45150c4dd9ba0e7bfc35b634306b3bf9763a16dd09a78bd6b1945dfbb6e2348f6ea0881d905f53635a5693ea435662ca333e6ccabc44b4878";

        Pool.BitcoinBlock memory block4 = createTestBlock(
            blockHash4,     // block hash
            blockHash3,     // previousBlockHash
            merkleRoot4,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block4.header.blockHash, account4);
        pool.submitBlock(block4, tx4, proof4, account4);
        createAndSetChainTip(blockHash4, blockHash3, tx4, bits, outputAddress, blockReward);

        // Block 5
        bytes32 blockHash5 = hex"00000000000000000000a174eebf5b9df9b9ebf062cc3c503a2024e7d9a618b6";
        bytes32 merkleRoot5 = hex"ea775c1d413d95198516d39d6fd1cfcffb37509f6d489dbd9aa3cae8765a9093";
        bytes32 tx5 = hex"1de793d68e2a400e684fcb45e2d3c9edfbd40694b8a97a741ca705e34f77edc8";
        bytes memory proof5 = hex"1ab4580980216c40d1f68541a44f70442d702ea01620a1b89d1c2b1cc006715f1999e7f179f824a7d5045a4663bdd322f85d3294f113090950b6150e7ae4cfbab9c671d24f84d8d00526f49aeaa6523b79918166fa1f3415565bf6bf8bfe79045c84ab7f328238caf4333c050bb02c1359702a480522e8780794bae305185e79f0397b57587b1f1268f4900e3f90d8485950413bc7979bdcf949bb46d0c5fa5bd973682f1c10c272517fa0c5b66d6ff2bbd8a1114765cdda2b4f4ee86945e9e9db84a20cbcafb76360f71365db0d357467972d23b1a264e7f339daa0d8353fb89473a980ed900260fe8e9bf1ffdc6719f0c7bbd54378bf4ffe30c1903cacdaa078b9a5eb36b45e26c89325c1f4816cce691e021b713aa28d5fb5090da48981e47a80314634917d8734638ccc80f4be850733331d7a911551e01270732dbf8c9a36321f64e9677dfc486cb6a38e640aa63e1c5e0f222728100b0a801e6bf43949acc145653bef6421ec5b87d8056a5b90fc8c3d9002aae6e046488b9e2c5c4cadfb16fff0459998024f324b3be890019f5403c9a816b71614e1d62d6fa4455f51";

        Pool.BitcoinBlock memory block5 = createTestBlock(
            blockHash5,     // block hash
            blockHash4,     // previousBlockHash
            merkleRoot5,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block5.header.blockHash, account5);
        pool.submitBlock(block5, tx5, proof5, account5);
        createAndSetChainTip(blockHash5, blockHash4, tx5, bits, outputAddress, blockReward);

        // Block 6
        bytes32 blockHash6 = hex"0000000000000000000129131b65d9a3c6b7af1f47f83074ea879c4c531787ce";
        bytes32 merkleRoot6 = hex"6b8f1f71602d7a21a40c68d3f1d7c46eba2fe8ca926968506f138edcdcc5d22e";
        bytes32 tx6 = hex"7cf13a87bc52cdcb988627565d886a9a50f78c99fcbac645045e5c84220d7006";
        bytes memory proof6 = hex"a32de63bbc1fafc6219935c41239795b7e2df0a4eb6dcf8debb2639a3323dc93330f051dfa2d646e992a847f0a67751399cb01c209ee4e165ca399dee060f9473a8ae680899b57abbf1513aec2fcde4b91d8a5bb519c1c81e4e026632a6cd6cf0b14e8c8f052a88d60e9ecaa9c4e0db318bb95dd17c8d6ae19d75f2211b9b2fd3ecd82392c27b025c20fc517e3af1f1ad092efabffb94c8af9ebcbc973c7bf621793f4154bb901b3257f49c8f9176c72c753c7323dbfccd75c45c54bab4c2aa7a6c1e4204e8b3399d3a7bc2a32fe8501c60ac4f7550b971cb1b36acafc28fa9af90c2ed8cc1dd61f148b841e97ff8383e59fa2ac921b48315e8dee1bca6cc2f31589bcacde9aefd085037b2ee448597cb80e91bbdae681032da6fff4215b465f39a49b77dd5a4c02c639d749c8294e763d21dd20b11c7480d5cc82d47b67f3dd739eb1230377c808e07a00bab3a233ba90faa10e7f1b1f515fc8ee019b952f96b75edcd51437ebea33562c2f35f963624616249cebeb8994264ad6ba89c53876119d34f926548fbaacc1ea41488a5a78ed99d560eb42380d02c8c876f2493f6c";

        Pool.BitcoinBlock memory block6 = createTestBlock(
            blockHash6,     // block hash
            blockHash5,     // previousBlockHash
            merkleRoot6,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block6.header.blockHash, account6);
        pool.submitBlock(block6, tx6, proof6, account6);
        createAndSetChainTip(blockHash6, blockHash5, tx3, bits, outputAddress, blockReward);

        // Block 7
        bytes32 blockHash7 = hex"0000000000000000000079495db7e2b7c3bddd1beb9b2e4e70b16991c24863f3";
        bytes32 merkleRoot7 = hex"6b8f1f71602d7a21a40c68d3f1d7c46eba2fe8ca926968506f138edcdcc5d22e";
        bytes32 tx7 = hex"7cf13a87bc52cdcb988627565d886a9a50f78c99fcbac645045e5c84220d7006";
        bytes memory proof7 = hex"a32de63bbc1fafc6219935c41239795b7e2df0a4eb6dcf8debb2639a3323dc93330f051dfa2d646e992a847f0a67751399cb01c209ee4e165ca399dee060f9473a8ae680899b57abbf1513aec2fcde4b91d8a5bb519c1c81e4e026632a6cd6cf0b14e8c8f052a88d60e9ecaa9c4e0db318bb95dd17c8d6ae19d75f2211b9b2fd3ecd82392c27b025c20fc517e3af1f1ad092efabffb94c8af9ebcbc973c7bf621793f4154bb901b3257f49c8f9176c72c753c7323dbfccd75c45c54bab4c2aa7a6c1e4204e8b3399d3a7bc2a32fe8501c60ac4f7550b971cb1b36acafc28fa9af90c2ed8cc1dd61f148b841e97ff8383e59fa2ac921b48315e8dee1bca6cc2f31589bcacde9aefd085037b2ee448597cb80e91bbdae681032da6fff4215b465f39a49b77dd5a4c02c639d749c8294e763d21dd20b11c7480d5cc82d47b67f3dd739eb1230377c808e07a00bab3a233ba90faa10e7f1b1f515fc8ee019b952f96b75edcd51437ebea33562c2f35f963624616249cebeb8994264ad6ba89c53876119d34f926548fbaacc1ea41488a5a78ed99d560eb42380d02c8c876f2493f6c";

        Pool.BitcoinBlock memory block7 = createTestBlock(
            blockHash7,     // block hash
            blockHash6,     // previousBlockHash
            merkleRoot7,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block7.header.blockHash, account7);
        pool.submitBlock(block7, tx7, proof7, account7);
        createAndSetChainTip(blockHash7, blockHash6, tx7, bits, outputAddress, blockReward);
        vm.stopPrank();

        // Expect 1000 to be distributed to addresses 3-7
        // because 1 and 2 got evicted from the ring buffer
        pool.distributeRewards(block1.header.blockHash);

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
        pool = new Pool();
        pool.initialize(oracleAddress, pegInAddress, 5);

        qsatBridge = new QSATBridge();
        qsatBridge.initialize(oracleAddress, address(pool));

        qsat = new QSAT();
        qsat.initialize("QSAT", "QSAT", address(qsatBridge));

        share = new Share();
        share.initialize("QuarryShares", "QShare", address(pool));

        qsatBridge.setQSATContract(address(qsat));
        pool.setShareContract(address(share));
        pool.setQSATBridgeContract(address(qsatBridge));

        vm.startPrank(oracleAddress);

        // create addresses
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 800))));

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash1 = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot1 = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 tx1 = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof1 = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory block1 = createTestBlock(
            blockHash1,     // block hash
            rootTip,        // previousBlockHash
            merkleRoot1,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block1.header.blockHash, account1);
        pool.submitBlock(block1, tx1, proof1, account1);
        createAndSetChainTip(blockHash1, rootTip, tx1, bits, outputAddress, blockReward);

        // Block 2 (4)
        bytes32 blockHash2 = hex"0000000000000000000013e54d657504fb9a5640ec5c04d388313f0b6b15614c";
        bytes32 merkleRoot2 = hex"1c71abf07208944cf0f5236086f88174491100440631258b4b0c57f840aa369f";
        bytes32 tx2 = hex"cec7bf4feb8fbe9f78e38e1b9ebaf1f4cb7df80a6c87ffc481d4fd130340d333";
        bytes memory proof2 = hex"975ce494bf811025a383d03b20f35e5887ef1c5f8c7651dd187ff93013938480aea4529abe4c6ac9846167d4ed151bb9385479a61f1e7fc3db3e7068e91e93397bbc726060857a84fb1cbf8e338594673e7a071073153f54651dc3c380eee9762faee26991ce83c841afe70ed65eb6a26bee2a07a41b9629b976c72318b5ead296300e1f72d7b8f4985c9ad7639a9f3696cef2a810f8b1aa1ddc2060d075b54365ba022b9523e2da289270a7d47944809eac23e91c976cab287ff45e6eec9ca3e4552efa855a984a09f4ca476c7b11aa75a3ee0e84f6a770d5abbe4d73dae744d88aca07848d24fcb22270ebd91bf2120074737cd58de6bc58cb9881bd090c756ab5d3904ab9577ae5c42aaccbae773e8d33e05364bf7f4be774484abf313642d6cc5b7fba9d71a4d5999e0d1862d9937c416ecf2f30630fae8d327ab8520e89cc6be5d6ae920abe03a907653575243292d8f052221d1a97d7af0ca4ec7061c08c37d2e3b1db8aebc723397e5c209476dd3282ed7a2bb1db78588c48034a2b3e";

        Pool.BitcoinBlock memory block2 = createTestBlock(
            blockHash2,      // block hash
            blockHash1,      // previousBlockHash
            merkleRoot2,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block2.header.blockHash, account2);
        assertTrue(pool.submitBlock(block2, tx2, proof2, account2), "Block 2 was not successfully submitted");
        createAndSetChainTip(blockHash2, blockHash1, tx2, bits, outputAddress, blockReward);

        // Block 3
        bytes32 blockHash3 = hex"00000000000000000000b18d8b9e899ec37c07838fcd05bb102e8c64a13a7033";
        bytes32 merkleRoot3 = hex"bf114b680c042e27ee669f2e39311145b12a518dde8c06cf9c38f57aa6c23207";
        bytes32 tx3 = hex"be23073c650f9bfbbee15dc605b74b7881a9e404ed392739eeb78b5adf18fffc";
        bytes memory proof3 = hex"0b8140177031e07f2e4920955b97c94041d2c2f8d94ed122720be9fd0c31cf6dabe2b446a0c0b829f3799380f45da0418ea84b68e1b1ac77ae79afe82fed4586613edaf22e424f5454643a8d847724d1d2d8bce5103d02785a36c520787d4bc337bbc0953073f1a3b968f1d2c0e6fc337202bc51b942e073d27755e544d0f863ddba205918f71a45021f5a6c17db368ddcc35c169bde5a63b3dfc762bc2b45315613ea381620c39c814ebf4ea32c6496042deb61ff1ae8f866df1a2a6a0200123bbf075d3e04fcc32312b00935e3998a2ad2509560ea1c221bdaab9327ef9a4725eb79f06405282dd9c45d2a4625b866d1d07d89c711642bbe9cb289702015ca8767b93cfb9167b104768d8e0664a277c98d806e32f59315eefbcc3ab374648be65afb941db3a558a3d5c0d8fab0dc5834c9c3f5825e1484d7f0dbd0219124edb002807ab225ca18ca1d01a74c6a958dea09dc8fd1ee48750a507eaf4c0e54bb6651318984c7c796912c33fa875143bf2cdd27964e4e0cebbbdd12b4b3482145d7581b1a7eb389d76936ff2d01b3be4b0e5a8eb0e9e7cbc36e3d92d06c1f4a50";

        Pool.BitcoinBlock memory block3 = createTestBlock(
            blockHash3,     // block hash
            blockHash2,     // previousBlockHash
            merkleRoot3,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block3.header.blockHash, account3);
        pool.submitBlock(block3, tx3, proof3, account3);
        createAndSetChainTip(blockHash3, blockHash2, tx3, bits, outputAddress, blockReward);

        // Block 4
        bytes32 blockHash4 = hex"00000000000000000002639cc859cf01486b6eb732f72204993e56cff78ce7b5";
        bytes32 merkleRoot4 = hex"1f963e48e6bebc6b3a26a42e39945ecd278268f0d6b655c0076d17706c71eaee";
        bytes32 tx4 = hex"2438fdba446b2333be44382df8927e6cc639bb2eae607bd87fca02141819459a";
        bytes memory proof4 = hex"48f1a63706c32f1ea889ce97f130d27cb9ea85e71e974f7ede0dc36a2962bfc4955d20e8fe83f223eeb697c06377aed2d5de83eef8710be89497477ed731af5511284f5f75eb51c627c4ab647063135eb6a6326e4ce1503f65ef1203b0b26b68e8f6f34ab52bea497002afc2e149f06c963793b70fd95b43de2653c8f6b44f0754494b6c2f7ba03ce52c2abc5a741764fc554bcbd860d2a7e19ee5f58b486f797835fb5a21e562dbf1299b9dd59ba89713489369c0b5eeae52d6c629365fc78727d8c81917ef75726ccee99bf38782ac4b179617b5b0ed783157d219f2ed473c7e77107979a6fa0727c5fbac6bfcfc0c0974389b1ecc3c8a9554f4b15d1826b5cb1c1cc34a805e45e9f8324de4813a9bf72dfb3f2d9807237dda15cde78f891a7eb344dcbad46665c800463a5d4c49ee6ab109e17ea0fab3415aaca43214e5139d714768c15e6010b2e00536aa5de9111ba9d3cf156d6427d5f733302af396581592fd10879716e45150c4dd9ba0e7bfc35b634306b3bf9763a16dd09a78bd6b1945dfbb6e2348f6ea0881d905f53635a5693ea435662ca333e6ccabc44b4878";

        Pool.BitcoinBlock memory block4 = createTestBlock(
            blockHash4,     // block hash
            blockHash3,     // previousBlockHash
            merkleRoot4,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block4.header.blockHash, account4);
        pool.submitBlock(block4, tx4, proof4, account4);
        createAndSetChainTip(blockHash4, blockHash3, tx4, bits, outputAddress, blockReward);

        // Block 5
        bytes32 blockHash5 = hex"00000000000000000000a174eebf5b9df9b9ebf062cc3c503a2024e7d9a618b6";
        bytes32 merkleRoot5 = hex"ea775c1d413d95198516d39d6fd1cfcffb37509f6d489dbd9aa3cae8765a9093";
        bytes32 tx5 = hex"1de793d68e2a400e684fcb45e2d3c9edfbd40694b8a97a741ca705e34f77edc8";
        bytes memory proof5 = hex"1ab4580980216c40d1f68541a44f70442d702ea01620a1b89d1c2b1cc006715f1999e7f179f824a7d5045a4663bdd322f85d3294f113090950b6150e7ae4cfbab9c671d24f84d8d00526f49aeaa6523b79918166fa1f3415565bf6bf8bfe79045c84ab7f328238caf4333c050bb02c1359702a480522e8780794bae305185e79f0397b57587b1f1268f4900e3f90d8485950413bc7979bdcf949bb46d0c5fa5bd973682f1c10c272517fa0c5b66d6ff2bbd8a1114765cdda2b4f4ee86945e9e9db84a20cbcafb76360f71365db0d357467972d23b1a264e7f339daa0d8353fb89473a980ed900260fe8e9bf1ffdc6719f0c7bbd54378bf4ffe30c1903cacdaa078b9a5eb36b45e26c89325c1f4816cce691e021b713aa28d5fb5090da48981e47a80314634917d8734638ccc80f4be850733331d7a911551e01270732dbf8c9a36321f64e9677dfc486cb6a38e640aa63e1c5e0f222728100b0a801e6bf43949acc145653bef6421ec5b87d8056a5b90fc8c3d9002aae6e046488b9e2c5c4cadfb16fff0459998024f324b3be890019f5403c9a816b71614e1d62d6fa4455f51";

        Pool.BitcoinBlock memory block5 = createTestBlock(
            blockHash5,     // block hash
            blockHash4,     // previousBlockHash
            merkleRoot5,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block5.header.blockHash, account5);
        pool.submitBlock(block5, tx5, proof5, account5);
        createAndSetChainTip(blockHash5, blockHash4, tx5, bits, outputAddress, blockReward);

        // Block 6
        bytes32 blockHash6 = hex"0000000000000000000129131b65d9a3c6b7af1f47f83074ea879c4c531787ce";
        bytes32 merkleRoot6 = hex"6b8f1f71602d7a21a40c68d3f1d7c46eba2fe8ca926968506f138edcdcc5d22e";
        bytes32 tx6 = hex"7cf13a87bc52cdcb988627565d886a9a50f78c99fcbac645045e5c84220d7006";
        bytes memory proof6 = hex"a32de63bbc1fafc6219935c41239795b7e2df0a4eb6dcf8debb2639a3323dc93330f051dfa2d646e992a847f0a67751399cb01c209ee4e165ca399dee060f9473a8ae680899b57abbf1513aec2fcde4b91d8a5bb519c1c81e4e026632a6cd6cf0b14e8c8f052a88d60e9ecaa9c4e0db318bb95dd17c8d6ae19d75f2211b9b2fd3ecd82392c27b025c20fc517e3af1f1ad092efabffb94c8af9ebcbc973c7bf621793f4154bb901b3257f49c8f9176c72c753c7323dbfccd75c45c54bab4c2aa7a6c1e4204e8b3399d3a7bc2a32fe8501c60ac4f7550b971cb1b36acafc28fa9af90c2ed8cc1dd61f148b841e97ff8383e59fa2ac921b48315e8dee1bca6cc2f31589bcacde9aefd085037b2ee448597cb80e91bbdae681032da6fff4215b465f39a49b77dd5a4c02c639d749c8294e763d21dd20b11c7480d5cc82d47b67f3dd739eb1230377c808e07a00bab3a233ba90faa10e7f1b1f515fc8ee019b952f96b75edcd51437ebea33562c2f35f963624616249cebeb8994264ad6ba89c53876119d34f926548fbaacc1ea41488a5a78ed99d560eb42380d02c8c876f2493f6c";

        Pool.BitcoinBlock memory block6 = createTestBlock(
            blockHash6,     // block hash
            blockHash5,     // previousBlockHash
            merkleRoot6,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block6.header.blockHash, account6);
        pool.submitBlock(block6, tx6, proof6, account6);
        createAndSetChainTip(blockHash6, blockHash5, tx3, bits, outputAddress, blockReward);

        // Block 7
        bytes32 blockHash7 = hex"0000000000000000000079495db7e2b7c3bddd1beb9b2e4e70b16991c24863f3";
        bytes32 merkleRoot7 = hex"6b8f1f71602d7a21a40c68d3f1d7c46eba2fe8ca926968506f138edcdcc5d22e";
        bytes32 tx7 = hex"7cf13a87bc52cdcb988627565d886a9a50f78c99fcbac645045e5c84220d7006";
        bytes memory proof7 = hex"a32de63bbc1fafc6219935c41239795b7e2df0a4eb6dcf8debb2639a3323dc93330f051dfa2d646e992a847f0a67751399cb01c209ee4e165ca399dee060f9473a8ae680899b57abbf1513aec2fcde4b91d8a5bb519c1c81e4e026632a6cd6cf0b14e8c8f052a88d60e9ecaa9c4e0db318bb95dd17c8d6ae19d75f2211b9b2fd3ecd82392c27b025c20fc517e3af1f1ad092efabffb94c8af9ebcbc973c7bf621793f4154bb901b3257f49c8f9176c72c753c7323dbfccd75c45c54bab4c2aa7a6c1e4204e8b3399d3a7bc2a32fe8501c60ac4f7550b971cb1b36acafc28fa9af90c2ed8cc1dd61f148b841e97ff8383e59fa2ac921b48315e8dee1bca6cc2f31589bcacde9aefd085037b2ee448597cb80e91bbdae681032da6fff4215b465f39a49b77dd5a4c02c639d749c8294e763d21dd20b11c7480d5cc82d47b67f3dd739eb1230377c808e07a00bab3a233ba90faa10e7f1b1f515fc8ee019b952f96b75edcd51437ebea33562c2f35f963624616249cebeb8994264ad6ba89c53876119d34f926548fbaacc1ea41488a5a78ed99d560eb42380d02c8c876f2493f6c";

        Pool.BitcoinBlock memory block7 = createTestBlock(
            blockHash7,     // block hash
            blockHash6,     // previousBlockHash
            merkleRoot7,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block7.header.blockHash, account3);
        pool.submitBlock(block7, tx7, proof7, account3);
        createAndSetChainTip(blockHash7, blockHash6, tx7, bits, outputAddress, blockReward);
        vm.stopPrank();

        // Expect 1000 to be distributed to addresses 3-7
        // because 1 and 2 got evicted from the ring buffer
        pool.distributeRewards(block1.header.blockHash);

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
        pool = new Pool();
        pool.initialize(oracleAddress, pegInAddress, 10);

        qsatBridge = new QSATBridge();
        qsatBridge.initialize(oracleAddress, address(pool));

        qsat = new QSAT();
        qsat.initialize("QSAT", "QSAT", address(qsatBridge));

        share = new Share();
        share.initialize("QuarryShares", "QShare", address(pool));

        qsatBridge.setQSATContract(address(qsat));
        pool.setShareContract(address(share));
        pool.setQSATBridgeContract(address(qsatBridge));

        vm.startPrank(oracleAddress);

        // create addresses
        address account1 = address(bytes20(keccak256(abi.encode(block.timestamp + 300))));
        address account2 = address(bytes20(keccak256(abi.encode(block.timestamp + 400))));
        address account3 = address(bytes20(keccak256(abi.encode(block.timestamp + 500))));
        address account4 = address(bytes20(keccak256(abi.encode(block.timestamp + 600))));
        address account5 = address(bytes20(keccak256(abi.encode(block.timestamp + 700))));
        address account6 = address(bytes20(keccak256(abi.encode(block.timestamp + 800))));
        address account7 = address(bytes20(keccak256(abi.encode(block.timestamp + 900))));

        // initial block
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (70000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash1 = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot1 = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 tx1 = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof1 = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory block1 = createTestBlock(
            blockHash1,     // block hash
            rootTip,        // previousBlockHash
            merkleRoot1,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block1.header.blockHash, account1);
        pool.submitBlock(block1, tx1, proof1, account1);
        createAndSetChainTip(blockHash1, rootTip, tx1, bits, outputAddress, blockReward);

        // Block 2 (4)
        bytes32 blockHash2 = hex"0000000000000000000013e54d657504fb9a5640ec5c04d388313f0b6b15614c";
        bytes32 merkleRoot2 = hex"1c71abf07208944cf0f5236086f88174491100440631258b4b0c57f840aa369f";
        bytes32 tx2 = hex"cec7bf4feb8fbe9f78e38e1b9ebaf1f4cb7df80a6c87ffc481d4fd130340d333";
        bytes memory proof2 = hex"975ce494bf811025a383d03b20f35e5887ef1c5f8c7651dd187ff93013938480aea4529abe4c6ac9846167d4ed151bb9385479a61f1e7fc3db3e7068e91e93397bbc726060857a84fb1cbf8e338594673e7a071073153f54651dc3c380eee9762faee26991ce83c841afe70ed65eb6a26bee2a07a41b9629b976c72318b5ead296300e1f72d7b8f4985c9ad7639a9f3696cef2a810f8b1aa1ddc2060d075b54365ba022b9523e2da289270a7d47944809eac23e91c976cab287ff45e6eec9ca3e4552efa855a984a09f4ca476c7b11aa75a3ee0e84f6a770d5abbe4d73dae744d88aca07848d24fcb22270ebd91bf2120074737cd58de6bc58cb9881bd090c756ab5d3904ab9577ae5c42aaccbae773e8d33e05364bf7f4be774484abf313642d6cc5b7fba9d71a4d5999e0d1862d9937c416ecf2f30630fae8d327ab8520e89cc6be5d6ae920abe03a907653575243292d8f052221d1a97d7af0ca4ec7061c08c37d2e3b1db8aebc723397e5c209476dd3282ed7a2bb1db78588c48034a2b3e";

        Pool.BitcoinBlock memory block2 = createTestBlock(
            blockHash2,      // block hash
            blockHash1,      // previousBlockHash
            merkleRoot2,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block2.header.blockHash, account2);
        assertTrue(pool.submitBlock(block2, tx2, proof2, account2), "Block 2 was not successfully submitted");
        createAndSetChainTip(blockHash2, blockHash1, tx2, bits, outputAddress, blockReward);

        // Block 3
        bytes32 blockHash3 = hex"00000000000000000000b18d8b9e899ec37c07838fcd05bb102e8c64a13a7033";
        bytes32 merkleRoot3 = hex"bf114b680c042e27ee669f2e39311145b12a518dde8c06cf9c38f57aa6c23207";
        bytes32 tx3 = hex"be23073c650f9bfbbee15dc605b74b7881a9e404ed392739eeb78b5adf18fffc";
        bytes memory proof3 = hex"0b8140177031e07f2e4920955b97c94041d2c2f8d94ed122720be9fd0c31cf6dabe2b446a0c0b829f3799380f45da0418ea84b68e1b1ac77ae79afe82fed4586613edaf22e424f5454643a8d847724d1d2d8bce5103d02785a36c520787d4bc337bbc0953073f1a3b968f1d2c0e6fc337202bc51b942e073d27755e544d0f863ddba205918f71a45021f5a6c17db368ddcc35c169bde5a63b3dfc762bc2b45315613ea381620c39c814ebf4ea32c6496042deb61ff1ae8f866df1a2a6a0200123bbf075d3e04fcc32312b00935e3998a2ad2509560ea1c221bdaab9327ef9a4725eb79f06405282dd9c45d2a4625b866d1d07d89c711642bbe9cb289702015ca8767b93cfb9167b104768d8e0664a277c98d806e32f59315eefbcc3ab374648be65afb941db3a558a3d5c0d8fab0dc5834c9c3f5825e1484d7f0dbd0219124edb002807ab225ca18ca1d01a74c6a958dea09dc8fd1ee48750a507eaf4c0e54bb6651318984c7c796912c33fa875143bf2cdd27964e4e0cebbbdd12b4b3482145d7581b1a7eb389d76936ff2d01b3be4b0e5a8eb0e9e7cbc36e3d92d06c1f4a50";

        Pool.BitcoinBlock memory block3 = createTestBlock(
            blockHash3,     // block hash
            blockHash2,     // previousBlockHash
            merkleRoot3,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block3.header.blockHash, account3);
        pool.submitBlock(block3, tx3, proof3, account3);
        createAndSetChainTip(blockHash3, blockHash2, tx3, bits, outputAddress, blockReward);

        // Block 4
        bytes32 blockHash4 = hex"00000000000000000002639cc859cf01486b6eb732f72204993e56cff78ce7b5";
        bytes32 merkleRoot4 = hex"1f963e48e6bebc6b3a26a42e39945ecd278268f0d6b655c0076d17706c71eaee";
        bytes32 tx4 = hex"2438fdba446b2333be44382df8927e6cc639bb2eae607bd87fca02141819459a";
        bytes memory proof4 = hex"48f1a63706c32f1ea889ce97f130d27cb9ea85e71e974f7ede0dc36a2962bfc4955d20e8fe83f223eeb697c06377aed2d5de83eef8710be89497477ed731af5511284f5f75eb51c627c4ab647063135eb6a6326e4ce1503f65ef1203b0b26b68e8f6f34ab52bea497002afc2e149f06c963793b70fd95b43de2653c8f6b44f0754494b6c2f7ba03ce52c2abc5a741764fc554bcbd860d2a7e19ee5f58b486f797835fb5a21e562dbf1299b9dd59ba89713489369c0b5eeae52d6c629365fc78727d8c81917ef75726ccee99bf38782ac4b179617b5b0ed783157d219f2ed473c7e77107979a6fa0727c5fbac6bfcfc0c0974389b1ecc3c8a9554f4b15d1826b5cb1c1cc34a805e45e9f8324de4813a9bf72dfb3f2d9807237dda15cde78f891a7eb344dcbad46665c800463a5d4c49ee6ab109e17ea0fab3415aaca43214e5139d714768c15e6010b2e00536aa5de9111ba9d3cf156d6427d5f733302af396581592fd10879716e45150c4dd9ba0e7bfc35b634306b3bf9763a16dd09a78bd6b1945dfbb6e2348f6ea0881d905f53635a5693ea435662ca333e6ccabc44b4878";

        Pool.BitcoinBlock memory block4 = createTestBlock(
            blockHash4,     // block hash
            blockHash3,     // previousBlockHash
            merkleRoot4,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block4.header.blockHash, account4);
        pool.submitBlock(block4, tx4, proof4, account4);
        createAndSetChainTip(blockHash4, blockHash3, tx4, bits, outputAddress, blockReward);

        // Block 5
        bytes32 blockHash5 = hex"00000000000000000000a174eebf5b9df9b9ebf062cc3c503a2024e7d9a618b6";
        bytes32 merkleRoot5 = hex"ea775c1d413d95198516d39d6fd1cfcffb37509f6d489dbd9aa3cae8765a9093";
        bytes32 tx5 = hex"1de793d68e2a400e684fcb45e2d3c9edfbd40694b8a97a741ca705e34f77edc8";
        bytes memory proof5 = hex"1ab4580980216c40d1f68541a44f70442d702ea01620a1b89d1c2b1cc006715f1999e7f179f824a7d5045a4663bdd322f85d3294f113090950b6150e7ae4cfbab9c671d24f84d8d00526f49aeaa6523b79918166fa1f3415565bf6bf8bfe79045c84ab7f328238caf4333c050bb02c1359702a480522e8780794bae305185e79f0397b57587b1f1268f4900e3f90d8485950413bc7979bdcf949bb46d0c5fa5bd973682f1c10c272517fa0c5b66d6ff2bbd8a1114765cdda2b4f4ee86945e9e9db84a20cbcafb76360f71365db0d357467972d23b1a264e7f339daa0d8353fb89473a980ed900260fe8e9bf1ffdc6719f0c7bbd54378bf4ffe30c1903cacdaa078b9a5eb36b45e26c89325c1f4816cce691e021b713aa28d5fb5090da48981e47a80314634917d8734638ccc80f4be850733331d7a911551e01270732dbf8c9a36321f64e9677dfc486cb6a38e640aa63e1c5e0f222728100b0a801e6bf43949acc145653bef6421ec5b87d8056a5b90fc8c3d9002aae6e046488b9e2c5c4cadfb16fff0459998024f324b3be890019f5403c9a816b71614e1d62d6fa4455f51";

        Pool.BitcoinBlock memory block5 = createTestBlock(
            blockHash5,     // block hash
            blockHash4,     // previousBlockHash
            merkleRoot5,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block5.header.blockHash, account5);
        pool.submitBlock(block5, tx5, proof5, account5);
        createAndSetChainTip(blockHash5, blockHash4, tx5, bits, outputAddress, blockReward);

        // Block 6
        bytes32 blockHash6 = hex"0000000000000000000129131b65d9a3c6b7af1f47f83074ea879c4c531787ce";
        bytes32 merkleRoot6 = hex"6b8f1f71602d7a21a40c68d3f1d7c46eba2fe8ca926968506f138edcdcc5d22e";
        bytes32 tx6 = hex"7cf13a87bc52cdcb988627565d886a9a50f78c99fcbac645045e5c84220d7006";
        bytes memory proof6 = hex"a32de63bbc1fafc6219935c41239795b7e2df0a4eb6dcf8debb2639a3323dc93330f051dfa2d646e992a847f0a67751399cb01c209ee4e165ca399dee060f9473a8ae680899b57abbf1513aec2fcde4b91d8a5bb519c1c81e4e026632a6cd6cf0b14e8c8f052a88d60e9ecaa9c4e0db318bb95dd17c8d6ae19d75f2211b9b2fd3ecd82392c27b025c20fc517e3af1f1ad092efabffb94c8af9ebcbc973c7bf621793f4154bb901b3257f49c8f9176c72c753c7323dbfccd75c45c54bab4c2aa7a6c1e4204e8b3399d3a7bc2a32fe8501c60ac4f7550b971cb1b36acafc28fa9af90c2ed8cc1dd61f148b841e97ff8383e59fa2ac921b48315e8dee1bca6cc2f31589bcacde9aefd085037b2ee448597cb80e91bbdae681032da6fff4215b465f39a49b77dd5a4c02c639d749c8294e763d21dd20b11c7480d5cc82d47b67f3dd739eb1230377c808e07a00bab3a233ba90faa10e7f1b1f515fc8ee019b952f96b75edcd51437ebea33562c2f35f963624616249cebeb8994264ad6ba89c53876119d34f926548fbaacc1ea41488a5a78ed99d560eb42380d02c8c876f2493f6c";

        Pool.BitcoinBlock memory block6 = createTestBlock(
            blockHash6,     // block hash
            blockHash5,     // previousBlockHash
            merkleRoot6,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block6.header.blockHash, account6);
        pool.submitBlock(block6, tx6, proof6, account6);
        createAndSetChainTip(blockHash6, blockHash5, tx3, bits, outputAddress, blockReward);

        // Block 7
        bytes32 blockHash7 = hex"0000000000000000000079495db7e2b7c3bddd1beb9b2e4e70b16991c24863f3";
        bytes32 merkleRoot7 = hex"6b8f1f71602d7a21a40c68d3f1d7c46eba2fe8ca926968506f138edcdcc5d22e";
        bytes32 tx7 = hex"7cf13a87bc52cdcb988627565d886a9a50f78c99fcbac645045e5c84220d7006";
        bytes memory proof7 = hex"a32de63bbc1fafc6219935c41239795b7e2df0a4eb6dcf8debb2639a3323dc93330f051dfa2d646e992a847f0a67751399cb01c209ee4e165ca399dee060f9473a8ae680899b57abbf1513aec2fcde4b91d8a5bb519c1c81e4e026632a6cd6cf0b14e8c8f052a88d60e9ecaa9c4e0db318bb95dd17c8d6ae19d75f2211b9b2fd3ecd82392c27b025c20fc517e3af1f1ad092efabffb94c8af9ebcbc973c7bf621793f4154bb901b3257f49c8f9176c72c753c7323dbfccd75c45c54bab4c2aa7a6c1e4204e8b3399d3a7bc2a32fe8501c60ac4f7550b971cb1b36acafc28fa9af90c2ed8cc1dd61f148b841e97ff8383e59fa2ac921b48315e8dee1bca6cc2f31589bcacde9aefd085037b2ee448597cb80e91bbdae681032da6fff4215b465f39a49b77dd5a4c02c639d749c8294e763d21dd20b11c7480d5cc82d47b67f3dd739eb1230377c808e07a00bab3a233ba90faa10e7f1b1f515fc8ee019b952f96b75edcd51437ebea33562c2f35f963624616249cebeb8994264ad6ba89c53876119d34f926548fbaacc1ea41488a5a78ed99d560eb42380d02c8c876f2493f6c";

        Pool.BitcoinBlock memory block7 = createTestBlock(
            blockHash7,     // block hash
            blockHash6,     // previousBlockHash
            merkleRoot7,    // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        pool.submitHash(block7.header.blockHash, account7);
        pool.submitBlock(block7, tx7, proof7, account7);
        createAndSetChainTip(blockHash7, blockHash6, tx7, bits, outputAddress, blockReward);
        vm.stopPrank();

        // Expect 1000 to be distributed to all addresses
        pool.distributeRewards(block1.header.blockHash);

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

    function test_distributeRewardWrongOutputAddress() public {
        vm.startPrank(oracleAddress);

        // initial block
        // height 841770
        bytes32 rootPrev = hex"00000000000000000001ac358a5f02db047627db4fb8e8fc39de280a6eeab6d8";
        bytes32 rootTip = hex"000000000000000000019b664a54f77632ab0c2d033587336879771f4262d1d2";

        // set initial chain tips
        createAndSetChainTip(
            rootPrev,// block hash
            0,  // prev block hash
            0,  // merkle root
            0,  // bits
            0,  // outputAddress
            0   // blockReward
        );
        createAndSetChainTip(rootTip, rootPrev, 0, 0, 0, 0);

        // create block params
        bytes32 outputAddress = bytes32(0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        uint256 blockReward = (50000);
        uint32 bits = 0x1b0404cb;

        bytes32 blockHash = hex"00000000000000000000d0f43abc93a2b94165b2b807e8df9606e897f424ed43";
        bytes32 merkleRoot = hex"56da6fe71821c3a69d9b5ae084ec2cc3358eaceff972236fab56cf07bf8a4843";
        bytes32 txHash = hex"d8a00f2f7ec48d002516a5685add770e4e4f41b56a5e61e31f52ff037798115a";
        bytes memory proof = hex"76251b09c0f495a31496c307df5abc70657859a4aeb6cc07c955c70245df95391c225cb01453a9e38005d0f929de6341c8288a1de971b1080be1c330951223bbe98c8627a01a9596ec39a661d120c104596b9eac90b5592c884f54327d87aaacacb83ae1da9126fa2ffee73947e0b1ff38b9647c3240c11cf5439133b240281a224d23b35c1778d9f07ba6772f92e6222c19a82740fbf8477623130070a63f636a9a685a9597ef9f117210cddb56b8f292091476994f5731b8bd0620b7359fd5ffd9571ac2a6a54f78e2047edaad3800e4a410fb9ee22cf8f2ce7c6f7211d15f5f9f908274a883ca31efa9cc04994bad6fb021103491abc619964a1a13f4569ef7ca468e1d902e2210f9ba357dea05a2d4e47da709a646089d538a0f442f2408a206f3b0755631d4724bedb136cf897ef26481357e9781a4801f2bc4cfce4e0bc9afa069d4c25902fad95e45e0774d7ade7c613b588b747daba06ad0fe8e84e77ef1c646b03d80b4bc157f23b6bc10dcd076c0d12c1bad17072e047e5c8d9c64524d7dbb1f97eb209da882be6ca649ed01ff5fe4953f1906360b026577183cb7";

        // instantiate a block
        Pool.BitcoinBlock memory block1 = createTestBlock(
            blockHash,      // block hash
            rootTip,        // previousBlockHash
            merkleRoot,     // merkleRootHash
            bits,           // bits
            outputAddress,  // outputAddress
            blockReward     // blockReward
        );

        // create address
        address account = (0x70997970C51812dc3A010C7d01b50e0d17dc79C8);

        bytes32 incorrectAddress = bytes32(0x1434567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef);
        pool.submitHash(blockHash, account);
        pool.submitBlock(block1, txHash, proof, account);
        createAndSetChainTip(blockHash, rootTip, txHash, bits, incorrectAddress, blockReward);
        vm.expectRevert("Block's output address does not match quarry peg in address");
        pool.distributeRewards(block1.header.blockHash);
        vm.startPrank(oracleAddress);
    }
}
