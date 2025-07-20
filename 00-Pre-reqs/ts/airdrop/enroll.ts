import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { AnchorProvider, Wallet, Program } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" });
const program = new Program(IDL, provider);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

const account_seeds = [
  Buffer.from("prereqs"),
  keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

const mintTs = Keypair.generate();

const authoritySeeds = [
  Buffer.from("collection"),
  mintCollection.toBuffer(),
];
const [authority] = PublicKey.findProgramAddressSync(authoritySeeds, program.programId);

(async () => {
  try {
    const txhash = await program.methods
      .initialize("IshaanXCoder") 
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        system_program: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();

    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }

  try {
    const txhash = await program.methods
      .submitTs()
      .accountsPartial({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority: authority,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SystemProgram.programId,
      })
      .signers([keypair, mintTs])
      .rpc();

    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();