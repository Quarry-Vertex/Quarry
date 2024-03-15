// quarry deployment scripts

use crate::deploy_contracts::deploy_contracts;
use quarry_sdk::Env;

mod deploy_contracts;

#[tokio::main]
async fn main() {
    // TODO: options to switch on env from cmd line args
    deploy_contracts(Env::Local, true).await.unwrap();
}
