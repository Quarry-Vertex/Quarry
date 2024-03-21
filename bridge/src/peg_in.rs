use eth_checksum::checksum;
use ethers::prelude::*;
use quarry_sdk::bindings::qsat_bridge::QSATBridge;
use serde_json::Value;
use std::str::FromStr;

// curl -s "https://blockstream.info/testnet/api/address/tb1qsgx55dp6gn53tsmyjjv4c2ye403hgxynxs0dnm/txs"
// get transactions for peg wallet
pub async fn get_peg_transactions(
    client: &reqwest::Client,
    address: &str,
) -> Result<Vec<Value>, reqwest::Error> {
    let endpoint = format!("https://blockstream.info/testnet/api/address/{}/txs", address);

    let tx_res: Value = client.get(&endpoint).send().await?.json().await?;

    let mut len = tx_res.clone().as_array().unwrap().len();
    if len > 5 {
        // 25 later
        len = 5;
    }
    let tx_vec: Vec<Value> = tx_res.clone().as_array().unwrap()[0..len].to_vec();

    Ok(tx_vec)
}

#[derive(Debug)]
pub struct PegTx {
    pub value: String,
    pub eth_address: H160,
}

pub async fn parse_transactions(
    latest_tx: &str,
    tx_vec: &Vec<Value>,
    peg_in: &str,
) -> Result<Vec<PegTx>, reqwest::Error> {
    let mut peg_txs = Vec::new();

    for tx in tx_vec {
        let tx_id = tx["txid"].as_str().unwrap();
        if tx_id == latest_tx {
            return Ok(peg_txs); // return up to latest
        }
        // run through every "vout" until you find OP_RETURN
        for v in tx["vout"].as_array().unwrap() {
            // ensure address is peg in and value is nonzero
            if let Some(address) = v["scriptpubkey_address"].as_str() {
                // convert value to a f64 representation of sats
                let value_f64 = v["value"].as_f64().unwrap() * 100_000_000.0;
                // esnure address is correct and positive value
                if address == peg_in &&  value_f64 >= 0.0 {
                    let asm = v["scriptpubkey_asm"].as_str().unwrap();
                    // make sure valid length
                    if asm.len() >= 66 {
                        // pull out OP_RETURN and ensure it's followed by OP_PUSHBYTES_20
                        if &asm[0..9] == "OP_RETURN" && &asm[10..25] == "OP_PUSHBYTES_20" {
                            // create address and ensure it's an eth address
                            let raw_op = &asm[26..66];
                            let eth_address = format!("0x{}", raw_op);
                            // try unwrapping as Address
                            if is_valid_eth_address(eth_address.as_str()) {
                                if let Ok(eth_address) = Address::from_str(eth_address.as_str()) {
                                    // populate transaction vector
                                    let value = format!("{}", value_f64);
                                    peg_txs.push(PegTx { value, eth_address });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(peg_txs)
}

fn is_valid_eth_address(address: &str) -> bool {
    if let Ok(parsed_address) = Address::from_str(address) {
        let checksummed_address = checksum(&format!("{:?}", parsed_address));
        return checksummed_address == address;
    }
    false
}

pub async fn peg_in_sc(
    tx: &PegTx,
    qsat_bridge: &QSATBridge<SignerMiddleware<Provider<RetryClient<Http>>, LocalWallet>>,
) {
    // convert PegTx values to Solidity types
    let amount: U256 = U256::from(tx.value.as_str().parse::<u64>().unwrap());
    let eth_address: Address = tx.eth_address;
    let sc_tx = qsat_bridge.peg_in_qsat(eth_address, amount);
    let sc_tx = sc_tx.send().await.unwrap();
    let receipt = sc_tx.await.unwrap();
    println!("Peg out: {:?}", receipt);
}

// pub async fn parse_transactions(
    // client: &reqwest::Client,
    // latest_tx: &str,
    // tx_vec: &Vec<Value>,
    // endpoint: &str,
    // peg_in: &str,
// ) -> Result<Vec<PegTx>, reqwest::Error> {
    // let mut peg_txs = Vec::new();

    // for tx in tx_vec {
        // let tx_id = tx["txid"].as_str().unwrap();
        // if tx_id == latest_tx {
            // break;
        // }
        // // let endpoint = format!("https://blockstream.info/testnet/api/tx/{}", tx_id);
        // // let mut tx_res: Value = client.get(&endpoint).send().await?.json().await?;
        // let mut tx_res: Value = client
            // .post(endpoint)
            // .json(&json!({
                // "method": "getrawtransaction",
                // "params": [tx_id, 1]     // pass '1' for verbosity
            // }))
            // .send()
            // .await?
            // .json()
            // .await?;

        // // using anvil account (9) for testing
        // tx_res["result"]["vout"][0]["scriptPubKey"]["asm"] = Value::String(
            // "OP_RETURN OP_PUSHBYTES_20 a0Ee7A142d267C1f36714E4a8F75612F20a79720".to_string(),
        // );

        // // run through every "vout" until you find OP_RETURN
        // for v in tx_res["result"]["vout"].as_array().unwrap() {
            // // ensure address is peg in and value is nonzero
            // if let Some(address) = v["scriptPubKey"]["address"].as_str() {
                // // convert value to a f64 representation of sats
                // let value_f64 = v["value"].as_f64().unwrap() * 100_000_000.0;
                // // esnure address is correct and positive value
                // if address == peg_in &&  value_f64 >= 0.0 {
                    // let asm = v["scriptPubKey"]["asm"].as_str().unwrap();
                    // // make sure valid length
                    // if asm.len() >= 66 {
                        // // pull out OP_RETURN and ensure it's followed by OP_PUSHBYTES_20
                        // if &asm[0..9] == "OP_RETURN" && &asm[10..25] == "OP_PUSHBYTES_20" {
                            // // create address and ensure it's an eth address
                            // let raw_op = &asm[26..66];
                            // let eth_address = format!("0x{}", raw_op);
                            // // try unwrapping as Address
                            // if is_valid_eth_address(eth_address.as_str()) {
                                // if let Ok(eth_address) = Address::from_str(eth_address.as_str()) {
                                    // // populate transaction vector
                                    // let value = format!("{}", value_f64);
                                    // peg_txs.push(PegTx { value, eth_address });
                                // }
                            // }
                        // }
                    // }
                // }
            // }
        // }
    // }

    // Ok(peg_txs)
// }
