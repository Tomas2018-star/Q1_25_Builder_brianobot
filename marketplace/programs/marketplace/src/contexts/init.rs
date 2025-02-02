use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Marketplace;
use crate::error::MarketplaceError;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // manages/owns the marketplace
    #[account(
        init,
        payer = admin,
        space = Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
    )]
    // this would control all the ATA needed to interact for each sales (per sales vault)
    pub marketplace: Account<'info, Marketplace>, // configuratoin for the marketplace
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    // holds SOls gotten from the platform fee collection duration sales of nfts
    pub treasury: SystemAccount<'info>, //
    #[account(
        init, 
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        // when initializing a mint with anchor
        // you must specify the mint authority and decimal
        mint::authority = marketplace,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    // needed because we are initializing the marketplace account
    pub system_program: Program<'info, System>,
    // needed because we are creating a token account for reward mint
    pub token_program: Interface<'info, TokenInterface>
}


impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(name.len() > 0 && name.len() < 4 + 33, MarketplaceError::NameTooLong);
        
        self.marketplace.set_inner( Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_mint_bump: bumps.rewards_mint,
            name
        });
         
        Ok(())
    }
}