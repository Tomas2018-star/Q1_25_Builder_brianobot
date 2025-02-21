use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod contexts;

pub use contexts::*;

declare_id!("EJq6mSWxKgnQFP9NBQzJ6Ngk3bK7CC66oGAwCG3wAvrN");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }
}


