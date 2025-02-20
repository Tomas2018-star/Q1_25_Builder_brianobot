use anchor_lang::prelude::*;

pub mod state;
pub mod contexts;
pub mod error;

pub use contexts::*;

declare_id!("DarLrGUYijwGz7M7R21NYSgf884STqWaedunKuL6AhtA");

#[program]
pub mod dice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}
