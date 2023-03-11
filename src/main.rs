use std::thread::sleep;
use std::{env, io};
use std::time::Duration;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use eyre::{Result, ErrReport};
use serde_json::Value;
use tokio;
use ethers::prelude::*;
use dotenv::dotenv;
pub mod bindings { pub mod erc20;}
use bindings::erc20 as ERC20;


trait IsFn {
    fn is_fn (&self, fn_sig: &str) -> bool;
}

impl IsFn for str {
    fn is_fn(&self, fn_sig: &str) -> bool {
        if self.to_lowercase().contains(fn_sig) {
            return true;
        } else {
            return false;
        }
    }
}

#[tokio::main]
async fn main() {

    dotenv().ok();




    println!("What's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to readline");


    println!("What's your email?");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("failed to readline");
    assert!(email.contains("@"), "Invalid email");

    println!("What's the address you want to monitor?");
    let mut monitor_address = String::new();
    io::stdin().read_line(&mut monitor_address).expect("failed to readline");
    assert!(monitor_address.contains("0x"), "Invalid address");


    let monitor = monitor("manas".to_string(), "bagrimanasbir@gmail.com".to_string(), "0x9696bc05C4E9B8992059915eA69EF4cDf391116B".to_string()).await; {
        match monitor {
            Ok(_) => {
                println!("Tracker exited");
            },
            Err(e) => {
                println!("Tracker exited with error: {:?}", e);
            }
        }
    }

}


async fn monitor(name: String, email: String, monitor_address_str: String) -> Result<()> {
    println!("Starting tracker for {} on address: {}", name, monitor_address_str);

    let provider =  Provider::try_from(env::var("RPC").expect("RPC not set")).unwrap().interval(Duration::from_millis(20000));
    let chain_id = provider.get_chainid().await?.as_u64();
    let signer = env::var("P_KEY").expect("P_KEY not set").parse::<LocalWallet>()?.with_chain_id(chain_id);
    let provider = SignerMiddleware::new(provider, signer);
    let mut block_number = 25186796;
    // let monitor_address = monitor_address_str.parse::<Address>()?;

    loop {
        block_number += 1;
        auth
        let block_txs = match provider.get_block_with_txs(block_number).await {
            Ok(block_txs) => {
                if block_txs.is_none() {
                    println!("No transactions in block: {:?}", block_number);
                    continue;
                } else {
                    block_txs.unwrap()
                }
            },
            Err(e) => {
                println!("Error getting block txs: {:?}", e);
                continue;
            }
        };

        for tx in block_txs.transactions {
            if tx.input.to_string().to_lowercase().contains("0x11b3d5e7") || tx.input.to_string().to_lowercase().contains("0xf5e3c462") || tx.input.to_string().to_lowercase().contains("0xaae40a2a") {
                println!("{:?}", tx.from);
            }
        }
    
    }
    Ok(())
}   
