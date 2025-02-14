use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod contexts;

pub use contexts::*;

declare_id!("6vXqLyZ4U9e3d1M5Xn8atqdWdHqpyyUx2MQ3kZHC7X9e");

#[program]
pub mod amm_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64,fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, lp_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(lp_amount, max_x, max_y)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_amount, min_x, min_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
        ctx.accounts.swap(args)?;
        Ok(())
    }
}
