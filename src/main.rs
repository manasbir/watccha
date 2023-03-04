use std::{env, io};
use std::time::Duration;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use eyre::{Result, ErrReport};
use tokio;
use ethers::prelude::*;
use dotenv::dotenv;


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


    let monitor = monitor(name, email, monitor_address).await; {
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
                        &tx.input.to_string()[34..74].parse::<Address>().unwrap()
                    );
                        
                } 
            }
        }


    
    }
    Ok(())
}   

fn send_mail (to: String, subject: String, body: String) -> Result<()> {
    let email = Message::builder()
        .from("Web3 Alert <bruhmanmaster@gmail.com>".parse().unwrap())
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
    let provider = Provider::try_from(format!("https://eth-goerli.g.alchemy.com/v2/{}", env::var("RPC").expect("RPC not set"))).unwrap();
    let name = provider.lookup_address(address).await;
    match name {
        Ok(name) => {
            return name;
        },
        Err(_) => {
            return address.to_string();
        }
    }
}
