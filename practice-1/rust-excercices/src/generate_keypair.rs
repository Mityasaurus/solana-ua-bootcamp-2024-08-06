use solana_sdk::signature::{Keypair, Signer};
use base58::ToBase58;

pub fn generate() {
    let keypair = Keypair::new();
    let public_key = keypair.pubkey();
    println!("Public key: {}", public_key.to_string());

    let secret_key = keypair.secret().as_ref().to_base58();
    println!("Private key: {}", secret_key);
}
