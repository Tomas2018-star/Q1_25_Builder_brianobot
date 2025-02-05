#![allow(unused_imports)]

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer_checked, MintTo, mint_to, Token, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use constant_product_curve::ConstantProduct;

use crate::state::Config;
use crate::error::AmmError;


#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub lp_provider : Signer<'info>, // the account providing liquidity for the market 
    pub mint_x: InterfaceAccount<'info, Mint>, // one part of the pair to be used for the exchange
    pub mint_y: InterfaceAccount<'info, Mint>,  // other part of the pair neeeded for the exchange
    #[account(
        has_one = mint_x,
        has_one = mint_y,
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
        mint::decimals = 6,
        mint::authority = config
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>, // the mint account of the mint to be used to mint tokens for the user for 
    // participating in the marketplace by providing liquidity
    #[account(
        mut,
        associated_token::mint = mint_x,
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
        mut,
        associated_token::authority = lp_provider,
        associated_token::mint = mint_y,
    )]
    pub lp_provider_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = lp_provider,
        associated_token::authority = lp_provider,
        associated_token::mint = mint_lp,
    )]
    pub lp_provider_ata_lp: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposit<'info> {
    pub fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        require!(amount > 0, AmmError::InvalidAmount);
        require!(!self.config.locked, AmmError::AMMLocked);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = match is_x {
            true => TransferChecked {
                    from: self.lp_provider_ata_x.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                    to: self.vault_x.to_account_info(),
                    authority: self.lp_provider.to_account_info(),
                },
            false => TransferChecked {
                    from: self.lp_provider_ata_y.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                    to: self.vault_y.to_account_info(),
                    authority: self.lp_provider.to_account_info(),
                },
        };
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        let mint_decimals = match is_x {
            true => self.mint_x.decimals,
            false => self.mint_y.decimals,
        };

        transfer_checked(cpi_ctx, amount, mint_decimals)?;

        Ok(())
    }

    pub fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.lp_provider_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let mint_y = self.mint_y.key().to_bytes();
        let mint_x = self.mint_x.key().to_bytes();
        let seed = self.config.seed.to_le_bytes();

        let seeds = [b"config", mint_x.as_ref(), mint_y.as_ref(), seed.as_ref()];        

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;
        
        Ok(())
    }
}