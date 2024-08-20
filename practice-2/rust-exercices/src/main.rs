mod create_token_account;
mod load_keypair;
mod mint_tokens;

use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let payer = load_keypair::load_keypair();

    let connection = RpcClient::new_with_commitment(
        env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );

    let mint_pubkey = Pubkey::from_str("Fw9YMh5ywx8iWBRPvW3ej3YJcMSmYBE1LgjrJHnAn3PW")?;
    let recipient_associated_token_account =
        Pubkey::from_str("2QL7cwHsQM3BHS9L3GJqZ4a5S3xSYEqYBL7WHAqHnt8U")?;

    let amount_to_mint = 9999 * 10u64.pow(2);

    mint_tokens::mint_tokens(
        &connection,
        &payer,
        &mint_pubkey,
        &recipient_associated_token_account,
        amount_to_mint,
    )?;

    Ok(())
}
