import * as anchor from "@project-serum/anchor";
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {program, stakeMintAddress, findStakeMintAuthorityPDA} from "../scripts/config";
import {createMints} from "../scripts/create_mints";
import {User} from "./user";

describe("staking", () => {
  
  before(async () => {
    await createMints();
  })

  it("Stake $beef for $stake", async () => {
    // 0. stakeMintAuthority = PDA with stakeMint as seed
    const [stakePDA, stakePDABump] = await findStakeMintAuthorityPDA();
    
    // 1. Prepare Token Bags
    const user = new User;
    await user.getOrCreateStakeTokenBag();
    
    // 2. Get current stake amount
    const userStakes = await user.stakeBalance();

    // 3. STAKE
    await program.methods
      .stake(stakePDABump, new anchor.BN(5_000))
      .accounts(
        {
          // Solana is lost: where are my spl program friends?
          tokenProgram: TOKEN_PROGRAM_ID, 
          // Token Program asks: what type of token am I supposed to print?
          stakeMint: stakeMintAddress,
          // Token Program asks: 
          // who is allowed to print tokens from stakeMint?
          stakeMintAuthority: stakePDA,
          // Token Program wonders: "where should I mint this to?"
          userStakeTokenBag: user.stakeTokenBag,
        }
      )
      .rpc();
  });
});
