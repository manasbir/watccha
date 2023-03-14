use std::fs;
use ethers::{types::{U256, Transaction, H160}, providers::{Provider, Http}};
use toml::Value;


pub(crate) async fn from(tx: Transaction, monitor_address: H160) -> Result<String, bool> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let toml: Value = toml::from_str(&toml_str).unwrap();
    let monitor_address_str = monitor_address.to_string();

    let provider = Provider::try_from(toml.get("general.rpc_url").unwrap().as_str().unwrap()).unwrap();

    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
        let body = reqwest::get(format!("https://sig.eth.samczsun.com/api/v1/signatures?function={}", &tx.input.to_string()[0..10])).await.unwrap();  
        let fn_name = body.json::<Value>().await.unwrap()["result"]["function"][&tx.input.to_string()[0..10]][0]["name"].as_str().unwrap().to_string();
        
        let res = reqwest::get(format!("https://api-goerli.etherscan.io/api?module=contract&action=getabi&address={:?}&apikey={}", &tx.to.unwrap(), toml.get("etherscan_key").unwrap().as_str().unwrap())).await.unwrap();
        let abi = &res.json::<Value>().await.unwrap()["result"];
        let abi = abi.as_str().unwrap();

        println!("{:?} called {} on you", 
            tx.from,
            fn_name
        );
    } return Err(false);
}