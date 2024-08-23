import {
  Connection,
  Transaction,
  sendAndConfirmTransaction,
  clusterApiUrl,
} from "@solana/web3.js";
import { load_keypair } from "./load-keypair";
import * as fs from "fs";

async function senderSigning() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const sender = load_keypair("SECRET_KEY");

  const serializedTransaction = fs.readFileSync("transaction.bin");

  const loadedTransaction = Transaction.from(serializedTransaction);

  loadedTransaction.sign(sender);

  const signature = await connection.sendRawTransaction(
    loadedTransaction.serialize()
  );

  console.log(`✅ Transaction sent with signature: ${signature}`);
}

senderSigning().catch((err) => console.error("Error:", err));
