use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signer;
use crate::load_keypair;

pub async fn check_balance(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keypair = load_keypair::load_keypair();
    let public_key = keypair.pubkey();

    let connection = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    println!("⚡️ Connected to devnet");

    let balance_in_lamports = connection.get_balance(&public_key)?;
    let balance_in_sol = balance_in_lamports as f64 / solana_sdk::native_token::LAMPORTS_PER_SOL as f64;

    println!("💰 The balance for the wallet at address {} is: {}", public_key, balance_in_sol);

    Ok(())
}
