use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{burn, transfer_checked, Burn, TransferChecked};
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
        mut,
        associated_token::authority = lp_provider,
        associated_token::mint = mint_x,
        associated_token::token_program = token_program,
    )]
    pub lp_provider_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut, // this is mut and not init, because we are working with the assumption that a user that wants to trade a pair
        // must already have ata for those pair
        associated_token::authority = lp_provider,
        associated_token::mint = mint_y,
        associated_token::token_program = token_program,
    )]
    pub lp_provider_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = lp_provider,
        associated_token::authority = lp_provider, // this checks that the lp_provider_ata_lp.owner = lp_provider.key()
        associated_token::mint = mint_lp, // this checks that the lp_provider_ata_lp.mint = mint_lp.key()
        associated_token::token_program = token_program,
    )]
    pub lp_provider_ata_lp: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        require!(lp_amount > 0, AmmError::InvalidAmount);
        require!(!self.config.locked, AmmError::AMMLocked);

        let xy_amount = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount, 
            self.vault_y.amount, 
            self.mint_lp.supply, 
            lp_amount, 
            6,
        ).unwrap();

        require!(min_x <= xy_amount.x, AmmError::InsufficientTokenX);
        require!(min_y <= xy_amount.y, AmmError::InsufficientTokenY);

        self.withdraw_tokens(true, xy_amount.x)?;
        self.withdraw_tokens(false, xy_amount.y)?;
        self.burn_lp_tokens(lp_amount)?;

        Ok(())
    }

    fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (TransferChecked {
                from: self.vault_x.to_account_info(),
                mint: self.mint_x.to_account_info(),
                to: self.lp_provider_ata_x.to_account_info(),
                authority: self.config.to_account_info(),
            }, self.mint_x.decimals),

            false => (TransferChecked {
                from: self.vault_y.to_account_info(),
                mint: self.mint_y.to_account_info(),
                to: self.lp_provider_ata_y.to_account_info(),
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

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, mint_decimals)?;

        Ok(())
    }

    fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.lp_provider_ata_lp.to_account_info(),
            authority: self.lp_provider.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        burn(cpi_ctx, amount)?;

        Ok(())
    }
}