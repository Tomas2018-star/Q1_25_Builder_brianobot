use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, mint_to, transfer_checked, CloseAccount, MintTo, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};


use crate::state::{Listing, Marketplace};

// reward the makret and/or the taker for their participation in the marketplace
// use the reward token mint as a reward

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close = maker, 
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_mint_bump,
        mint::authority = marketplace,
        mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> Purchase<'info> {
    // send solr
    // transfering price from taker to the maker
    // take part of the price as fee
    pub fn pay(&mut self) -> Result<()> {
        // pay the maker 
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // 
        let fee = self.marketplace.fee as u64;
        let amount = self.listing.price.checked_sub(fee).unwrap();

        transfer(cpi_ctx, amount)?;

        // pay the fee to the marketplace
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, fee)?;

        Ok(())
    }

    // transfer the nft
    pub fn transfer(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = [
            &self.marketplace.key().to_bytes()[..], 
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, 1, 0);

        Ok(())
    }

    // close the accounts
    pub fn close_vault_account(&mut self) -> Result<()> {
        let seeds = [
            &self.marketplace.key().to_bytes()[..], 
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts, 
            signer_seeds
        );

        close_account(cpi_ctx)?;
        
        Ok(())
    }

    // reward buter
    pub fn reward_buyer(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(), 
            to: self.taker.to_account_info(), // the ATA that belongs to the taker
            authority: self.marketplace.to_account_info(),
        };

        let seeds = &[
            "marketplace".as_bytes(), 
            self.marketplace.name.as_str().as_bytes(),
            &[self.marketplace.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

}