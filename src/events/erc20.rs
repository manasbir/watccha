use ethers::types::{U256, Transaction};
use crate::bindings::erc20 as ERC20;
use crate::utils::*;

async fn from(tx: Transaction) {
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

async fn to() {
    let mut hi = 3;
}