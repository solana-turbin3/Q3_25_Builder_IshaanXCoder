import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Marketplace } from "../target/types/marketplace";

describe("marketplace", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.marketplace as Program<Marketplace>;

  it("Say Hello!", async () => {
    const tx = await program.methods.sayHello().rpc();
    console.log("Your transaction signature", tx);
  });
});
