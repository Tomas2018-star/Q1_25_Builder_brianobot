#![allow(unused_imports)]

pub mod state;
pub mod error;
pub mod contexts;
pub mod instructions;

pub use state::*;
pub use error::*;
pub use contexts::*;
pub use instruction::*;

use anchor_lang::prelude::*;

declare_id!("F1ARGgzeMbriizXy4x2XiJ1r3sGS8RYmXhQm1iVySdpp");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }
}
