use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};


#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init, 
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(), // this ensures that the derived PDA is not derived with the current program key
        // default the seeds_program = crate::ID, which points the current program
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref() @MarketplaceError::,
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program == metadata_program.key(),
        bump,
    )]
    // this is the metadata account of the NFT
    pub metadata: Account<'info, MetadataAccount>,
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let seeds = [

        ];

        let signer_seeds = None;

        // let accounts = TransferChecked 

        Ok(())

    }
 
    pub fn close_listing(&mut self) -> Result<()> {
        Ok(())
    }
}