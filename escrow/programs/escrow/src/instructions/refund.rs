
use anchor_lang::prelude::*;  
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{TransferChecked, transfer_checked, close_account, CloseAccount};

use crate::state::EscrowState;


#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    // an InterfaceAccount is a wrapper around an AccountInfo that provides
    // a more ergonomic interface for interacting with the account
    // it is used to interact with the associated token account
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a, // ? how does this access works associated_token::mint = mint_a
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    // the assumption here is that the maker has already created the associated token account
    // since they want to exchange token a for token b they must already have an ATA to store token a
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,  
    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],  
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    // this ATA would hold the token received from maker of the escrow
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        
        // use the TransferChecked struct to transfer tokens from the maker ATA to the escrow vault
        // not the Usual Transfer Struct we use for normal SOL transfer
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(), // since the escrow account owns the from account (vault account)
        };

        let escrow_seed = self.escrow.seed.to_le_bytes();

        let seeds = [
            b"escrow", 
            self.maker.key.as_ref(),
            escrow_seed.as_ref(),
            &[self.escrow.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // transfer the tokens
        // use amount when you want to get the amount of token a token account holds
        // use get_lamports() when you want to get the amount of SOL/lamports stored on an account
        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        // close the escrow account and the vault account here
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let seed_bytes = self.escrow.seed.to_le_bytes();

        // this seeds needs to match that used in the account struct
        let seeds = &[b"escrow", self.escrow.maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]];

        let signers_seeds = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_accounts, &signers_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}