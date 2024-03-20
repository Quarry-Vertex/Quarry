use serde_json::{json, Value};

pub async fn get_best_hash(client: &reqwest::Client, endpoint: &str) -> Result<String, reqwest::Error> {
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
            "blockHash": hash,
            "previousBlockHash": prev_hash,
            "merkleRootHash": merkle_root,
            "bits": bits,
        },
        "address": address,
        "value": value,
    }))
}
