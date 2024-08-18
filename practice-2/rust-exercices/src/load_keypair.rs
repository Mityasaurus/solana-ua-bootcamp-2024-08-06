use solana_sdk::signature::Keypair as SolanaKeypair;
use std::env;

pub fn load_keypair() -> SolanaKeypair {
    dotenv::dotenv().ok();

    let secret_key_string = env::var("SECRET_KEY").expect("SECRET_KEY not set");

    let secret_key_bytes: Vec<u8> =
        serde_json::from_str(&secret_key_string).expect("Failed to parse SECRET_KEY as byte array");

    if secret_key_bytes.len() != 64 {
        panic!(
            "Invalid secret key length: expected 64 bytes, got {}",
            secret_key_bytes.len()
        );
    }

    SolanaKeypair::from_bytes(&secret_key_bytes).expect("Failed to create Solana Keypair")
}
