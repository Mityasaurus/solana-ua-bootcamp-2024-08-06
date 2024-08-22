import "dotenv/config";
import { PublicKey, Connection, clusterApiUrl } from "@solana/web3.js";
import { mintTo } from "@solana/spl-token";
import { load_keypair } from "./load-keypair";

async function mintTokens() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const signer1 = load_keypair("SECRET_KEY");
  const signer2 = load_keypair("SECRET_KEY_2");

  const multisigAccountPubkey = new PublicKey(
    "2UMFerhT4TMX792sbtMo6Td3GESyVMNocmgAxvbkoDU3"
  );

  const mint = new PublicKey("GDKtQVg57kjXxLsHkfVzpDRGV12qKsVcQxs8Ei2GFdoZ");

  const multisigTokenAccount = new PublicKey(
    "5CbeWmKb77bAYfw2TFspv1m29hoTMTKzKm47pbytGbQ2"
  );

  const mintAmount = 3500;

  const mintTransactionSignature = await mintTo(
    connection,
    signer1,
    mint,
    multisigTokenAccount,
    multisigAccountPubkey,
    mintAmount,
    [signer1, signer2]
  );

  console.log(
    `💸 Minted ${mintAmount} tokens to ${multisigTokenAccount.toBase58()}`
  );

  console.log(`✅ Signature: ${mintTransactionSignature}`);
}

mintTokens().catch((err) => console.error("Error:", err));
