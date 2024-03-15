use ethers::middleware::SignerMiddleware;
use ethers::prelude::{Provider, U256};
use ethers::providers::Middleware;
use ethers::signers::{LocalWallet, Signer};
use eyre::Result;
use quarry_sdk::{bindings, Deployment, Env};
use std::sync::Arc;
use std::time::Duration;
use tokio;

pub async fn deploy_contracts(env: Env, write_deployment: bool) -> Result<()> {
    let mut deployment = Deployment::get(env);
    let provider = Provider::new_client(&deployment.eth_rpc_url.clone(), 15, 500).unwrap();

    let chain_id = provider.get_chainid().await?;
    let wallet = deployment
        .deployer_pkey
        .clone()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let oracle_wallet = deployment
        .oracle_pkey
        .clone()
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let provider = Arc::new(SignerMiddleware::new(
        provider.interval(Duration::from_millis(500)),
        wallet,
    ));

    // TODO: these contracts are not behind a proxy
    let qbtc = bindings::qbtc::QBTC::deploy(Arc::clone(&provider), ())
        .unwrap()
        .send()
        .await?;
    let pool = bindings::pool::Pool::deploy(Arc::clone(&provider), ())
        .unwrap()
        .send()
        .await?;
    let share = bindings::share::Share::deploy(Arc::clone(&provider), ())
        .unwrap()
        .send()
        .await?;

    qbtc.initialize("QuarryBTC".to_string(), "QBTC".to_string(), pool.address())
        .send()
        .await?
        .await
        .unwrap()
        .unwrap();

    pool.initialize(
        oracle_wallet.address(),
        deployment.peg_in_address.0,
        U256::from(deployment.ring_buffer_size),
    )
    .send()
    .await?
    .await
    .unwrap()
    .unwrap();

    share
        .initialize(
            "QuarryShares".to_string(),
            "QSHARE".to_string(),
            pool.address(),
        )
        .send()
        .await?
        .await
        .unwrap()
        .unwrap();

    println!("QBTC: {:?}", qbtc.address());
    println!("Pool: {:?}", pool.address());
    println!("Share: {:?}", share.address());

    deployment.qbtc = qbtc.address();
    deployment.pool = pool.address();
    deployment.share = share.address();

    if write_deployment {
        deployment.write();
    }

    Ok(())
}
