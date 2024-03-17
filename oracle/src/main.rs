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
    //let provider = Provider::new_client(&deployment.eth_rpc_url.clone(), 15, 500).unwrap();
    //let chain_id = provider.get_chainid().await.unwrap();
    // let wallet = deployment
        // .oracle_pkey
        // .clone()
        // .parse::<LocalWallet>()
        // .unwrap()
        // .with_chain_id(chain_id.as_u64());

    // let provider = Arc::new(SignerMiddleware::new(
        // provider.interval(Duration::from_millis(500)),
        // wallet,
    // ));

    // let pool = Pool::new(deployment.pool, provider.clone());

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

        /*
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
        */
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
        "previousBlockHash": prev_hash,
        "merkleRootHash": merkle_root,
        "address": address,
        "value": value,
        "bits": bits,
    }))
}
