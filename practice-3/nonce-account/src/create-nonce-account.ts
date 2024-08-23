import "dotenv/config";
import {
  Connection,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  SystemProgram,
  NonceAccount,
  clusterApiUrl,
} from "@solana/web3.js";
import { load_keypair } from "./load-keypair";

async function createNonceAccount() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const nonceAccountKeypair = Keypair.generate();
  const payer = load_keypair("SECRET_KEY");

  const noncePubkey = nonceAccountKeypair.publicKey;

  const nonceAccountLength = 80;

  const nonceAccountBalance =
    await connection.getMinimumBalanceForRentExemption(nonceAccountLength);

  const createNonceAccountTransaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      lamports: nonceAccountBalance,
      newAccountPubkey: noncePubkey,
      programId: SystemProgram.programId,
      space: nonceAccountLength,
    }),
    SystemProgram.nonceInitialize({
      noncePubkey: noncePubkey,
      authorizedPubkey: payer.publicKey,
    })
  );

  await sendAndConfirmTransaction(connection, createNonceAccountTransaction, [
    payer,
    nonceAccountKeypair,
  ]);

  console.log(`✅ Nonce account created: ${noncePubkey.toBase58()}`);
}

createNonceAccount().catch((err) => console.error("Error:", err));
