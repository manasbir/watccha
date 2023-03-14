use ethers::prelude::SignerMiddleware;
use ethers::types::H160;
use ethers::providers::Provider;
use toml::Value;
use std::fs;
use std::time::Duration;


async fn resolve_ens_name(address: H160) -> String {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let config: Value = toml::from_str(&toml_str).unwrap();
    let provider = Provider::try_from(config["general"]["rpc_url"].as_str().unwrap()).unwrap().interval(Duration::from_millis(2000));
    let signer = config["general"]["p_key"].as_str().unwrap().parse().unwrap();
    SignerMiddleware::new(provider, signer);

    let name = provider.lookup_address(address).await;
    match name {
        Ok(name) => {
            return name;
        },
        Err(_) => {
            return format!("{:?}", address);
        }
    }
}