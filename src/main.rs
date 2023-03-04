use std::{env, io};
use std::time::Duration;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use eyre::{Result, ErrReport};
use tokio;
use ethers::prelude::*;

#[tokio::main]
async fn main() {
    println!("What's your name? \n");
    let name = String::new();

    io::stdin().read_line(&mut name).expect("failed to readline");
 
    print!("You entered {}", name);

}


async fn monitor(name: String, email: String, monitor_address: String) -> Result<()> {
    let provider =  Provider::try_from(env::var("RPC").expect("RPC not set")).unwrap().interval(Duration::from_millis(2000));
    let chain_id = provider.get_chainid().await?.as_u64();
    let signer = env::var("P_KEY").expect("P_KEY not set").parse::<LocalWallet>()?.with_chain_id(chain_id);
    let provider = SignerMiddleware::new(provider, signer);

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

        

    
    }
}   

fn send_mail (to: String, subject: String, body: String) -> Result<()> {
    println!("Hello, world!");
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
