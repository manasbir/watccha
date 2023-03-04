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
    
    let res = reqwest::get(format!("https://api-goerli.etherscan.io/api?module=contract&action=getabi&address={:?}&apikey={}", "7b5C526B7F8dfdff278b4a3e045083FBA4028790", env::var("E_KEY").expect("E_KEY not set").as_str())).await.unwrap();
    let abi = &res.json::<Value>().await.unwrap()["result"];
    let abi = abi.as_str().unwrap();
    println!("ABI: {:?}", abi);




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
    println!("Starting tracker for {} on address: {}", name, monitor_address_str);

    let provider =  Provider::try_from(env::var("RPC").expect("RPC not set")).unwrap().interval(Duration::from_millis(2000));
    let chain_id = provider.get_chainid().await?.as_u64();
    let signer = env::var("P_KEY").expect("P_KEY not set").parse::<LocalWallet>()?.with_chain_id(chain_id);
    let provider = SignerMiddleware::new(provider, signer);
    let monitor_address = monitor_address_str.parse::<Address>()?;

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
            if tx.to == Some(monitor_address) || tx.from == monitor_address {
                println!("From: {:?}, To: {:?}", tx.from, tx.to.unwrap());
            }
            if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) {
                if tx.input.to_string().is_fn(&"0xa9059cbb") {
                    println!("{} transferred {} {:?} to you and now transfering out....", 
                        tx.from,
                        &tx.input.to_string()[74..138].parse::<U256>().unwrap()/U256::from(10u64.pow(18)),
                        ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await?
                    );
                    let sent_mail = send_mail(format!("{} <{}>", name, email).to_string(), format!("{:?} MONITORING ALERT! | ERC20 Transfer", monitor_address_str).to_string(), 
                        format!("{:?} transferred {} {} to you on block {} \n View here: https://etherscan.io/tx/{}", 
                            resolve_ens_name(tx.from).await,
                            &tx.input.to_string()[74..138].parse::<U256>().unwrap()/U256::from(10u64.pow(18)),
                            ERC20::ERC20::new(tx.to.unwrap(), provider.clone().into()).symbol().call().await?,
                            tx.block_number.unwrap(),
                            tx.hash
                        )
                    ); {
                        match sent_mail {
                            Ok(_) => {
                                println!("Mail sent");
                            },
                            Err(e) => {
                                println!("Error sending mail: {:?}", e);
                            }
                        }
                    }
                        
                } else {
                    // generalized decoder time
                
                    // get fn sig
                    // put into sig.samczsun.com
                    // get fn name and params
                    // create abigen
                    // fully decoded
                    // get gpt to generate an email
                    // send email
                    // profit

                    let body = reqwest::get(format!("https://sig.eth.samczsun.com/api/v1/signatures?function={}", &tx.input.to_string()[0..10])).await?;  
                    let fn_name = body.json::<Value>().await?["result"]["function"][&tx.input.to_string()[0..10]][0]["name"].as_str().unwrap().to_string();
                    let res = reqwest::get(format!("https://api-goerli.etherscan.io/api?module=contract&action=getabi&address={:?}&apikey={}", &tx.to.unwrap(), env::var("E_KEY").expect("E_KEY not set"))).await.unwrap();
                    let abi = &res.json::<Value>().await.unwrap()["result"];
                    let abi = abi.as_str().unwrap();

                    println!("{:?} called {} on you", 
                        tx.from,
                        fn_name
                    );
                }
            }
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

        let creds = Credentials::new("bagrimanasbir@gmail.com".to_owned(), "aapzuovuauhjbamy".to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
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
