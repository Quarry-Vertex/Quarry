use ethers::prelude::*;
use quarry_sdk::bindings::qsat_bridge::{PegOutQSATEventFilter, QSATBridge};
use quarry_sdk::{Deployment, Env};
use std::sync::Arc;
use std::time::Duration;
use serde_json::Value;

mod peg_in;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    // random mining address used for testing
    let peg_in_addr = "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj";
    let endpoint =
        "https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/";
    // let deployment = Deployment::get(Env::Local);
    // let provider = Provider::new_client(&deployment.eth_rpc_url.clone(), 15, 500).unwrap();
    // let chain_id = provider.get_chainid().await.unwrap();
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
    // let qsat_bridge = QSATBridge::new(deployment.qsat_bridge, provider.clone());
    // just implement it in storage
    // every burn request push to an array in a smart contract
    // expose methods to read from array
    // write script that burns latest burn request
    let mut latest_tx_id = "".to_string(); // get from local
    // loop {
        let tx_json: Vec<Value> = peg_in::get_peg_transactions(&client, peg_in_addr).await.unwrap();
        // println!("{:?}", tx_json);
        let tx_id = tx_json[0]["txid"].as_str().unwrap().to_string();
        // call this val tx_unsent
        let peg_in_txs = peg_in::parse_transactions(
            &client,
            latest_tx_id.as_str(),
            &tx_json.clone(),
            endpoint,
            peg_in_addr,
        )
        .await;
        latest_tx_id = tx_id;
        println!("{:?}", peg_in_txs);
        // call SC interopt function on "tx_unsent"
        // for tx in peg_in_txs.unwrap().into_iter() {
            // peg_in::peg_in_sc(&tx, &qsat_bridge).await;
        // }
        // tokio::time::sleep(Duration::from_secs(5)).await;
    // }
}
