use std::env;
use base58::FromBase58;
use ed25519_dalek::{SecretKey as Ed25519SecretKey, PublicKey as Ed25519PublicKey};
use solana_sdk::signature::Keypair as SolanaKeypair;

pub fn load_keypair() -> SolanaKeypair {
    dotenv::dotenv().ok();

    let secret_key_base58 = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let secret_key_bytes = secret_key_base58.from_base58().expect("Failed to decode Base58");

    if secret_key_bytes.len() != 32 {
        panic!("Invalid secret key length: expected 32 bytes, got {}", secret_key_bytes.len());
    }

    let ed25519_secret_key = Ed25519SecretKey::from_bytes(&secret_key_bytes).expect("Invalid secret key");
    let ed25519_public_key = Ed25519PublicKey::from(&ed25519_secret_key);

    let public_key_bytes = ed25519_public_key.to_bytes();
    let public_key_bytes_length = public_key_bytes.len();
    if public_key_bytes_length != 32 {
        panic!("Invalid public key length: expected 32 bytes, got {}", public_key_bytes_length);
    }

    let mut keypair_bytes = secret_key_bytes.to_vec();
    keypair_bytes.extend_from_slice(&public_key_bytes);

    SolanaKeypair::from_bytes(&keypair_bytes).expect("Failed to create Solana Keypair")
}
