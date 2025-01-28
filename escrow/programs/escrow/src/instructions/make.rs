use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, mint, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::state::EscrowState;


#[derive(Accounts)]
#[instruction(seeds: u8)] // this means i would pass a type u8 to the seeds field inside the instruction
// when using multiple instructions params, keep them in the same order in the instruction as provided here
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    // an InterfaceAccount is a wrapper around an AccountInfo that provides
    // a more ergonomic interface for interacting with the account
    // it is used to interact with the associated token account
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a, // ? how does this access works associated_token::mint = mint_a
        associated_token::authority = maker,
    )]
    // the assumption here is that the maker has already created the associated token account
    // since they want to exchange token a for token b they must already have an ATA to store token a
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,  
    #[account(
        init,
        payer = maker,
        space = 8 + EscrowState::INIT_SPACE,
        seeds = [b"escrow", maker.key.as_ref(), seeds.to_le_bytes().as_ref() ],
        bump,
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
    )]
    // this ATA would hold the token received from maker of the escrow
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token: Program<'info, AssociatedToken>,
}