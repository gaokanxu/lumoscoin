import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LumoscoinFirstProxy } from "../target/types/lumoscoin_first_proxy";

describe("lumoscoin-first-proxy", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LumoscoinFirstProxy as Program<LumoscoinFirstProxy>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
