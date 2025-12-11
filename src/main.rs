use std::error::Error;

mod yellowstone;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    
    let yellowstone_endpoint = std::env::var("YELLOWSTONE_ENDPOINT")
        .unwrap_or_else(|_| "http://127.0.0.1:10000".to_string());
    let yellowstone_token = std::env::var("YELLOWSTONE_TOKEN").ok();

    println!("Starting PumpSwap Indexer...");
    println!("   Endpoint: {}", yellowstone_endpoint);
    println!("   Token: {}", if yellowstone_token.is_some() { "Set" } else { "Not set" });

    let worker = yellowstone::YellowstoneWorker::new(yellowstone_endpoint, yellowstone_token);
    worker.run().await;

    Ok(())
}
