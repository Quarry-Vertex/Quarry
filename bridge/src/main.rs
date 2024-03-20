use ethers::prelude::*;
use quarry_sdk::bindings::qsat_bridge::{QSATBridge, PegOutQSATEventFilter};
use quarry_sdk::{Deployment, Env};
use std::sync::Arc;
use std::time::Duration;

mod peg_in;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let peg_in =  "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj";
    let endpoint =  "https://dark-icy-putty.btc.quiknode.pro/c312ce60f2c274142fe6c7c08cb3999c3ae354eb/";
    let mut latest_tx_id = "";
    //loop {
        let tx_json = peg_in::get_peg_transactions(&client, peg_in).await.unwrap();
        // call this val tx_unsent
        let peg_in_txs = peg_in::parse_transactions(&client, latest_tx_id, &tx_json, endpoint, "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj").await;
        let tx_id = tx_json[0]["txid"].as_str().unwrap(); 
        latest_tx_id = tx_id;
        // call SC interopt function on "tx_unsent"
        // for tx in peg_in_txs.into_iter() { peg_in(tx); }
        // tokio::time::sleep(Duration::from_secs(5)).await;
    //}
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
    let qsat_bridge = QSATBridge::new(deployment.qsat_bridge, provider.clone());
    let event = qsat_bridge.event::<PegOutQSATEventFilter>();
}
