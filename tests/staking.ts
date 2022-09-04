import * as anchor from "@project-serum/anchor";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { airdropBeefToken } from "../scripts/airdrop_beef";
import {
  program,
  stakeMintAddress,
  findStakeMintAuthorityPDA,
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
    const [beefPDA, _] = await getProgramBeefTokenBagPDA();

    await program.methods
      .createBeefTokenBag()
      .accounts({
        beefMint: beefMintAddress,
        programBeefTokenBag: beefPDA,
        payer: user.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .rpc();

    const tokenHelper = new TokenHelper(beefMintAddress);
    expect(await tokenHelper.balance(beefPDA)).to.be.eql(0);
  });

  it("Stake $beef for $stake", async () => {
    // 0. Prepare Token Bags
    const user = new User();
    await user.getOrCreateBeefTokenBag();
    await user.getOrCreateStakeTokenBag();
    const userStakes = await user.stakeBalance();
    const userBeefs = await user.beefBalance();

    const [stakePDA, stakePDABump] = await findStakeMintAuthorityPDA();
    const [beefBagPDA, beefBagBump] = await getProgramBeefTokenBagPDA();

    // 2. Execute our stuff
    await program.methods
      .stake(stakePDABump, beefBagBump, new anchor.BN(5_000))
      .accounts({
        // Solana is lost: where are my spl program friends?
        tokenProgram: TOKEN_PROGRAM_ID,
        userBeefTokenBag: user.beefTokenBag,
        userBeefTokenBagAuthority: user.wallet.publicKey,
        programBeefTokenBag: beefBagPDA,
        beefMint: beefMintAddress,
        // Token Program asks: what type of token am I supposed to print?
        stakeMint: stakeMintAddress,
        // Token Program asks:
        // who is allowed to print tokens from stakeMint?
        stakeMintAuthority: stakePDA,
        // Token Program wonders: "where should I mint this to?"
        userStakeTokenBag: user.stakeTokenBag,
      })
      .rpc();

    expect(await user.stakeBalance()).to.be.equal(userStakes + 5_000);
    expect(await user.beefBalance()).to.be.equal(userBeefs - 5_000);
    const tokenHelper = new TokenHelper(beefMintAddress);
    expect(await tokenHelper.balance(beefBagPDA)).to.be.equal(5_000);
  });
});

export const getProgramBeefTokenBagPDA = async (): Promise<
  [PublicKey, number]
> => {
  const seed = beefMintAddress;

  return await PublicKey.findProgramAddress(
    [seed.toBuffer()],
    program.programId
  );
};
