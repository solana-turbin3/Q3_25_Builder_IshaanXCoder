import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SatoshiDice } from "../target/types/satoshi_dice";

describe("satoshi_dice", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.satoshiDice as Program<SatoshiDice>;

  it("Say Hello!", async () => {
    const tx = await program.methods.sayHello().rpc();
    console.log("Your transaction signature", tx);
  });
});
