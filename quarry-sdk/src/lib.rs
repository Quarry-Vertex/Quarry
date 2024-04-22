pub mod bindings;
use ethers::contract::{ContractFilter, MultiAbigen, SelectContracts};
use ethers::types::{H160, H256};
use serde::{Deserialize, Serialize};
use std::env;
use std::env::current_dir;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Deployment {
    // move to secrets / env vars in prod
    // for now just do everything in one json
    pub deployer_pkey: String,
    pub oracle_pkey: String,
    // contract addresses
    #[serde(default)]
    pub qsat: H160,
    #[serde(default)]
    pub qsat_bridge: H160,
    #[serde(default)]
    pub pool: H160,
    #[serde(default)]
    pub share: H160,

    // random constants
    pub peg_in_address: H256,
    pub ring_buffer_size: u64,
    pub eth_rpc_url: String,
    pub btc_rpc_url: String,

    #[serde(skip_serializing, skip_deserializing)]
    pub env: Option<Env>,
}

impl Deployment {
    fn path(env: &Env) -> String {
        let d = current_dir().unwrap();
        let mut d = d.as_path();
        // get parent until we get to folder "Quarry" or "stratum"
        while d.file_name().unwrap() != "stratum" && d.file_name().unwrap() != "Quarry" {
            d = d.parent().unwrap();
        }
        let quarry = d.to_string_lossy().to_string();
        format!("{}/deployment.{}", quarry, env.name())
    }
    pub fn get(env: Env) -> Self {
        let path = Deployment::path(&env);
        let contents = std::fs::read_to_string(path).unwrap();
        let mut deployment: Deployment = serde_json::from_str(&contents).unwrap();
        deployment.env = Some(env);
        deployment
    }

    pub fn write(&self) {
        let path = Deployment::path(self.env.as_ref().unwrap());
        let contents = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(path, contents).unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Env {
    Local,
}

impl Env {
    pub fn name(&self) -> String {
        match self {
            Env::Local => "local".to_string(),
        }
    }
}

pub fn regenerate_bindings() {
    let current_dir = env::current_dir().unwrap();
    let root = current_dir.parent().unwrap();
    println!("root: {}", root.to_string_lossy());
    let abigen =
        MultiAbigen::from_json_files(format!("{}/contracts", root.to_string_lossy())).unwrap();

    let contracts = SelectContracts::default()
        .add_name("QSAT")
        .add_name("QSATBridge")
        .add_name("Share")
        .add_name("Pool");

    let abigen = abigen.with_filter(ContractFilter::Select(contracts));

    abigen
        .build()
        .unwrap()
        .write_to_module(
            format!("{}/quarry-sdk/src/bindings", root.to_string_lossy()),
            false,
        )
        .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::regenerate_bindings;

    #[test]
    fn regen_bindings() {
        regenerate_bindings()
    }
}
