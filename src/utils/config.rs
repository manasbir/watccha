use serde::Serialize;
use ethers::types::H160;


pub struct Config {
    general: General,
    email: Email,
    events: Vec<Listener>,
}
#[derive(Debug, Serialize)]

pub struct General {
    rpc_url: String,
    p_key: String,
    etherscan_key: String,
}
#[derive(Debug, Serialize)]

pub struct Email {
    email: String,
    app_email: String,
    app_password: String,
}
#[derive(Debug, Serialize)]

pub struct Listener {
    function: String,
    email: bool,
    address: H160,
}