use solana_sdk::signature::Signer;
mod load_keypair;
mod send_sol;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keypair = load_keypair::load_keypair();

    println!("{}", keypair.pubkey());

    // send_sol::send_sol(
    //     &keypair,
    //     "9mR4XFLEuF8zCcunEjJzm4n4Ua8XUrA8RwKr22vbJYus",
    //     0.01,
    //     "https://api.devnet.solana.com",
    // )?;

    Ok(())
}
