import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault1 } from "../target/types/vault1";
import { expect } from "chai";

describe("vault1", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Vault1 as Program<Vault1>;
  const user = provider.wallet;

  let vaultStatePda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let vaultBump: number;
  let vaultStateBump: number;

  before(async () => {
    [vaultStatePda, vaultStateBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("state"), user.publicKey.toBuffer()],
      program.programId
    );

    [vaultPda, vaultBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), vaultStatePda.toBuffer()],
      program.programId
    );
  });

  it("initializes the vault", async () => {
    await program.methods.initialize()
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.vaultState.fetch(vaultStatePda);
    expect(state.vaultBump).to.equal(vaultBump);
    expect(state.stateBump).to.equal(vaultStateBump);
  });

  it("deposits lamports", async () => {
    const amount = anchor.web3.LAMPORTS_PER_SOL / 10; // 0.1 SOL

    const userBefore = await provider.connection.getBalance(user.publicKey);
    const vaultBefore = await provider.connection.getBalance(vaultPda);

    await program.methods.deposit(new anchor.BN(amount))
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const userAfter = await provider.connection.getBalance(user.publicKey);
    const vaultAfter = await provider.connection.getBalance(vaultPda);

    expect(vaultAfter).to.equal(vaultBefore + amount);
    expect(userAfter).to.be.lessThan(userBefore); // Because of tx fee
  });

  it("withdraws lamports", async () => {
    const amount = anchor.web3.LAMPORTS_PER_SOL / 20; // 0.05 SOL

    const userBefore = await provider.connection.getBalance(user.publicKey);
    const vaultBefore = await provider.connection.getBalance(vaultPda);

    await program.methods.withdraw(new anchor.BN(amount))
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const userAfter = await provider.connection.getBalance(user.publicKey);
    const vaultAfter = await provider.connection.getBalance(vaultPda);

    expect(vaultAfter).to.equal(vaultBefore - amount);
    expect(userAfter).to.be.greaterThan(userBefore - amount); // Because of tx fee
  });

  it("closes the vault", async () => {
    const vaultBefore = await provider.connection.getBalance(vaultPda);
    const userBefore = await provider.connection.getBalance(user.publicKey);

    await program.methods.close()
      .accounts({
        user: user.publicKey,
        vaultState: vaultStatePda,
        vault: vaultPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const userAfter = await provider.connection.getBalance(user.publicKey);
    let vaultInfo;

    try {
      vaultInfo = await provider.connection.getAccountInfo(vaultStatePda);
    } catch (_) {
      vaultInfo = null;
    }

    expect(vaultInfo).to.be.null;
    expect(userAfter).to.be.greaterThan(userBefore); // Got remaining lamports
  });
});
