use ethers::prelude::*;
use quarry_sdk::bindings::qsat_bridge::QSATBridge;

// sends 'amount' btc to wallet 'address'
// amount will be passed into as SAT
pub async fn peg_out(address: &str, amount: f64) {}

pub async fn pollBurnRequests(
    qsat_bridge: &QSATBridge<SignerMiddleware<Provider<RetryClient<Http>>, LocalWallet>>,
) {
    let len_call = qsat_bridge.get_total_burn_requests();
    let burn_len = len_call.send().await.unwrap();

    // use length to iterate through burn requests and send btc
    // needs to be multisig
    // for i in range 0..burn_len {
    //    handle btc transaction
    // }

}
