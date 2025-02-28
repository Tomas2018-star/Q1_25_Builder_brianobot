use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};

use crate::state::EscrowState;


#[derive(Accounts)]
#[instruction(seeds: u64)] // this means i would pass a type u8 to the seeds field inside the instruction
// when using multiple instructions params, keep them in the same order in the instruction as provided here
pub struct Make<'info> {
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
        init, 
        payer = maker,
        space = 8 + EscrowState::INIT_SPACE,
        // every seed in the seeds array must be a byte slice
        // &[u8], so integers are converted to bytes by using to_le_bytes()
        // to_le_bytes() returns a byte version of the seed and as_ref() returns a reference to that byte which
        // together is a byte slice
        seeds = [b"escrow", maker.key().as_ref(), seeds.to_le_bytes().as_ref()], // 
        bump,
    )]
    pub escrow: Account<'info, EscrowState>,
    #[account( // notice how TokenAccount init does not require space constraint
        init,
        payer = maker,
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

impl<'info> Make<'info> {
    pub fn make(&mut self, seed: u64, receive_amount: u64, bumps: &MakeBumps) -> Result<()> {
        self.escrow.set_inner(EscrowState {
            seed,
            maker: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            receive_amount,
            bump: bumps.escrow,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        
        // use the TransferChecked struct to transfer tokens from the maker ATA to the escrow vault
        // not the Usual Transfer Struct we use for normal SOL transfer
        let cpi_accounts = TransferChecked {
            from: self.maker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // transfer the tokens
        transfer_checked(cpi_ctx, amount, self.mint_a.decimals)?;
        Ok(())
    }
}
