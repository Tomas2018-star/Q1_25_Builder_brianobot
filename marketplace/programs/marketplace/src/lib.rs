use anchor_lang::prelude::*;

pub mod state;
pub mod error;
pub mod contexts;

pub use contexts::*;


declare_id!("F1ARGgzeMbriizXy4x2XiJ1r3sGS8RYmXhQm1iVySdpp");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_listing()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?; 
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault_account()?;
        ctx.accounts.reward_buyer()?;
        Ok(()) // i am assuming this is the end you are referring to??

    }
}
