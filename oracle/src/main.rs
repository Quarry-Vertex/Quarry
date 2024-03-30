// Oracle service for interfacing with the SharesPool smart contract

use bitcoin::base58;
use bitcoin::bech32;
use ethers::prelude::*;
use log;
use quarry_sdk::bindings::pool::{BitcoinBlock, BlockHeader, Pool};
use quarry_sdk::{Deployment, Env};
use simplelog::*;
use std::fs::File;
use std::sync::Arc;
use std::time::Duration;

mod quicknode_funcs;

#[tokio::main]
async fn main() {
    // Setup logging
    let log_file = File::create("oracle.log").unwrap();
    WriteLogger::init(LevelFilter::Info, Config::default(), log_file).unwrap();

    // Smart Contract Setup
    let deployment = Deployment::get(Env::Local);
    let provider = Provider::new_client(&deployment.eth_rpc_url.clone(), 15, 500).unwrap();
    let chain_id = provider.get_chainid().await.unwrap();
    let wallet = deployment
        .oracle_pkey
        .clone()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let provider = Arc::new(SignerMiddleware::new(
        provider.interval(Duration::from_millis(500)),
        wallet,
    ));

    let pool = Pool::new(deployment.pool, provider.clone());

    let mut previous_merkle: [u8; 32] = [0; 32];

    // keep track of previous hashes on errors (for testing purposes)
    // let mut prev_addresses: Vec<String> = vec![];

    loop {
        let client = reqwest::Client::new();

        let best_hash = quicknode_funcs::get_best_hash(&client, &deployment.btc_rpc_url)
            .await
            .unwrap();

        let best_block =
            quicknode_funcs::get_best_block(&client, &deployment.btc_rpc_url, &best_hash)
                .await
                .unwrap();

        let block_hash: [u8; 32] = H256::from_slice(
            &hex::decode(
                best_block["header"]["blockHash"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("0x"),
            )
            .unwrap(),
        )
        .into();

        let previous_block_hash: [u8; 32] = H256::from_slice(
            &hex::decode(
                best_block["header"]["previousBlockHash"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("0x"),
            )
            .unwrap(),
        )
        .into();

        let merkle_root_hash: [u8; 32] = H256::from_slice(
            &hex::decode(
                best_block["header"]["merkleRootHash"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("0x"),
            )
            .unwrap(),
        )
        .into();

        let bits_hex_str = best_block["header"]["bits"]
            .as_str()
            .unwrap()
            .trim_start_matches("0x");
        let bits_bytes = hex::decode(bits_hex_str).unwrap();
        let bits = u32::from_be_bytes(bits_bytes.try_into().unwrap());

        // Assuming best_block["address"] is a String containing the Bitcoin address
        let address_str = best_block["address"]
            .as_str()
            .unwrap()
            .trim_start_matches("0x");

        // add new address to list
        // if !prev_addresses.contains(&address_str.to_string()) {
        // prev_addresses.push(address_str.to_string());
        // }
        // Decode the address based on it's encoding format
        // only two choices so expect and error if that isn't the case
        let mut address_bytes = match &address_str[0..3] {
            // bech32
            "tb1" | "bc1" => match bech32::decode(address_str) {
                Ok(v) => v.1,
                Err(e) => panic!("Cannot bech32 parse => {:?}\n{:?}", address_str, e),
            },
            // base58
            _ => match base58::decode(address_str) {
                Ok(v) => v,
                Err(e) => panic!("Cannot base58 parse => {:?}\n{:?}", address_str, e),
            },
        };

        // Pad or truncate the byte array to fit into 32 bytes
        address_bytes.resize(32, 0);

        // Convert to a fixed-size array
        let output_address: [u8; 32] = address_bytes.try_into().unwrap();

        // unwrap as u64
        let value_u64 = best_block["value"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        // convert to solidity U256
        let block_reward = U256::from(value_u64);

        // instatiate block header solidity struct
        let header = BlockHeader {
            block_hash,
            previous_block_hash,
            merkle_root_hash,
            bits,
        };

        // instatiate block solidity struct
        let chain_tip = BitcoinBlock {
            header,
            output_address,
            block_reward,
        };

        // new block found
        if previous_merkle != merkle_root_hash {
            log::info!("Best Hash: {:?}", best_hash);
            log::info!("Best Block: {:?}", best_block);
            let tx = pool.set_chain_tip(chain_tip);
            let tx = match tx.send().await {
                Ok(v) => v,
                Err(e) => {
                    panic!("Failed to send transaction to pool\n{:?}", e);
                }
            };
            let receipt = tx.await.unwrap();
            log::info!("Set chain tip: {:?}", receipt);
            previous_merkle = merkle_root_hash;
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
