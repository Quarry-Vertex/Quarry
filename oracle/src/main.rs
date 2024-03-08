// Oracle service for interfacing with the SharesPool smart contract

use ethers::prelude::*;
use reqwest::Client;
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // let endpoint = "https://yolo-holy-sheet.btc-testnet.quiknode.pro/920ff3a045338aaeebaec4a4ea5ee278ee4e737e/";
    // let client = Client::new();

    // let best_hash = get_best_hash(&client, endpoint).await.unwrap();
    // let best_block = get_best_block(&client, endpoint, &best_hash).await.unwrap();
    // println!("Best Hash: {:?}", best_hash);
    // println!("Best Block: {:?}", best_block);

    // set oracle parameters
    let oracle_pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let contract_addr = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    let chain = "http://localhost:8545";

    send_best_block(
        // &client,
        // endpoint,
        oracle_pk,
        contract_addr,
        chain,
    )
    .await
    .unwrap();
}

async fn send_best_block(
    // client: &Client,
    // endpoint: &str,
    oracle_pk: &str,
    contract_addr: &str,
    chain: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load the chain
    let provider = Provider::<Http>::try_from(chain.to_string())?;
    let wallet = Wallet::from_str(oracle_pk)?.with_chain_id(provider.get_chainid().await?.as_u64());
    // test to see if it is correct oracle address
    let derived_address = wallet.clone().address();
    println!("Derived Address: {:?}", derived_address);
    let client_arc = Arc::new(SignerMiddleware::new(provider, wallet));

    // Load the ABI
    abigen!(
        SharesPoolContract,
        "../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );
    let contract = SharesPoolContract::new(contract_addr.parse::<H160>()?, client_arc.clone());

    // let test = contract.get_one_hundred().call().await?;
    let test_result = contract.get_one_hundred().call().await;
    match test_result {
        Ok(value) => {
            println!("{:?}", value);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error calling get_one_hundred: {:?}", e);
            Err(e.into())
        }
    }
    // Get the best block information
    /*
     * let best_hash = get_best_hash(client, endpoint).await?;
     * let best_block = get_best_block(client, endpoint, &best_hash).await?;
     * let previous_block_hash: [u8; 32] = H256::from_slice(&hex::decode(
     *     best_block["previousBlockHash"]
     *         .as_str()
     *         .unwrap()
     *         .trim_start_matches("0x"),
     * )?)
     * .into();
     * let merkle_root_hash: [u8; 32] = H256::from_slice(&hex::decode(
     *     best_block["merkleRootHash"]
     *         .as_str()
     *         .unwrap()
     *         .trim_start_matches("0x"),
     * )?)
     * .into();
     * let chain_tip = shares_pool_contract::ChainTip {
     *     previous_block_hash: previous_block_hash,
     *     merkle_root_hash: merkle_root_hash,
     * };
     */

    // Prepare the function call
    // let result: String = contract.method("setChainTip", chain_tip)?.call().await?;
}

async fn get_best_hash(client: &Client, endpoint: &str) -> Result<String, reqwest::Error> {
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
    client: &Client,
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
