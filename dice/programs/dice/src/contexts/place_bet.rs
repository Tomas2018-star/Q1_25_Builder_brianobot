use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::Bet;


#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct PlaceBet<'info> {
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
        init,
        payer = player,
        space = Bet::INIT_SPACE + 8,
        // since the vault key is derived from the house key
        // using the vault key here is the same as deriving from the house key
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}


impl<'info> PlaceBet<'info> {
    pub fn create_bet(&mut self, seed: u128, roll: u8, amount: u64, bumps: &PlaceBetBumps) -> Result<()> {
        let clock = Clock::get().unwrap();

        self.bet.set_inner( Bet {
            player: self.player.key(),
            seed,
            slot: clock.slot,
            // the slot is specific to solana, 
            amount, // the amount they are willing to risk
            roll, // what they are betting on
            bump: bumps.bet,
        });

        Ok(())
    }

    pub fn transfer_to_vault(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}