use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )] // TODO: why are SystemAccount not initable from inside anchor
    // ANS: SystemAccount are expected to be owned by the system program and alreadt exist
    // Unlike the Account<T>, which needs initialization to initialize the data field of the underlying solana account
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.house.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}