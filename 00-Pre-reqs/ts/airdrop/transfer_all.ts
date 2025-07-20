// transfer_all.ts (optional extra script)
import { Connection, Transaction, SystemProgram, Keypair, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("FXtkuJGP5Zmwu3QUgvhm3MuzcxUV3AMyd6WHdwnT8rxr");
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    const balance = await connection.getBalance(from.publicKey);

    let transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      })
    );

    transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    transaction.feePayer = from.publicKey;

    const fee = (await connection.getFeeForMessage(transaction.compileMessage(), 'confirmed')).value || 0;

    transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [from]);
    console.log(`Tranfserred all! TX: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(e);
  }
})();
