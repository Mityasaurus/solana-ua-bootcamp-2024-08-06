mod check_balance;
mod load_keypair;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = check_balance::check_balance("https://api.devnet.solana.com").await {
        eprintln!("Error: {:?}", e);
    }

    Ok(())
}