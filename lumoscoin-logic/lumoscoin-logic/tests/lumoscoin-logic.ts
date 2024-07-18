import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LumoscoinLogic } from "../target/types/lumoscoin_logic";

describe("lumoscoin-logic", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LumoscoinLogic as Program<LumoscoinLogic>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
