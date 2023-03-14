use etherstypes::{U256, Transaction, Provider};
use crate::bindings::erc20 as ERC20;
use crate::utils::is_fn::IsFn;
use toml::Value;

async fn from(tx: Transaction, monitor_address: H160, email: bool) -> Result<String, bool> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let toml: Value = toml::from_str(&toml_str).unwrap();

    let provider = Provider::try_from(toml.general.rpc_url).unwrap();

    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
        if tx.input.to_string().is_fn(&"0xa9059cbb") {
            println!("{} transferred {} {:?} to you and now transfering out....", 
                tx.from,
                &tx.input.to_string()[74..138].parse::<U256>().unwrap() / U256::from(10u64.pow(18)),
                ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await?
            );
        }
    }
    return String::from("test");
}

async fn from(tx: Transaction, email: bool) {
    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
        if tx.input.to_string().is_fn(&"0xa9059cbb") {
            println!("{} transferred {} {:?} to you and now transfering out....", 
                tx.from,
                &tx.input.to_string()[74..138].parse::<U256>().unwrap() / U256::from(10u64.pow(18)),
                ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await?
            );
        }
    }
}