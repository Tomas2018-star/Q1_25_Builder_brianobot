use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::StakeConfig;

/*
The Config is the configuration for the whole staking contract (Universal). 
*/

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE + 8,
        seeds = [b"stake_config"], // having 1 seed means we can only have one PDA for this account from a program
        bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = admin,
        // space is not needed for Mint Account even when they are initED
        seeds = [b"rewards", config.key().as_ref()], // the types of seeds must be a reference to an array of bytes
        bump,
        // this is needed when the pda is not a standard pda but a mint pda
        // when init is provided for Mint Account they are created with the 
        // mint constraint provided in the account attribute
        mint::authority = config, // look into multisigs
        mint::decimals = 6,
    )]
    // this simply enforces that the Account passed here must implement the base interface
    // InterfaceAccount checks for ownership and handle deserialization
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>, // needed because the context would need the system program to create account
    // use Interface when checking that the account is one of a set of given Program
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init(&mut self, points_per_stake: u8, max_stake: u8, freeze_period: u32, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner( StakeConfig {
            points_per_stake,
            max_stake,
            freeze_period,
            rewards_bump: bumps.rewards_mint,
            bump: bumps.config,
        });
        
        Ok(())
    }
}