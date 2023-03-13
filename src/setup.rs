use std::{env, io};
use toml::{to_string, from_str};
use ethers::types::{H160, Transaction};
use tui::{Terminal, backend::CrosstermBackend};
use eyre::{Result, ErrReport};

struct Config {
    general: General,
    email: Email,
    address: H160,
    events: Vec<fn(Transaction, bool) -> Result<String, bool>>
}

struct General {
    rpc_url: String,
    p_key: String,
    etherscan_key: String,
}

struct Email {
    email: String,
    app_email: String,
    app_password: String,
}
fn main() {

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {} in file {}", query, file_path);

    let rpc_url = String::new();
    let p_key = String::new();
    let etherscan_key = String::new();
    let email = String::new();
    let app_email = String::new();
    let app_password = String::new();

    let monitor_address = String::new();


    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout())).unwrap();


}