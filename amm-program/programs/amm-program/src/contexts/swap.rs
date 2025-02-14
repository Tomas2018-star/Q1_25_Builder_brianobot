use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, mint, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::state::Config;
use crate::error::AmmError;


#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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

    // #[account(address = config.mint_x)] # this is not neccessary since we use the has_one check on the config
    pub mint_x: InterfaceAccount<'info, Mint>, // one part of the pair to be used for the exchange
    // #[account(address = config.mint_y)] # this is not neccessary since we use the has_one check on the config
    pub mint_y: InterfaceAccount<'info, Mint>,  // other part of the pair neeeded for the exchange
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
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_ata_y: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
pub struct SwapArgs {
    is_x: bool,
    amount: u64, 
    min: u64,
}

impl<'info> Swap<'info> {
    pub fn swap(&mut self, args: SwapArgs) -> Result<()> {
        require!(args.amount > 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.vault_x.amount,
            self.config.fee,
            None,
        ).unwrap();

        let p = match args.is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, args.amount, args.min).unwrap();

        require_neq!(res.deposit, 0, AmmError::InvalidAmount);
        require_neq!(res.withdraw, 0, AmmError::InvalidAmount);
        
        // Transfer to Vault
        self.transfer_to_vault()?;
        
        // Transfer The Receiving Token from Vault to the Usee Ata
        self.withdraw_from_vault()?;

        Ok(())
    }

    fn transfer_to_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimaks) = match args.is_x {
            true => ( TransferChecked {
                from: self.user_ata_x.to_account_info(),
                mint: self.mint_x.to_account_info(),
                to: self.vault_x.to_account_info(),
                authority: self.user.to_account_info(),
            }, self.mint_x.decimals),
            false => ( TransferChecked {
                from: self.user_ata_x.to_account_info(),
                mint: self.mint_x.to_account_info(),
                to: self.vault_x.to_account_info(),
                authority: self.user.to_account_info(),
            }, self.mint_x.decimals),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, res.deposit, mint_decimaks)?;

        Ok(())
    }

    fn withdraw_from_vault(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match args.is_x {
            true => (TransferChecked {
                from: self.vault_y.to_account_info(),
                mint: self.mint_y.to_account_info(),
                to: self.user_ata_y.to_account_info(),
                authority: self.config.to_account_info(),
            }, self.mint_x.decimals),

            false => (TransferChecked {
                from: self.vault_x.to_account_info(),
                mint: self.mint_x.to_account_info(),
                to: self.user_ata_x.to_account_info(),
                authority: self.config.to_account_info(),
            }, self.mint_y.decimals),
        };


        let config_bump = self.config.seed.to_le_bytes();

        let seeds = [
            b"config", 
            self.mint_x.to_account_info().key.as_ref(),
            self.mint_y.to_account_info().key.as_ref(),
            config_bump.as_ref(),
            &[self.config.config_bump],
        ];

        let signer_seeds =  &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, res.withdraw, mint_decimals)?;

        Ok(())
    }
}