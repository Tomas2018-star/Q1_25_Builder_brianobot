use anchor_lang::prelude::*;

use crate::state::Bet;


#[derive(Accounts)]
#[instructions(seed: u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: SystemAccount<'info>,
    #[account(
        init, 
        payer = player,
        // in solana le_bytes is the standard
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        space = Bet::InitSpace + 8,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}


impl<'info> PlaceBet<'info> {
    pub fn create_bet(&mut self) -> Result<()> {
        // self.bet

        Ok(())
    }
}