import "dotenv/config";
import { PublicKey, Connection, clusterApiUrl } from "@solana/web3.js";
import { createMultisig } from "@solana/spl-token";
import { load_keypair } from "./load-keypair";

async function createMultisigAccount() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const signer1 = load_keypair("SECRET_KEY");
  const signer2 = load_keypair("SECRET_KEY_2");

  const requiredSigners = 2;

  let multisigAccountPubkey: PublicKey = await createMultisig(
    connection,
    signer1,
    [signer1.publicKey, signer2.publicKey],
    requiredSigners
  );

  console.log(`✅ Multisig account created: ${multisigAccountPubkey}`);
}

createMultisigAccount().catch((err) => console.error("Error:", err));
