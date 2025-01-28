pub mod state;
pub mod contexts;
pub mod instructions;

pub use state::*;
pub use contexts::*;
pub use instruction::*;

use anchor_lang::prelude::*;

declare_id!("F1ARGgzeMbriizXy4x2XiJ1r3sGS8RYmXhQm1iVySdpp");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
