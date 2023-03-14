use std::fs;
use ethers::{types::{U256, Transaction, H160}, providers::{Provider, Http}};
use crate::bindings::erc20 as ERC20;
use toml::Value;

async fn from(tx: Transaction, monitor_address: H160) -> Result<String, bool> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let toml: Value = toml::from_str(&toml_str).unwrap();
    let monitor_address_str = monitor_address.to_string();

    let provider = Provider::try_from(toml.get("general.rpc_url").unwrap().as_str().unwrap()).unwrap();

    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
        if tx.input.to_string().to_lowercase().contains(&"0xa9059cbb".to_lowercase()) {
            println!("{} transferred {} {:?} to you and now transfering out....", 
                tx.from,
                &tx.input.to_string()[74..138].parse::<U256>().unwrap() / U256::from(10u64.pow(18)),
                ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await?
            );
            return Ok(String::from("test"));
        } return Err(false);
    } return Err(false);
}

async fn to(tx: Transaction, monitor_address: H160) -> Result<String, bool> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let toml: Value = toml::from_str(&toml_str).unwrap();
    let monitor_address_str = monitor_address.to_string();

    let provider = Provider::try_from(toml.get("general.rpc_url").unwrap().as_str().unwrap()).unwrap();

    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
        if tx.input.to_string().to_lowercase().contains(&"0xa9059cbb".to_lowercase()) {
            println!("{} transferred {} {:?} to you and now transfering out....", 
                tx.from,
                &tx.input.to_string()[74..138].parse::<U256>().unwrap() / U256::from(10u64.pow(18)),
                ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await?
            );
            return Ok(String::from("test"));
        } return Err(false);
    }
        return Err(false);
}