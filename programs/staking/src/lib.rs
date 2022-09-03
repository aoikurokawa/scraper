use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token};

declare_id!("BusWxEkAkWkBm9Dpfv2eLoFNTuRzpKKWaC2QQAg7bRUW");
pub const STAKE_MINT_ADDRESS: &str = "J4Lnzdm6yioomASsTUbFMZh1SpRe9QSBTx8J4Gs8yWzw";

#[program]
pub mod staking {
    use super::*;


    pub fn stake(ctx: Context<Stake>, stake_mint_authority_bump: u8,amount: u64) -> Result<()> {
        let stake_amount = amount;
        
        let stake_mint_address = ctx.accounts.stake_mint.key();
        let seeds = &[stake_mint_address.as_ref(), &[stake_mint_authority_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MinTo {
                mint: ctx.accounts.stake_mint.to_account_info(),
                authority: ctx.accounts.stake_mint_authority.to_account_info(),
                to: ,
            },
            &signer,
        );
        token::mint_to(cpi_ctx, stake_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(stake_mint_authority_bump: u8)]
pub struct Stake<'info> {
    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        address = STAKE_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    pub stake_mint: Account<'info, Mint>,
    /// CHECK: only used as a signing PDA
    #[account(
        seeds = [stake_mint.key().as_ref()],
        bump = stake_mint_authority_bump
    )]
    pub stake_mint_authority: UncheckedAccount<'info>,
}
