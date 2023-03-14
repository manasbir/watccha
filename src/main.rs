use std::fs;
use std::future::Future;
use std::str::FromStr;
use std::{env, io, time::Duration};
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use eyre::{Result, ErrReport};
use serde_json::Value;
use tokio;
use ethers::prelude::*;
pub mod bindings {
    pub mod erc20;
}
pub mod events;
pub mod utils;
use events::*;
use utils::*;

// use clap::{CommandFactory, Parser, Subcommand}; could be what i need
// add libloading

struct Event {
    name: String,
    email: bool,
    address: H160,
    function: fn(Transaction, H160) -> dyn Future<Output = Result<String, bool>>,
}




#[tokio::main]
async fn main() {

    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let config: Value = toml::from_str(&toml_str).unwrap();

    let fns = config.get("events").unwrap().as_array().unwrap();
    let mut functions: Vec<Event> = Vec::new();

    for f in fns {
        let fn_name = f.get("function").unwrap().as_str().unwrap();
        println!("Adding function: {:?}", f);


        match fn_name {
            "erc20_from" => {
                functions.push(Event {
                    name: "erc20_from".to_string(),
                    email: f.get("email").unwrap().as_bool().unwrap(),
                    address: f.get("address").unwrap().as_str().unwrap().parse().unwrap(),
                    function: events::erc20::from,
                });
            },
            "erc20_to" => {
                functions.push(Event {
                    name: "erc20_to".to_string(),
                    email: f.get("email").unwrap().as_bool().unwrap(),
                    address: f.get("address").unwrap().as_str().unwrap().parse().unwrap(),
                    function: events::erc20::to,
                });
            },
            "generalized" => {
                /* functions.push(Event {
                    name: "generalized".to_string(),
                    email: f.get("email").unwrap().as_bool().unwrap(),
                    address: f.get("address").unwrap().as_str().unwrap().parse().unwrap(),
                    function: events::generalized::generalized,
                }); */
            },
            _ => {
                println!("Invalid function: {}", fn_name);
            }
        }
    }


    let monitor = monitor().await; {
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


async fn monitor() -> Result<()> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let config: Value = toml::from_str(&toml_str).unwrap();
    let provider = Provider::try_from(config.get("general.rpc_url").unwrap().as_str().unwrap()).unwrap().interval(Duration::from_millis(2000));
    // let provider = SignerMiddleware::new(provider, signer);
    
    
    let fns = config.get("events").unwrap().as_array().unwrap();
    let mut functions: Vec<Event> = Vec::new();

    for f in fns {
        let f = f.as_str().unwrap();
        println!("Adding function: {}", f);
        match f {
            "erc20_from" => {

            },
            "erc20_to" => {
                // functions.push(utils::erc20::to);
            },
            "generalized" => {
                // functions.push(utils::erc20::to);
            },
            _ => {
                println!("Invalid function: {}", f);
            }
        }
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

            events::erc20::from(tx, H160::from_str("")).await;
            events::erc20::to(tx, H160::zero()).await;
            
           
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
