use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;

declare_id!("EJq6mSWxKgnQFP9NBQzJ6Ngk3bK7CC66oGAwCG3wAvrN");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
