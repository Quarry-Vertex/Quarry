mod bindings;

use ethers::contract::{ContractFilter, MultiAbigen, SelectContracts};
use std::env;

pub fn regenerate_bindings() {
    let current_dir = env::current_dir().unwrap();
    let root = current_dir.parent().unwrap();
    println!("root: {}", root.to_string_lossy());
    let abigen =
        MultiAbigen::from_json_files(format!("{}/contracts", root.to_string_lossy()))
            .unwrap();

    let contracts = SelectContracts::default()
        .add_name("QBTC")
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
