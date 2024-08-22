import "dotenv/config";
import {
  PublicKey,
  Connection,
  clusterApiUrl,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";
import {
  createInitializeMultisigInstruction,
  getOrCreateAssociatedTokenAccount,
  createMint,
} from "@solana/spl-token";
import { load_keypair } from "./load-keypair";

async function createMintAccount() {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

  const signer1 = load_keypair("SECRET_KEY");

  const multisigAccountPubkey = new PublicKey(
    "2UMFerhT4TMX792sbtMo6Td3GESyVMNocmgAxvbkoDU3"
  );

  const mint = await createMint(
    connection,
    signer1,
    multisigAccountPubkey,
    multisigAccountPubkey,
    2
  );

  console.log(`✅ Mint created: ${mint.toBase58()}`);

  //   // Создаем токенный аккаунт, связанный с мультиподписным аккаунтом
  //   const multisigTokenAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     signer1,
  //     mint,
  //     multisigAccountPubkey
  //   );

  //   console.log(
  //     `Multisig token account: ${multisigTokenAccount.address.toBase58()}`
  //   );

  //   // Минтим токены
  //   const mintAmount = 1000000000; // 1 токен с учетом decimals = 9

  //   const mintTransactionSignature = await mintTo(
  //     connection,
  //     signer1, // fee payer
  //     mint,
  //     multisigTokenAccount.address, // Куда минтить
  //     multisigAccountPubkey, // multisig authority
  //     mintAmount,
  //     [signer1, signer2] // Подписи подписантов для minting
  //   );

  //   console.log(
  //     `💸 Minted ${mintAmount} tokens to ${multisigTokenAccount.address.toBase58()}`
  //   );

  //   console.log(`✅ Signature: ${mintTransactionSignature}`);
}

createMintAccount().catch((err) => console.error("Error:", err));
