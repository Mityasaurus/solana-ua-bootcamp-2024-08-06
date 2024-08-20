use instruction::create_associated_token_account;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey, signer::keypair::Keypair, signer::Signer, transaction::Transaction,
};
use spl_associated_token_account::{get_associated_token_address, instruction};
use spl_token::id as spl_token_id;

pub fn create_token_account(
    connection: &RpcClient,
    payer: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let associated_token_address = get_associated_token_address(owner, mint);

    if let Ok(account) = connection.get_account(&associated_token_address) {
        if account.owner == spl_token_id() {
            println!("Token account already exists.");
            return Ok(associated_token_address);
        }
    }

    let create_account_ix =
        create_associated_token_account(&payer.pubkey(), owner, mint, &spl_token_id());

    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix],
        Some(&payer.pubkey()),
        &[payer],
        connection.get_latest_blockhash()?,
    );

    connection.send_and_confirm_transaction(&tx)?;

    println!("Associated token account created.");
    Ok(associated_token_address)
}
