import { Keypair } from "@solana/web3.js";
const kp = Keypair.generate();

console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);
console.log(`[${kp.secretKey.toString()}]`);
// console.log(`[${kp.secretKey}]`)


