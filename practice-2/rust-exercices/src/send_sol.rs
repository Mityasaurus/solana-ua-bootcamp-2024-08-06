use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;

pub fn send_sol(
    keypair: &Keypair,
    recipient_str: &str,
    amount_sol: f64,
    cluster_api_url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let connection =
        RpcClient::new_with_commitment(cluster_api_url.to_string(), CommitmentConfig::confirmed());

    println!("🔑 Our public key is: {}", keypair.pubkey());

    let recipient = Pubkey::from_str(recipient_str)?;
    println!("Attempting to send {} SOL to {}", amount_sol, recipient);

    let lamports = (amount_sol * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64;

    let send_sol_instruction = transfer(&keypair.pubkey(), &recipient, lamports);

    // Memo Program ID
    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
    let memo_text = "Hello from Solana!";

    let memo_instruction = Instruction::new_with_bytes(
        memo_program_id,
        memo_text.as_bytes(),
        vec![AccountMeta::new(keypair.pubkey(), true)],
    );

    let message = Message::new(
        &[send_sol_instruction, memo_instruction],
        Some(&keypair.pubkey()),
    );
    let transaction = Transaction::new(&[keypair], message, connection.get_latest_blockhash()?);

    let signature = connection.send_and_confirm_transaction(&transaction)?;
    println!("✅ Transaction confirmed, signature: {}", signature);

    Ok(signature.to_string())
}
