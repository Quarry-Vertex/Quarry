// Oracle service for interfacing with the SharesPool smart contract

use ethers::prelude::*;
use quarry_sdk::bindings::pool::{ChainTip, Pool};
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

    loop {
        let client = reqwest::Client::new();
        let best_hash = get_best_hash(&client, &deployment.btc_rpc_url)
            .await
            .unwrap();
        let best_block = get_best_block(&client, &deployment.btc_rpc_url, &best_hash)
            .await
            .unwrap();
        println!("Best Hash: {:?}", best_hash);
        println!("Best Block: {:?}", best_block);

        let previous_block_hash: [u8; 32] = H256::from_slice(
            &hex::decode(
                best_block["previousBlockHash"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("0x"),
            )
            .unwrap(),
        )
        .into();
        let merkle_root_hash: [u8; 32] = H256::from_slice(
            &hex::decode(
                best_block["merkleRootHash"]
                    .as_str()
                    .unwrap()
                    .trim_start_matches("0x"),
            )
            .unwrap(),
        )
        .into();
        let chain_tip = ChainTip {
            previous_block_hash,
            merkle_root_hash,
        };

        let tx = pool.set_chain_tip(chain_tip);
        let tx = tx.send().await.unwrap();
        let receipt = tx.await.unwrap();
        println!("Set chain tip: {:?}", receipt);

        tokio::time::sleep(Duration::from_secs(60)).await;
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
    let res: Value = client
        .post(endpoint)
        .json(&json!({
            "method": "getblock",
            "params": [hash]
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(json!({
        "previousBlockHash": format!("0x{}", res["result"]["previousblockhash"].as_str().unwrap()),
        "merkleRootHash": format!("0x{}", res["result"]["merkleroot"].as_str().unwrap()),
    }))
}
