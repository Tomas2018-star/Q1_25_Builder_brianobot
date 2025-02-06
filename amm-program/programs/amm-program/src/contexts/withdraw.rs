use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer_checked, MintTo, burn, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use constant_product_curve::ConstantProduct;

use crate::state::Config;
use crate::error::AmmError;


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub lp_provider : Signer<'info>, // the account providing liquidity for the market 
    // #[account(address = config.mint_x)] # this is not neccessary since we use the has_one check on the config
    pub mint_x: InterfaceAccount<'info, Mint>, // one part of the pair to be used for the exchange
    // #[account(address = config.mint_y)] # this is not neccessary since we use the has_one check on the config
    pub mint_y: InterfaceAccount<'info, Mint>,  // other part of the pair neeeded for the exchange
    #[account(
        has_one = mint_x, // checks config.mint_x == mint_x
        has_one = mint_y, // checks config.mint_y == mint_y
        seeds = [
            b"config", 
            mint_x.key().to_bytes().as_ref(),
            mint_y.key().to_bytes().as_ref(),
            config.seed.to_le_bytes().as_ref()    
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6, // ensures that the mint account provided here matches the same one initialized in the init step
        mint::authority = config // ensures that the authority of the mint account is the config
    )]
    // mint::authority maps to the mint_authority field in the Mint struct. Anchor uses mint::authority = config as a 
    // more readable and intuitive way to enforce mint_lp.mint_authority == Some(config.key())
    pub mint_lp: InterfaceAccount<'info, Mint>, // the mint account of the mint to be used to mint tokens for the user for 
    // participating in the marketplace by providing liquidity
    #[account(
        mut,
        associated_token::mint = mint_x, // vault_x.mint == mint_x.key() this is the full form
        associated_token::authority = config,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority = lp_provider,
        associated_token::mint = mint_x,
    )]
    pub lp_provider_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, // this is mut and not init, because we are working with the assumption that a user that wants to trade a pair
        // must already have ata for those pair
        associated_token::authority = lp_provider,
        associated_token::mint = mint_y,
    )]
    pub lp_provider_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = lp_provider,
        associated_token::authority = lp_provider, // this checks that the lp_provider_ata_lp.owner = lp_provider.key()
        associated_token::mint = mint_lp, // this checks that the lp_provider_ata_lp.mint = mint_lp.key()
    )]
    pub lp_provider_ata_lp: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}