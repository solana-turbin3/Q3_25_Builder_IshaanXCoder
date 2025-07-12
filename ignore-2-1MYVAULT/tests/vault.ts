import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.vault as Program<Vault>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);

    // Fetch the account to check if it was initialized (example)
    // const vaultAccount = await program.account.vault.fetch(<vault public key>);
    // console.log("Vault account:", vaultAccount);
    // Add assertions here, e.g.:
    // assert.ok(vaultAccount.isInitialized);
  });
});
