use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let stake_amount = amount;
    }
}

#[derive(Accounts)]
pub struct Initialize {}
