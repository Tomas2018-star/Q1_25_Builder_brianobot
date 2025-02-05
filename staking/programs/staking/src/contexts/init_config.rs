use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::StakeConfig;


#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = StakeConfig::INIT_SPACE + 8,
        seeds = [b"stake_config"],
        bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()], // the types of seeds must be a reference to an array of bytes
        bump,
        // this is needed when the pda is not a standard pda but a mint pda
        mint::authority = config, // 
        mint::decimals = 6,
    )]
    pub rewards_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
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