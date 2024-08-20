use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token::{id as spl_token_id, instruction::mint_to};

pub fn mint_tokens(
    connection: &RpcClient,
    sender: &Keypair,
    mint: &Pubkey,
    recipient: &Pubkey,
    amount: u64,
) -> Result<String, Box<dyn std::error::Error>> {
    let mint_to_ix = mint_to(
        &spl_token_id(),
        mint,
        recipient,
        &sender.pubkey(),
        &[],
        amount,
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&sender.pubkey()),
        &[sender],
        recent_blockhash,
    );

    let signature = connection.send_and_confirm_transaction(&tx)?;

    println!("✅ Success! Mint Token Transaction: {}", signature);

    Ok(signature.to_string())
}
