import "dotenv/config";
import { PublicKey, Connection, clusterApiUrl } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { load_keypair } from "./load-keypair";

async function createMultisigTokenAccount() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const signer1 = load_keypair("SECRET_KEY");

  const multisigAccountPubkey = new PublicKey(
    "2UMFerhT4TMX792sbtMo6Td3GESyVMNocmgAxvbkoDU3"
  );

  const mint = new PublicKey("GDKtQVg57kjXxLsHkfVzpDRGV12qKsVcQxs8Ei2GFdoZ");

  const multisigTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    signer1,
    mint,
    multisigAccountPubkey
  );

  console.log(
    `✅ Multisig token account: ${multisigTokenAccount.address.toBase58()}`
  );
}

createMultisigTokenAccount().catch((err) => console.error("Error:", err));
