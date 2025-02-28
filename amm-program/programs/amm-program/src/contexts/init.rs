use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};


use crate::state::Config;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key().as_ref()],
        bump,
        // this ensures that mint_lp must be a mint account with it decimal set to 6 and it's authority set to config
        mint::decimals = 6,
        mint::authority = config
    )]
    // this is the token that would be used to reward the user for providing liquidity for the AMM
    pub mint_lp: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    // needed to store the token x
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        // this means that the account used here must be an ata 
        // and it mint must match mint_y and it authority must match config
        associated_token::mint = mint_y,
        associated_token::authority = config,
        associated_token::token_program = token_program,
    )]
    // needed to store the token y
    // if the intent is to check ATA alone, use Account<'info, AssociatedToken>
    pub vault_y: Account<'info, TokenAccount>, 
    #[account(
        init,
        payer = initializer,
        space = Config::INIT_SPACE,
        seeds = [
            b"config", 
            mint_x.key().to_bytes().as_ref(),
            mint_y.key().to_bytes().as_ref(),
            seed.to_le_bytes().as_ref()
        ],
        bump
    )]
    // this accounts holds configuration for the token pair
    pub config: Account<'info, Config>,

    // needed here because we are initializing a token program
    pub token_program: Program<'info, Token>,
    // needed here because we are initializing a system program (config)
    pub system_program: Program<'info, System>,
    // needed here because we are initializing an associated token program (vault_x and vault_y)
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner( Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
        });
        
        Ok(())
    }
     
}