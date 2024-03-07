// Oracle service for interfacing with the SharesPool smart contract

use ethers::prelude::*;
use reqwest::Client;
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let endpoint = "https://yolo-holy-sheet.btc-testnet.quiknode.pro/920ff3a045338aaeebaec4a4ea5ee278ee4e737e/";
    let client = Client::new();

    let best_hash = get_best_hash(&client, endpoint).await.unwrap();
    let best_block = get_best_block(&client, endpoint, &best_hash).await.unwrap();
    println!("Best Hash: {:?}", best_hash);
    println!("Best Block: {:?}", best_block);

    // set oracle parameters
    /* let oracle = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"; */
    let oracle_pk = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    /* let abi_path = "../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json"; */
    let contract_addr = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
    let chain = "http://localhost:8545";

    send_best_block(
        &client,
        endpoint,
        /* oracle, */
        oracle_pk,
        /* abi_path, */
        contract_addr,
        chain,
    )
    .await
    .unwrap();
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

async fn send_best_block(
    client: &Client,
    endpoint: &str,
    /* oracle: &str, */
    oracle_pk: &str,
    /* abi_path: &str, */
    contract_addr: &str,
    chain: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load the chain
    let provider = Provider::<Http>::try_from(chain.to_string())?;
    let wallet = Wallet::from_str(oracle_pk)?.with_chain_id(provider.get_chainid().await?.as_u64());
    let client_arc = Arc::new(SignerMiddleware::new(provider, wallet));

    // Load the ABI
    /* let abi_str = std::fs::read_to_string(abi_path)?; */
    abigen!(
        SharesPoolContract,
        "../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json",
        event_derives(serde::Deserialize, serde::Serialize)
    );
    let contract = SharesPoolContract::new(contract_addr.parse::<Address>()?, client_arc.clone());

    // Get the best block information
    let best_hash = get_best_hash(client, endpoint).await?;
    let best_block = get_best_block(client, endpoint, &best_hash).await?;
    let previous_block_hash: [u8; 32] = H256::from_slice(&hex::decode(
        best_block["previousBlockHash"]
            .as_str()
            .unwrap()
            .trim_start_matches("0x"),
    )?)
    .into();
    let merkle_root_hash: [u8; 32] = H256::from_slice(&hex::decode(
        best_block["merkleRootHash"]
            .as_str()
            .unwrap()
            .trim_start_matches("0x"),
    )?)
    .into();
    let chain_tip = shares_pool_contract::ChainTip {
        previous_block_hash: previous_block_hash,
        merkle_root_hash: merkle_root_hash,
    };

    // Prepare the function call
    let result: String = contract.method("setChainTip", chain_tip)?.call().await?;
    println!("{}", result);
    Ok(())
}

/* async fn send_best_block( */
/* client: &Client, */
/* endpoint: &str, */
/* oracle: &str, */
/* oracle_pk: &str, */
/* abi_path: &str, */
/* contract_addr: &str, */
/* chain: &str, */
/* ) -> Result<(), web3::Error> { */
/* // Load the chain */
/* let transport = Http::new(chain).unwrap(); */
/* let web3 = Web3::new(transport); */

/* // Load the ABI */
/* let abi = std::fs::read(abi_path).expect("Failed to read ABI file"); */
/* let contract_abi = ethabi::Contract::load(&*abi).expect("Failed to parse ABI"); */

/* // Set up the contract */
/* let contract_address: Address = contract_addr.parse().unwrap(); */
/* let contract = Contract::new(web3.eth(), contract_address, contract_abi); */

/* // Get the best block information */
/* let best_hash = get_best_hash(client, endpoint).await.unwrap(); */
/* let best_block = get_best_block(client, endpoint, &best_hash).await.unwrap(); */
/* let previous_block_hash = H256::from_slice( */
/* &hex::decode( */
/* best_block["previousBlockHash"] */
/* .as_str() */
/* .unwrap() */
/* .trim_start_matches("0x"), */
/* ) */
/* .unwrap(), */
/* ); */
/* let merkle_root_hash = H256::from_slice( */
/* &hex::decode( */
/* best_block["merkleRootHash"] */
/* .as_str() */
/* .unwrap() */
/* .trim_start_matches("0x"), */
/* ) */
/* .unwrap(), */
/* ); */

/* // Prepare the function call */
/* let function_name = "setChainTip"; */
/* let params = vec![ */
/* Token::Bytes(previous_block_hash.to_fixed_bytes().to_vec()), */
/* Token::Bytes(merkle_root_hash.to_fixed_bytes().to_vec()), */
/* ]; */

/* // Get the nonce for the transaction */
/* let nonce = web3 */
/* .eth() */
/* .transaction_count(oracle.parse().unwrap(), None) */
/* .await?; */

/* // Prepare the transaction options */
/* let options = Options { */
/* gas: Some(1_000_000.into()), */
/* gas_price: Some(10_000_000_000u64.into()), */
/* nonce: Some(nonce), */
/* ..Default::default() */
/* }; */

/* // Prepare the transaction data */
/* let tx_data = contract */
/* .abi() */
/* .function(function_name) */
/* .unwrap() */
/* .encode_input(&params) */
/* .unwrap(); */

/* // Create TransactionParameters */
/* let tx_parameters = TransactionParameters { */
/* to: Some(contract_address), */
/* data: Bytes(tx_data), */
/* gas: options.gas.unwrap(), */
/* gas_price: Some(options.gas_price.unwrap()), */
/* value: options.value.unwrap_or_else(U256::zero), */
/* nonce: options.nonce, */
/* ..Default::default() */
/* }; */

/* // Sign the transaction */
/* let secret_key = SecretKey::from_slice(&hex::decode(oracle_pk).unwrap()).unwrap(); */
/* let signed_tx = web3 */
/* .accounts() */
/* .sign_transaction(tx_parameters, &secret_key) */
/* .await?; */

/* // Send the signed transaction */
/* let tx_hash = web3 */
/* .eth() */
/* .send_raw_transaction(signed_tx.raw_transaction) */
/* .await?; */

/* println!("Transaction hash: {:?}", tx_hash); */

/* Ok(()) */
/* } */
