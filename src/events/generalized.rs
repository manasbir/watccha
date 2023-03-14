use std::fs;
use ethers::{types::{U256, Transaction, H160}, providers::{Provider, Http}};
use toml::Value;

pub(crate) async fn from(tx: Transaction, monitor_address: H160) -> Result<String, bool> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let toml: Value = toml::from_str(&toml_str).unwrap();
    let monitor_address_str = monitor_address.to_string();

    let provider = Provider::try_from(toml.get("general.rpc_url").unwrap().as_str().unwrap()).unwrap();

    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
        
    } return Err(false);
}