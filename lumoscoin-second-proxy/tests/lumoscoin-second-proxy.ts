import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LumoscoinSecondProxy } from "../target/types/lumoscoin_second_proxy";

describe("lumoscoin-second-proxy", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LumoscoinSecondProxy as Program<LumoscoinSecondProxy>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
