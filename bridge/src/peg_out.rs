use ethers::prelude::*;
use quarry_sdk::bindings::qsat_bridge::{PegOutRequest, QSATBridge};

#[derive(Debug)]
pub struct RustPegOutRequest {
    btc_address: [u8; 32],
    amount: U256,
}

// sends 'amount' btc to wallet 'address'
// amount will be passed into as SAT
// pub async fn peg_out(address: &str, amount: f64) {}
pub async fn _handle_peg_outs(
    qsat_bridge: &QSATBridge<SignerMiddleware<Provider<RetryClient<Http>>, LocalWallet>>,
) -> Vec<RustPegOutRequest> {
    let mut requests: Vec<RustPegOutRequest> = vec![];

    let burn_len: U256 = qsat_bridge
        .get_total_peg_out_requests()
        .call()
        .await
        .unwrap();

    for i in 0..burn_len.as_u64() {
        let peg_out_request: PegOutRequest = qsat_bridge
            .get_peg_out_request(i.into())
            .call()
            .await
            .unwrap();

        let btc_address = peg_out_request.btc_address;
        let amount = peg_out_request.amount;
        let request: RustPegOutRequest = RustPegOutRequest {
            btc_address,
            amount,
        };
        requests.push(request);

        // Handle the BTC transaction for peg-out

        println!("Processed PegOutRequest: {:?}", peg_out_request);
    }
    return requests;
}

pub fn _print_all_pegs(pegs: &Vec<RustPegOutRequest>) {
    for pr in pegs {
        println!("{:?}", pr);
    }
}
