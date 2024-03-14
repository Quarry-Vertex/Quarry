///`BitcoinBlock((uint32,bytes32,bytes32,uint32,uint32,uint32),bytes32,bytes8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct BitcoinBlock {
    pub header: BlockHeader,
    pub output_address: [u8; 32],
    pub block_reward: [u8; 8],
}
///`BlockHeader(uint32,bytes32,bytes32,uint32,uint32,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_block_hash: [u8; 32],
    pub merkle_root_hash: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
}
///`ChainTip(bytes32,bytes32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ChainTip {
    pub previous_block_hash: [u8; 32],
    pub merkle_root_hash: [u8; 32],
}
