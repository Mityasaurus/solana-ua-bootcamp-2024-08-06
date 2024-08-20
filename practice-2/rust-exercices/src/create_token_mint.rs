use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;
use std::error::Error;

pub fn create_mint(
    keypair: &Keypair,
    decimals: u8,
    cluster_api_url: &str,
) -> Result<Pubkey, Box<dyn Error>> {
    let connection =
        RpcClient::new_with_commitment(cluster_api_url.to_string(), CommitmentConfig::confirmed());

    let mint_pubkey = Keypair::new();

    let rent_exemption = connection.get_minimum_balance_for_rent_exemption(Mint::LEN)?;

    let create_account_ix = system_instruction::create_account(
        &keypair.pubkey(),
        &mint_pubkey.pubkey(),
        rent_exemption,
        Mint::LEN as u64,
        &spl_token::id(),
    );

    let initialize_mint_ix = initialize_mint(
        &spl_token::id(),
        &mint_pubkey.pubkey(),
        &keypair.pubkey(),
        None,
        decimals,
    )?;

    let mut transaction = Transaction::new_with_payer(
        &[create_account_ix, initialize_mint_ix],
        Some(&keypair.pubkey()),
    );
    transaction.sign(
        &[keypair, &mint_pubkey],
        connection.get_latest_blockhash().unwrap(),
    );

    connection.send_and_confirm_transaction(&transaction)?;

    println!("✅ Token mint created: {}", mint_pubkey.pubkey());

    Ok(mint_pubkey.pubkey())
}
