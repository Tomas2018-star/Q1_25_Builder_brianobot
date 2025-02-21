use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{error::DiceError, state::Bet};

const REFUND_COOLDOWN_SLOTS: usize = 1000000;


#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: SystemAccount<'info>,
    #[account(
        mut,
        // in solana le_bytes is the standard
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}


impl<'info> RefundBet<'info> {
    pub fn withdraw_from_vault(&mut self, amount: u64, bumps: &RefundBetBumps) -> Result<()> {
        let clock = Clock::get()?;

        require_gt!(
            (clock.slot - self.bet.slot) as usize,
            REFUND_COOLDOWN_SLOTS,
            DiceError::RefundCooldownNotElapsed
        );

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let seeds = [
            b"vault", 
            self.house.to_account_info().key.as_ref(),
            &[bumps.vault],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}