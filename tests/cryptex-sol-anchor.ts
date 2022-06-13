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

  /*it("Make PDA take control of fluidity's ${Usdc} token account", async () => {
    const keypairUser = getKeypair("main");
    const usdcAcctUser = getPublicKey("usdc");
    const cryptex_usdcAcctUser = getPublicKey("cusdc");
    const programId = getProgramId();

    const current_auth_keypair = getKeypair("mint_auth");
    
    const assumed_fluidity_usdc_account = new PublicKey('ACqqDBXdFhgatszRESwmdkfgLH7coJm7SxaTuiEhEQ9y')

    await program.methods.assignAuthorityToPda().accounts({
        currentAuthoritySigner: current_auth_keypair.publicKey,
        acctOrMintPubkey: assumed_fluidity_usdc_account,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([current_auth_keypair])
      .rpc();
      assert.ok(true);
  });*/

  it("Wrap Token", async () => {
    const keypairUser = getKeypair("main");
    const usdcAcctUser = getPublicKey("usdc");
    const cryptex_usdcAcctUser = getPublicKey("cusdc");
    const programId = getProgramId();

    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from("cryptex")],
      programId
    );
    await program.methods.wrap(new BN(10)).accounts({
        signer: keypairUser.publicKey,
        transferToPubkey: new PublicKey('ACqqDBXdFhgatszRESwmdkfgLH7coJm7SxaTuiEhEQ9y'),
        ownerPubkey: usdcAcctUser,
        mintPubkey: new PublicKey('6zdV6NKr7JnnyFxwGyBjgD3N8sJrR9rM5nmqUw7msrS'),
        mintToPubkey: cryptex_usdcAcctUser,
        pdaAccountPubkey: PDA[0],
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([keypairUser])
      .rpc();
      assert.ok(true);
  });

  it("UnWrap Token", async () => {
    const keypairUser = getKeypair("main");
    const usdcAcctUser = getPublicKey("usdc");
    const cryptex_usdcAcctUser = getPublicKey("cusdc");
    const programId = getProgramId();

    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from("cryptex")],
      programId
    );

    await program.methods.unwrap(new BN(10)).accounts({
      signer: keypairUser.publicKey,
      transferToPubkey: usdcAcctUser,
      ownerPubkey: new PublicKey('ACqqDBXdFhgatszRESwmdkfgLH7coJm7SxaTuiEhEQ9y'),
      mintPubkey: new PublicKey('6zdV6NKr7JnnyFxwGyBjgD3N8sJrR9rM5nmqUw7msrS'),
      burnFrom: cryptex_usdcAcctUser,
      pdaAccountPubkey: PDA[0],
      tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([keypairUser])
      .rpc();
      assert.ok(true);
  });

});