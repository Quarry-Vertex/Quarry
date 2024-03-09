// build.rs
use ethers::contract::Abigen;
use std::env;
use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("src/bindings/shares_pool.rs");

    if env::var("CARGO_FEATURE_REGENERATE_ABI").is_ok() {
        let abi_source = "../contracts/rewardContracts/out/SharesPool.sol/SharesPoolAbi.json";

        // Remove the existing bindings if they exist
        if out_file.exists() {
            fs::remove_file(&out_file)?;
        }

        // Generate the ABI bindings
        Abigen::new("SharesPool", abi_source)?.generate()?.write_to_file(&out_file)?;
    } else {
        // Ensure a dummy file exists if the feature is not enabled
        if !out_file.exists() {
            fs::write(&out_file, "pub struct SharesPool;")?;
        }
    }

    Ok(())
}


