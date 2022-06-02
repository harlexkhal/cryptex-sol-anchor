import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CryptexSolAnchor } from "../target/types/cryptex_sol_anchor";

describe("cryptex-sol-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CryptexSolAnchor as Program<CryptexSolAnchor>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
