use std::fs;
use ethers::{types::{Transaction, H160}, utils::format_ether};
use reqwest::{header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue}, Client};
use serde_json::json;
use toml::Value;

// lots of work to go here


pub(crate) async fn decoder(tx: Transaction, monitor_address: H160) -> Result<String, bool> {
    let toml_str = fs::read_to_string("src/config.toml").unwrap();
    let toml: Value = toml::from_str(&toml_str).unwrap();
    let monitor_address_str = monitor_address.to_string();

    if tx.input.to_string().to_lowercase().contains(&monitor_address_str.replace("0x", "").to_lowercase()) || tx.from == monitor_address || tx.to == Some(monitor_address){
        if tx.input.to_string().len() < 10 {
            return Ok(format!("{:?} sent {} eth to {:?} on tx {:?}", tx.from, format_ether(tx.value) , tx.to.unwrap(), tx.hash));
        }

        // lots of work to be done here
        let num_of_data = tx.input.to_string().len() - 10 / 64;
        let mut data: Vec<String> = Vec::new();
        for i in 0..num_of_data {
            if i*64+10+64 > tx.input.to_string().len() {
                break;
            }
            data.push(tx.input.to_string()[10 + (i * 64)..10 + (i * 64) + 64].to_string());
        }

        let body = reqwest::get(format!("https://sig.eth.samczsun.com/api/v1/signatures?function={}", &tx.input.to_string()[0..10])).await.unwrap();  
        let fn_name = body.json::<Value>().await.unwrap();
        let fn_name = &fn_name["result"]["function"][&tx.input.to_string()[0..10]];
        if fn_name.as_array() == Some(&vec![]){
            return Ok(format!("{:?} called a unknown function to {:?} with the sig {} and data {:#?}", tx.from, tx.to.unwrap(), &tx.input.to_string()[0..10], data.join(", ")));
        }
        let fn_name = fn_name[0]["name"].as_str().unwrap().to_string();
        dbg!(&fn_name);
        let res = reqwest::get(format!("https://api-goerli.etherscan.io/api?module=contract&action=getabi&address={:?}&apikey={}", &tx.to.unwrap(), toml["general"]["etherscan_key"].as_str().unwrap())).await.unwrap();
        let abi = &res.json::<Value>().await.unwrap()["result"];
        let abi = abi.as_str().unwrap();

        

        let prompt = format!("Generate an email for a eth transaction (dont include a subject line or a greeting(e.g \"dear [recipient],\"),, be concise) with the following function name {} and the following inputs {} to this address {:?} from this address {:?} \n 
        ex:
            0xfb9779477e5b4834bf2bc02dd29b97b344d0f700 called swap to 0xwadadada with params of x tokens of dai and x tokens of usdc
        ", 
            fn_name,
            data.join(", "),
            tx.to.unwrap(),
            tx.from
        );

        print!("Prompt: {}", prompt);
        let url = "https://api.openai.com/v1/engines/davinci-codex/completions";
    
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", toml["general"]["openai_api"].as_str().unwrap())).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    
        let response = client.post(url)
            .headers(headers)
            .json(&json!({
                "prompt": prompt,
                "max_tokens": 20,
                "temperature": 0.5
            }))
            .send()
            .await.unwrap();
    
        let result = response.json::<serde_json::Value>().await.unwrap();
        println!("{:?}", result);
        println!("{}", result["choices"][0]["text"]);
    

        println!("{:?} called {} on tx {:?}", 
            tx.from,
            fn_name,
            tx.hash
        );
    } return Err(false);
}