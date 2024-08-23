import {
  Connection,
  PublicKey,
  Transaction,
  SystemProgram,
  NonceAccount,
  clusterApiUrl,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { load_keypair } from "./load-keypair";
import * as fs from "fs";

async function createTransaction() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const sender = new PublicKey("MihZqMqFvq37jmPHCeujvUvKMCKhV6hgzL6KA1gGTnv");
  const recipient = load_keypair("SECRET_KEY_2");

  const noncePubkey = new PublicKey(
    "AiLoLgdGdjHWs4xXRnLXNuyrWtrQqrQndo3w38sDbi7n"
  );

  const nonceAccountInfo = await connection.getAccountInfo(noncePubkey);
  const nonceAccount = NonceAccount.fromAccountData(nonceAccountInfo!.data);

  const transferInstruction = SystemProgram.transfer({
    fromPubkey: sender,
    toPubkey: recipient.publicKey,
    lamports: 0.5 * LAMPORTS_PER_SOL,
  });

  const transaction = new Transaction().add(
    SystemProgram.nonceAdvance({
      noncePubkey,
      authorizedPubkey: sender,
    }),
    transferInstruction
  );

  transaction.feePayer = sender;

  transaction.recentBlockhash = nonceAccount.nonce;

  const serializedTransaction = transaction.serialize({
    requireAllSignatures: false,
  });

  fs.writeFileSync("transaction.bin", serializedTransaction);
}

createTransaction().catch((err) => console.error("Error:", err));
