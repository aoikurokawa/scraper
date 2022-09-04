use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("BusWxEkAkWkBm9Dpfv2eLoFNTuRzpKKWaC2QQAg7bRUW");

#[program]
pub mod staking {
    pub const STAKE_MINT_ADDRESS: &str = "J4Lnzdm6yioomASsTUbFMZh1SpRe9QSBTx8J4Gs8yWzw";
    pub const BEEF_MINT_ADDRESS: &str = "88vGScBgunKkmKcXyg65kiJ8XnZ9VvXs3xYwSnB4Nwia";

    use super::*;

    pub fn create_beef_token_bag(_ctx: Context<CreateBeefTokenBag>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, stake_mint_authority_bump: u8, program_beef_bag_bump: u8, amount: u64) -> Result<()> {
        let stake_amount = amount;

        let stake_mint_address = ctx.accounts.stake_mint.key();
        let seeds = &[stake_mint_address.as_ref(), &[stake_mint_authority_bump]];
        let signer = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.stake_mint.to_account_info(),
                authority: ctx.accounts.stake_mint_authority.to_account_info(),
                to: ctx.accounts.user_stake_token_bag.to_account_info(),
            },
            &signer,
        );
        token::mint_to(cpi_ctx, stake_amount)?;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_beef_token_bag.to_account_info(),
                to: ctx.accounts.program_beef_token_bag.to_account_info(),
                authority: ctx.accounts.user_beef_token_bag_authority.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, stake_amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateBeefTokenBag<'info> {
    #[account(
        init,
        payer = payer,

        // We use the token mint as as seed fo the mapping -> think "HashMap[seeds+bump] = pda" 
        seeds = [BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap().as_ref()],
        bump,

        // Token Program want to know that what kind of token this token bag is for
        token::mint = beef_mint,

        // It's a PDA so the authority is itself!
        token::authority = program_beef_token_bag,
    )]
    pub program_beef_token_bag: Account<'info, TokenAccount>,
    #[account(
        address = BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    pub beef_mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(stake_mint_authority_bump: u8, program_beef_bag_bump: u8 )]
pub struct Stake<'info> {
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub user_beef_token_bag: Account<'info, TokenAccount>,
    pub user_beef_token_bag_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [beef_mint.key().as_ref()],
        bump = program_beef_bag_bump,
    )]
    pub program_beef_token_bag: Account<'info, TokenAccount>,
    #[account(
        address = BEEF_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    pub beef_mint: Account<'info, Mint>,
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
    #[account(mut)]
    pub user_stake_token_bag: Account<'info, TokenAccount>,
}
