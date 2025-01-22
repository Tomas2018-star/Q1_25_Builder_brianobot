#![allow(unused_imports)]
use anchor_lang::prelude::*;

declare_id!("HNjhjVf3ashYdQDqZjwmgMBQJgquLDxHpWdCHzRmQiiy");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize()?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init, 
        payer = signer, 
        space = VaultState::INIT_SPACE + 8,
        seeds = [b"state", signer.key().as_ref()],
        bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>, // represent any account owned by the system program
    // this include most wallets (keypairs)
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"state", signer.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
    

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, _amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(&mut self, _amount: u64) -> Result<()> {
        Ok(())
    }
} 