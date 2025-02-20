use anchor_lang::prelude::*;

use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,  
};

use crate::state::Bet;
use crate::error::DiceError;


#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(mut)]
    pub player: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info,  Bet>,
    #[account( 
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: this is the instructions sysvar acccount
    pub instructions_sysvar: AccountInfo<'info>,
    // this sysvar allows for instruction introspection
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instructions_sysvar.to_account_info())?;

        require_keys_eq!(ix.program_id, ed25519_program::ID, DiceError::CustomError);



        Ok(())
    }
}