use std::fs;
use std::{env, io, time::Duration};
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use eyre::{Result, ErrReport};
use serde_json::Value;
use tokio;
use ethers::prelude::*;
pub mod events;
pub mod utils;
use events::*;
use utils::*;

// use clap::{CommandFactory, Parser, Subcommand}; could be what i need
// add libloading

type fns = fn(Transaction, bool) -> Result<String, bool>;





#[tokio::main]
async fn main() {

    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let config: Value = toml::from_str(&toml_str).unwrap();
    let fns = config.get("events").unwrap().as_array().unwrap();
    for functions in fns {
    println!("{:?}", functions);
    }
    println!("{}", events::erc20::from.as_str);


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


    let monitor = monitor(name.replace("\n", "").replace(" ", ""), email.replace("\n", "").replace(" ", ""), monitor_address.to_lowercase().replace("\n", "").replace(" ", "")).await; {
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
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let config: Value = toml::from_str(&toml_str).unwrap();
    let provider = Provider::try_from(config.get("general.rpc_url").unwrap().as_str().unwrap()).unwrap().interval(Duration::from_millis(2000));
    println!("Starting tracker for {} on address: {}", name, monitor_address_str);
    // let provider = SignerMiddleware::new(provider, signer);
    let fns = config.get("events").unwrap().as_array().unwrap();

    for function in fns {
        println!("pizza!")
    }

    let mut stream = provider.watch_blocks().await?;



    while let Some(block) = stream.next().await {
        
        let block_txs = match provider.get_block_with_txs(block).await {
            Ok(block_txs) => {
                if block_txs.is_none() {
                    println!("No transactions in block: {:?}", block);
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
        println!("Block: {:?}, Transactions: {:?}", block, block_txs.transactions.len());

        for tx in block_txs.transactions {
            // do a vec of functions, and if they return !false then we continue

            
           
        }


    
    }
    Ok(())
}   

fn send_mail (to: String, subject: String, body: String) -> Result<()> {
    let email = Message::builder()
        .from("Monitoring Alert <bruhmanmaster@gmail.com>".parse().unwrap())
        .reply_to("manas <bagrimanasbir@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

        let creds = Credentials::new("bagrimanasbir@gmail.com".to_owned(), "--".to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(ErrReport::msg(e)),
    }
}

async fn resolve_ens_name(address: H160) -> String {
    let provider =  Provider::try_from(env::var("RPC").expect("RPC not set")).unwrap().interval(Duration::from_millis(2000));
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
