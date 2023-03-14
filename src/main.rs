use std::fs;
use std::future::Future;
use std::str::FromStr;
use std::{env, io, time::Duration};
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use eyre::{Result, ErrReport};
use toml::Value;
use tokio;
use ethers::prelude::*;
pub mod bindings {
    pub mod erc20;
}
pub mod events;
pub mod utils;
use events::*;

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

    let toml_str = fs::read_to_string("src/config.toml").expect("Failed to read file");
    let config: Value = toml_str.parse::<Value>().expect("Failed to parse TOML");

    let fns = config.get("events").unwrap().as_array().unwrap();
    let mut functions: Vec<Event> = Vec::new();

    for f in fns {
        let fn_name = f.get("function").unwrap().as_str().unwrap();
        println!("Adding function: {:?}", f);
/* 

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
        } */
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
    let provider = Provider::try_from(config["general"]["rpc_url"].as_str().unwrap()).unwrap().interval(Duration::from_millis(2000));
    // let provider = SignerMiddleware::new(provider, signer);



    let latest_block = provider.get_block_number().await?;
    let block_txs = provider.get_block_with_txs(latest_block).await.unwrap().unwrap();
    let tx = block_txs.transactions[0].clone();

    let test = events::generalized::decoder(tx.clone(), tx.from).await;
    match test {
        Ok(res) => {
            println!("Test: {:?}", res);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    
    let fns = config["events"].as_array().unwrap();
    let mut functions: Vec<Event> = Vec::new();

    for f in fns {
        let f = f["function"].as_str().unwrap();
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

            let res = events::erc20::from(tx.clone(), H160::from_str("0x9696bc05C4E9B8992059915eA69EF4cDf391116B").unwrap()).await;
            match res {
                Ok(res) => {
                    send_mail("ERC20 Transfer".to_string(), res).unwrap();
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            let res = events::erc20::to(tx.clone(), H160::from_str("0x9696bc05C4E9B8992059915eA69EF4cDf391116B").unwrap()).await;
            match res {
                Ok(res) => {
                    send_mail("ERC20 Transfer".to_string(), res).unwrap();
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            let res = events::generalized::decoder(tx.clone(), H160::from_str("0x9696bc05C4E9B8992059915eA69EF4cDf391116B").unwrap()).await;
            match res {
                Ok(res) => {
                    send_mail("Generalized Transfer".to_string(), res).unwrap();
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
           
        }


    
    }
    Ok(())
}   

fn send_mail (subject: String, body: String) -> Result<()> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let config: Value = toml::from_str(&toml_str).unwrap();
    let to = config.get("general.email").unwrap().as_str().unwrap();

    let email = Message::builder()
        .from("Monitoring Alert <bruhmanmaster@gmail.com>".parse().unwrap())
        .reply_to("manas <bagrimanasbir@gmail.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

        let creds = Credentials::new("bagrimanasbir@gmail.com".to_owned(), config.get("general.app_password").unwrap().as_str().unwrap() .to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(ErrReport::msg(e)),
    }
}


