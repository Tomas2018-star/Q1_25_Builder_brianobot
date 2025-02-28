use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};
use constant_product_curve::{ConstantProduct, LiquidityPair, SwapResult};

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
    #[account(
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
        mint::decimals = 6, // ensures that the mint account provided here matches the same one initialized in the init step
        mint::authority = config // ensures that the authority of the mint account is the config
    )]
    // mint::authority maps to the mint_authority field in the Mint struct. Anchor uses mint::authority = config as a 
    // more readable and intuitive way to enforce mint_lp.mint_authority == Some(config.key())
    pub mint_lp: InterfaceAccount<'info, Mint>, // the mint account of the mint to be used to mint tokens for the user for 
    // #[account(address = config.mint_x)] # this is not neccessary since we use the has_one check on the config
    pub mint_x: InterfaceAccount<'info, Mint>, // one part of the pair to be used for the exchange
    // #[account(address = config.mint_y)] # this is not neccessary since we use the has_one check on the config
    pub mint_y: InterfaceAccount<'info, Mint>,  // other part of the pair neeeded for the exchange
    #[account(
        mut,
        associated_token::mint = mint_x, // vault_x.mint == mint_x.key() this is the full form
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
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
        associated_token::token_program = token_program,
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
        require!(self.config.locked == false, AmmError::AMMLocked);

        let mut curve = ConstantProduct::init(
            self.vault_x.amount, // number of token x
            self.vault_y.amount, // number of token y
            self.mint_lp.supply, // number of lp token
            self.config.fee, 
            None,
        ).unwrap();

        let p = match args.is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let res = curve.swap(p, args.amount, args.min).map_err(AmmError::from)?;

        require_neq!(res.deposit, 0, AmmError::InvalidAmount);
        require_neq!(res.withdraw, 0, AmmError::InvalidAmount);

        let res2 = SwapResult {
            deposit: res.deposit.clone(),
            withdraw: res.withdraw.clone(),
            fee: res.fee.clone(),
        };
        
        // Transfer to Vault
        self.transfer_to_vault(args.clone(), res)?;
        
        // Transfer The Receiving Token from Vault to the Usee Ata
        self.withdraw_from_vault(args, res2)?;

        Ok(())
    }

    fn transfer_to_vault(&mut self, args: SwapArgs, res: SwapResult) -> Result<()> {
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

    fn withdraw_from_vault(&mut self, args: SwapArgs, res: SwapResult) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match args.is_x {
            true => (TransferChecked {
                from: self.vault_y.to_account_info(),
                mint: self.mint_y.to_account_info(),
                to: self.user_ata_y.to_account_info(),
                authority: self.config.to_account_info(),
            }, self.mint_y.decimals),

            false => (TransferChecked {
                from: self.vault_x.to_account_info(),
                mint: self.mint_x.to_account_info(),
                to: self.user_ata_x.to_account_info(),
                authority: self.config.to_account_info(),
            }, self.mint_x.decimals),
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