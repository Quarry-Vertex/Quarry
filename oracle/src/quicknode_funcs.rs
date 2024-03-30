use serde_json::{json, Value};

pub async fn get_best_hash(
    client: &reqwest::Client,
    endpoint: &str,
) -> Result<String, reqwest::Error> {
    let res: Value = client
        .post(endpoint)
        .json(&json!({ "method": "getbestblockhash" }))
        .send()
        .await?
        .json()
        .await?;

    Ok(res["result"].as_str().unwrap().to_string())
}

pub async fn get_best_block(
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

    let prev_hash = format!(
        "0x{}",
        block_res["result"]["previousblockhash"].as_str().unwrap()
    );
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

    let vout_array = tx_res["result"]["vout"].as_array().unwrap();

    let mut address = String::new();
    let mut btc_value: Option<f64> = None;

    // some outputs are empty and will panic the program
    // find the first transaction output with an address
    for vout_entry in vout_array {
        if let Some(addr) = vout_entry["scriptPubKey"]["address"].as_str() {
            address = format!("0x{}", addr);
            btc_value = Some(vout_entry["value"].as_f64().unwrap());
            break; // Exit the loop once the first address is found
        }
    }
    if btc_value == None {
        panic!("no value set");
    }
    // convert BTC -> SAT
    let value = format!("{}", (btc_value.unwrap() * (100_000_000 as f64)) as u64);

    // return serialized data for SC
    Ok(json!({
        "header": {
            "blockHash": hash,
            "previousBlockHash": prev_hash,
            "merkleRootHash": merkle_root,
            "bits": bits,
        },
        "address": address,
        "value": value,
    }))
}
