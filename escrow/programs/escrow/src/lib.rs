pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use self::instructions::*;


declare_id!("ASnvhxNh4U9fwETxahiCCGa18LXjRNUDfvJnNAC5tRyg");


#[program]
pub mod escrow {
    use super::*; // this brings everthing from the parent scope into the escrow mod scope

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.make(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(receive_amount)? ;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close()?;
        Ok(())
    }

    // taker wants to swap token b for token a
    // you do not have to store them in a vault like you did for token a
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.release()?;
        ctx.accounts.close()?;
        Ok(())
    }
}

