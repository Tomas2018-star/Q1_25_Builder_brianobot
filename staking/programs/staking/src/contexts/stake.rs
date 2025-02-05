use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::{UserAccountState, StakeAccountState, StakeConfig};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = 
    )]
    pub nft_mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>,
    
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"stake_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn init(&mut self, points: u32, amount_staked: u8, bumps: &StakeBumps) -> Result<()> {
        Ok(())
    }
}