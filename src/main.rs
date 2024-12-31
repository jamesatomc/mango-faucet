use reqwest::blocking::Client;
use serde_json::json;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let client = Client::new();
    let url = "https://faucet.testnet.mangonetwork.io/gas";
    let payload = json!({
        "FixedAmountRequest": {
            "recipient": "addrees"
        }
    });

    for _ in 0..10 {
        match client.post(url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send() {
                Ok(response) => println!("Request sent: {:?}", response.status()),
                Err(e) => eprintln!("Error sending request: {}", e),
            }
        sleep(Duration::from_secs(20));
    }
}