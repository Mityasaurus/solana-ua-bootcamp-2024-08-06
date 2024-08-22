import {
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
  Connection,
  clusterApiUrl,
} from "@solana/web3.js";
import { load_keypair } from "./load-keypair";
import {
  getOrCreateAssociatedTokenAccount,
  createTransferInstruction,
} from "@solana/spl-token";

async function createTokenTransferTransaction() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const sender = load_keypair("SECRET_KEY");
  const recipient = load_keypair("SECRET_KEY_2");

  const mintPublicKey = new PublicKey(
    "Fw9YMh5ywx8iWBRPvW3ej3YJcMSmYBE1LgjrJHnAn3PW"
  );

  const senderTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    sender,
    mintPublicKey,
    sender.publicKey
  );
  const recipientTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    recipient,
    mintPublicKey,
    recipient.publicKey
  );

  const transferInstruction = createTransferInstruction(
    senderTokenAccount.address,
    recipientTokenAccount.address,
    sender.publicKey,
    10000
  );

  const transaction = new Transaction().add(transferInstruction);

  transaction.recentBlockhash = (
    await connection.getLatestBlockhash()
  ).blockhash;

  transaction.sign(recipient, sender);

  const signature = await sendAndConfirmTransaction(connection, transaction, [
    recipient,
    sender,
  ]);

  console.log(`Transaction successfully sent. Signature: ${signature}`);
}

createTokenTransferTransaction().catch((err) => console.error(err));
