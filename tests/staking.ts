import * as anchor from "@project-serum/anchor";
import { SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { airdropBeefToken } from "../scripts/airdrop_beef";
import {
  program,
  stakeMintAddress,
  findStakeMintAuthorityPDA,
  findBeefMintAuthorityPDA,
  beefMintAddress,
} from "../scripts/config";
import { createMints } from "../scripts/create_mints";
import { User } from "./user";
import { TokenHelper } from "./token_helper";
import { expect } from "chai";

describe("staking", () => {
  before(async () => {
    await createMints();
    await airdropBeefToken();
  });

  it("It creates the program beef token bag", async () => {
    const user = new User();
    const [beefPDA, _] = await findBeefMintAuthorityPDA();

    await program.methods.createBeefTokenBag().accounts({
      beefMint: beefMintAddress,
      programBeefTokenBag: beefPDA,
      payer: user.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    });

    const tokenHelper = new TokenHelper(beefMintAddress);
    expect(await tokenHelper.balance(beefPDA)).to.be.eql(0);
  });

  it("Stake $beef for $stake", async () => {
    // 0. stakeMintAuthority = PDA with stakeMint as seed
    const [stakePDA, stakePDABump] = await findStakeMintAuthorityPDA();

    // 1. Prepare Token Bags
    const user = new User();
    await user.getOrCreateStakeTokenBag();

    // 2. Get current stake amount
    const userStakes = await user.stakeBalance();

    // 3. STAKE
    await program.methods
      .stake(stakePDABump, new anchor.BN(5_000))
      .accounts({
        // Solana is lost: where are my spl program friends?
        tokenProgram: TOKEN_PROGRAM_ID,
        // Token Program asks: what type of token am I supposed to print?
        stakeMint: stakeMintAddress,
        // Token Program asks:
        // who is allowed to print tokens from stakeMint?
        stakeMintAuthority: stakePDA,
        // Token Program wonders: "where should I mint this to?"
        userStakeTokenBag: user.stakeTokenBag,
      })
      .rpc();
  });
});
