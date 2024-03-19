// Oracle service for interfacing with the SharesPool smart contract

use base58::FromBase58;
use ethers::prelude::*;
use quarry_sdk::bindings::pool::{BlockHeader, BitcoinBlock, Pool};
use quarry_sdk::{Deployment, Env};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() {
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

    loop {
        let client = reqwest::Client::new();
        let best_hash = get_best_hash(&client, &deployment.btc_rpc_url)
            .await
            .unwrap();
        let best_block = get_best_block(&client, &deployment.btc_rpc_url, &best_hash)
            .await
            .unwrap();
        // println!("Best Hash: {:?}", best_hash);
        // println!("Best Block: {:?}", best_block);

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
        let address_str = best_block["address"].as_str().unwrap().trim_start_matches("0x");

        // Decode the address from Base58Check format
        let mut address_bytes = address_str.from_base58().unwrap();

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

        // create params for Pools function
        let header = BlockHeader {
            previous_block_hash,
            merkle_root_hash,
            bits,
        };
        let chain_tip = BitcoinBlock {
            header,
            output_address,
            block_reward,
        };

        println!("Best Hash: {:?}", best_hash);
        if previous_merkle != merkle_root_hash {
            println!("Best Block: {:?}", best_block);
            println!("{:?}", chain_tip);
            let tx = pool.set_chain_tip(chain_tip);
            let tx = tx.send().await.unwrap();
            let receipt = tx.await.unwrap();
            println!("Set chain tip: {:?}", receipt);
            previous_merkle = merkle_root_hash;
        } else {
            println!("No new block");
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

async fn get_best_hash(client: &reqwest::Client, endpoint: &str) -> Result<String, reqwest::Error> {
    let res: Value = client
        .post(endpoint)
        .json(&json!({ "method": "getbestblockhash" }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res["result"].as_str().unwrap().to_string())
}

async fn get_best_block(
    client: &reqwest::Client,
    endpoint: &str,
    hash: &str,
) -> Result<Value, reqwest::Error> {
    // get block data
    let block_res: Value = client
        .post(endpoint)
        .json(&json!({
            "method": "getblock",
            "params": [hash]
        }))
        .send()
        .await?
        .json()
        .await?;

    let prev_hash = format!("0x{}", block_res["result"]["previousblockhash"].as_str().unwrap());
    let merkle_root = format!("0x{}", block_res["result"]["merkleroot"].as_str().unwrap());
    let bits = format!("0x{}", block_res["result"]["bits"].as_str().unwrap());
    // the coinbase transaction is the first listed transaction
    let coin_base_tx = format!("{}", block_res["result"]["tx"][0].as_str().unwrap());

    // find transaction information for coinbase transaction
    let tx_res: Value = client
        .post(endpoint)
        .json(&json!({
            "method": "getrawtransaction",
            "params": [coin_base_tx, 1]     // pass '1' for verbosity
        }))
        .send()
        .await?
        .json()
        .await?;

    let address = format!("0x{}", tx_res["result"]["vout"][0]["scriptPubKey"]["address"].as_str().unwrap());
    let btc_value = tx_res["result"]["vout"][0]["value"].as_f64().unwrap();
    // convert BTC -> SAT
    let value = format!("{}", btc_value * 100_000_000.0);

    // return serialized data for SC
    Ok(json!({
        "header": {
            "previousBlockHash": prev_hash,
            "merkleRootHash": merkle_root,
            "bits": bits,
        },
        "address": address,
        "value": value,
    }))
}
