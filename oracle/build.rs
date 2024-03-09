// build.rs
use ethers::contract::Abigen;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if env::var("CARGO_FEATURE_REGENERATE_ABI").is_ok() {
        let abi_source = "../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json";
        let out_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/bindings/shares_pool.rs");

        // Generate the ABI bindings
        Abigen::new("SharesPool", abi_source)?.generate()?.write_to_file(out_file)?;
    }

    Ok(())
}

