import "dotenv/config";
import { Keypair } from "@solana/web3.js";

export const load_keypair = (env_name: string): Keypair => {
  const asArray = Uint8Array.from(JSON.parse(process.env[env_name]!));
  return Keypair.fromSecretKey(asArray);
};
