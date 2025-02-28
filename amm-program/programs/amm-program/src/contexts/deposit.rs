use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer_checked, MintTo, mint_to, TransferChecked};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use constant_product_curve::ConstantProduct;

use crate::state::Config;
use crate::error::AmmError;


#[derive(Accounts)]
pub struct Deposit<'info> {
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

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, lp_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        // lp_amount: amount of LP Tokens that the user wants to claim
        // max_x: the max amount of x they are willing to deposit
        // max_y: the max amount of y they are willing to deposit
        require!(lp_amount > 0, AmmError::InvalidAmount);
        require!(!self.config.locked, AmmError::AMMLocked);

        let (x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            true => (max_x, max_y), // we set the pair as the initial ratio
            false => {
                // this basically calculates how much of token x and token y
                // the user needs to deposit to mint lp_amount of the lp tokens
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,  // current reserve of token x
                    self.vault_y.amount,  // current reserve of token y
                    self.mint_lp.supply,  // total supply of lp tokens in circulation
                    lp_amount,  // the amount of lp tokens the user wants to mint
                    6 // decimal precision of the lp tokens
                ).unwrap();
                (amounts.x, amounts.y) 
            },
        };

        require!(max_x >= x, AmmError::InsufficientTokenX);
        require!(max_y >= y, AmmError::InsufficientTokenY);

        self.deposit_token(true, x)?;
        self.deposit_token(false, y)?;
        self.mint_lp_tokens(lp_amount)?;

        Ok(())
    }

    // taking a token from the user to the amm pool
    fn deposit_token(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let (cpi_accounts, mint_decimals) = match is_x {
            true => (TransferChecked {
                    from: self.lp_provider_ata_x.to_account_info(),
                    mint: self.mint_x.to_account_info(),
                    to: self.vault_x.to_account_info(),
                    authority: self.lp_provider.to_account_info(),
                }, self.mint_x.decimals),
            false => (TransferChecked {
                    from: self.lp_provider_ata_y.to_account_info(),
                    mint: self.mint_y.to_account_info(),
                    to: self.vault_y.to_account_info(),
                    authority: self.lp_provider.to_account_info(),
                }, self.mint_y.decimals),
        };
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        transfer_checked(cpi_ctx, amount, mint_decimals)?;

        Ok(())
    }

    fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
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
        // it is important to note that new_with_signers can only be used on Accountstruct that have the 
        // the authority as a pda that belongs to our program   

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;
        
        Ok(())
    }
}