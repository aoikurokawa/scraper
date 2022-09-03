import * as anchor from "@project-serum/anchor";
import {TOKEN_PROGRAM_ID} from "@solana/spl-token";
import {program, stakeMintAddress, findStakeMintAuthorityPDA} from "../scripts/config";
import {createMints} from "../scripts/create_mints";

describe("staking", () => {
  
  before(async () => {
    await createMints();
  })

  it("Stake $beef for $stake", async () => {
    const [stakePDA, stakePDABump] = await findStakeMintAuthorityPDA();

    await program.methods
      .stake(stakePDABump, new anchor.BN(5_000))
      .accounts(
        {
          tokenProgram: TOKEN_PROGRAM_ID, 
          stakeMint: stakeMintAddress,
          stakeMintAuthority: stakePDA,
        }
      )
      .rpc();
  });
});
