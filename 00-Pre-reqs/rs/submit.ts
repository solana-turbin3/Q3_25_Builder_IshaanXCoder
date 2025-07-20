import fs from "fs";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import anchor from "@coral-xyz/anchor";
import IDL from "./idl.json" assert { type: "json" };

const { AnchorProvider, Program, Wallet } = anchor;

const programId = new PublicKey(IDL.address); 

async function main() {
  const secret = JSON.parse(fs.readFileSync("dev-wallet.json", "utf8"));
  const keypair = Keypair.fromSecretKey(Uint8Array.from(secret));

  const connection = new Connection("https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a");
  const wallet = new Wallet(keypair);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
    preflightCommitment: "confirmed"
  });

  const program = new Program(IDL, programId, provider); 

  const [prereqsPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("prereqs"), keypair.publicKey.toBuffer()],
    programId
  );

  const tx = await program.methods
    .initialize("IshaanXCoder") 
    .accounts({
      user: keypair.publicKey,
      account: prereqsPda,
      systemProgram: SystemProgram.programId,
    })
    .signers([keypair])
    .rpc();

  console.log("✅ Initialized account! TX:", tx);
}

main().catch(console.error);
