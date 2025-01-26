#![allow(unused_imports)]
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;


declare_id!("HNjhjVf3ashYdQDqZjwmgMBQJgquLDxHpWdCHzRmQiiy");

#[program]
pub mod vault {
    use super::*;

    // anchor packages the account struct and put it in the accounts attribute of the context
    // anchor also stores all the bump(s) values in a struct called <AccountStructName>Bumps on the context too
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
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
#[derive(InitSpace)] // needed to automatically determine the size of the account
// it creates a Constants (INIT_SPACE ) that holds the size of the account
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)] // this marked as mutable because the balance of signer would be decreased
    pub signer: Signer<'info>, // represent the account that would sign the transaction
    // we would almost always need the signer
    #[account(
        init, 
        payer = signer, 
        space = 8 + VaultState::INIT_SPACE ,
        seeds = [b"state", signer.key().as_ref()], // &str -> bytes
        bump,
    )]
    // basically, Account type accounts allow us to defined a struct to store data for that account
    // this data would be stored in the native solana account's data attribute, and when loaded by Anchor
    // would be automatically deserialized into the struct

    // the Account field also ensures that the account provided is owned by the program
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>, // represent any account owned by the system program
    // this include most wallets (keypairs)
    pub system_program: Program<'info, System>, // needed because an account creation happens and this is done by the system program
}

impl<'info> Initialize<'info> {
    // anchor creates a <AccountStructName>Bumps struct that holds the bump values for the seeds
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        // this was the route followed in the class
        // self.vault_state.state_bump = bumps.vault_state;
        // self.vault_state.vault_bump = bumps.vault;

        // following the set_inner patterns feels cleaner and more idiomatic
        // set_inner is defined in the impl block of the Account type
        self.vault_state.set_inner( VaultState {
            vault_bump: bumps.vault,
            state_bump: bumps.vault_state,
        });
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
        seeds = [vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
    
impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // we need the account info of the system program to use in a CPI call
        let system_program = self.system_program.to_account_info();

        // the accounts needed for the CPI, the Transfer struct is an Account struct for account validation
        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };
 
        let cpi_ctx = CpiContext::new(system_program, accounts);

        transfer(cpi_ctx,  amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, _amount: u64) -> Result<()> {
        let system_program = self.system_program.to_account_info();

        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(system_program, accounts);
        
        transfer(cpi_ctx, _amount)?;

        Ok(())
    }
} 