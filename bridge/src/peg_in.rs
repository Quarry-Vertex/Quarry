use serde_json::{json, Value};

// get transactions for peg wallet
pub async fn get_peg_transactions(
    client: &reqwest::Client,
    address: &str,
) -> Result<Vec<Value>, reqwest::Error> {
    let endpoint = format!("https://blockstream.info/api/address/{}/txs", address);

    let tx_res: Value = client
        .get(&endpoint)
        .send()
        .await?
        .json()
        .await?;

    let mut len = tx_res.clone().as_array().unwrap().len();
    if len > 25 {
        len = 25;
    }
    let tx_vec: Vec<Value> = tx_res.clone().as_array().unwrap()[0..len].to_vec();

    Ok(tx_vec)
}

#[derive(Debug)]
pub struct PegTx {
    value: String,
    eth_address: String,
}

/*
curl https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/ \
    -X POST \
    -H "Content-Type: application/json" \
    --data '{"method": "getrawtransaction", "params": ["c7cfd4ad3662e062c5f70c933e5bd435bdd13e2271caef68b5c41ac981d12025", 1]}'
*/

pub async fn parse_transactions(
    client: &reqwest::Client,
    latest_tx: &str,
    tx_vec: &Vec<Value>,
    endpoint: &str,
    peg_in: &str,
) -> Result<Vec<PegTx>, reqwest::Error> {
    let mut peg_txs = Vec::new();

    for tx in tx_vec {
        let tx_id = tx["txid"].as_str().unwrap(); 
        if tx_id == latest_tx {
            break
        }
        let tx_res: Value = client
            .post(endpoint)
            .json(&json!({
                "method": "getrawtransaction",
                "params": [tx_id, 1]     // pass '1' for verbosity
            }))
            .send()
            .await?
            .json()
            .await?;

        // find random eth address for this
        // tx_res["result"]["vout"][0]["scriptPubKey"]["asm"] = "OP_RETURN OP_PUSHBYTES_20 ";

        // run through every "vout" until you find OP_RETURN
        for v in tx_res["result"]["vout"].as_array().unwrap() {
            // ensure address is peg in and value is nonzero
            if v["address"] == peg_in && v["value"].as_f64().unwrap() > 0.0 {
                let asm = v["scriptPubKey"]["asm"].as_str().unwrap();
                if asm.len() >= 25 {
                    // pull out OP_RETURN and ensure it's followed by OP_PUSHBYTES_20
                    if &asm[0..9] == "OP_RETURN" && &asm[10..25] == "OP_PUSHBYTES_20" {
                        // create address and ensure it's an eth address
                        let raw_op = &asm[26..66];
                        let eth_address = format!("0x{}", raw_op);
                        if is_valid_ethereum_address(eth_address.as_str()) {
                            let btc_value = v["value"].as_f64().unwrap();
                            // convert BTC -> SAT
                            let value = format!("{}", btc_value * 100_000_000.0);
                            peg_txs.push(PegTx {
                                value,
                                eth_address,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(peg_txs)
}

fn is_valid_ethereum_address(address: &str) -> bool {
    let re = regex::Regex::new("^0x[0-9a-fA-F]{40}$").unwrap();

    if !re.is_match(address) {
        // Check if it has the correct length and starts with 0x
        return false;
    }

    // If it has upper and lower case characters, it must pass checksum verification
    let re_upper = regex::Regex::new("^[0-9a-fA-F]{40}$").unwrap();
    let re_lower = regex::Regex::new("^[0-9A-F]{40}$").unwrap();
    if re_upper.is_match(address) || re_lower.is_match(address) {
        return true;
    }

    // Failed checksum verification
    false
}

/*
 *     function pegInQSAT(address ethAddress, uint256 amount) public onlyOracleOrSharesPool {
 *         require(qsat.balanceOf(address(this)) >= amount,
 *             "Bridge contract has insufficient QSAT");
 * 
 *         qsat.transfer(ethAddress, amount);
 * 
 *         emit PegInQSATEvent(ethAddress, amount);
 *     }
 */

// (TODO) make function to call pegin functions in SC
pub async fn _peg_in(tx: &PegTx) {
    let amount = tx.value.as_str();
    let address = tx.eth_address.as_str();
}
