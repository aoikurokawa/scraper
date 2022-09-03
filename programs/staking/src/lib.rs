use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let stake_amount = amount;

        // let cpi_ctx = CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     token::MinTo {
        //         mint: ctx.accounts.stake_mint.to_account_info(),
        //         authority: ,
        //         to: ,
        //     },
        //     &signer,
        // );
        // token::mint_to(cpi_ctx, stake_amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    pub token_program: Account<'info, Token>,
    pub stake_mint: Account<'info, Mint>,
}
