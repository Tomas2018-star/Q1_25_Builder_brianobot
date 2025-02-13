use anchor_lang::prelude::*;

pub mod error;
pub mod state;
pub mod contexts;

declare_id!("EJq6mSWxKgnQFP9NBQzJ6Ngk3bK7CC66oGAwCG3wAvrN");

#[program]
pub mod staking {
    use contexts::RegisterUser;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
