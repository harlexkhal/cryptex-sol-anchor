import { TOKEN_PROGRAM_ID, transferInstructionData } from "@solana/spl-token";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CryptexSolAnchor } from "../target/types/cryptex_sol_anchor";
import { PublicKey } from "@solana/web3.js";
import {
  getKeypair,
  getProgramId,
  getPublicKey,
} from "./utils";
import { BN } from "bn.js";
import { assert } from "chai";

describe("Cryptex Dapp Test", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.CryptexSolAnchor as Program<CryptexSolAnchor>;

  it("Test stake", async () => {
    const keypairUser = getKeypair("main");
    const usdcAcctUser = getPublicKey("usdc");
    const cryptex_usdcAcctUser = getPublicKey("cusdc");
    const programId = getProgramId();

    await program.methods.stake(new BN(1000000)).accounts({
        signer: keypairUser.publicKey,
        destinationPubkey: new PublicKey('ACqqDBXdFhgatszRESwmdkfgLH7coJm7SxaTuiEhEQ9y'),
        sourcePubkey: usdcAcctUser,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([keypairUser])
      .rpc();
      assert.ok(true);
  });

  it("Test mint", async () => {
    const keypairUser = getKeypair("main");
    const cryptex_usdcAcctUser = getPublicKey("cusdc");

    await program.methods.mint(new BN(1000000)).accounts({
        signer: keypairUser.publicKey,
        mintTokenPubkey: new PublicKey('6zdV6NKr7JnnyFxwGyBjgD3N8sJrR9rM5nmqUw7msrS'),
        destinationPubkey: cryptex_usdcAcctUser,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([keypairUser])
      .rpc();
      assert.ok(true);
  });
});