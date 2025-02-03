use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;

pub use contexts::*;

declare_id!("6vXqLyZ4U9e3d1M5Xn8atqdWdHqpyyUx2MQ3kZHC7X9e");

#[program]
pub mod amm_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
