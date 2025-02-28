use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{
    associated_token::AssociatedToken, mint, token_2022::CloseAccount, token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked, close_account}
};


use crate::state::EscrowState;


#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    #[account(address = escrow.mint_a)] // this constaints prevent abuse of the mint as was provided during the escrow creation process
    // we would need the mint again here since
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    #[account(address = escrow.mint_b)] // this constaints prevent abuse of the mint as was provided during the escrow creation process
    pub mint_b: Box<InterfaceAccount<'info, Mint>>, 
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a, // ? how does this access works associated_token::mint = mint_a
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    // this is needed to store the tokens that would be received from 
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,  // this is expected to already exist for the taker since they must own those tokens
        associated_token::mint = mint_b, // ? how does this access works associated_token::mint = mint_a
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    // this is needed to store the tokens that would be give in the escrow from the take
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = taker,
        // the constaints below are called SPL Constraints, and they are either used as checked or as initialization values
        // they are used as checks when the account is not init, and vice versa
        associated_token::mint = mint_b, // ? how does this access works associated_token::mint = mint_a
        associated_token::authority = maker ,// we can not use escrow.maker, // difference between account and public keys as used in different part of anchor
        associated_token::token_program = token_program,
    )] 
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>, 
    #[account(
        mut,
        close = taker, 
        has_one = mint_b, // this checks that the escrow account has a field call mint_b and that field' value == mint_b value
        // escrow.mint_b == mint_b (from the top of the Account struct)
        has_one = mint_a, // same check as above
        has_one = maker,
        seeds = [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Box<Account<'info, EscrowState>>,
    #[account(
        mut,
        associated_token::mint = mint_a, // the constraint here applies to associated_token
        // vault.mint == mint_a
        associated_token::authority = escrow,
        // vault.authority == escrow
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>, // needed to close the escrow and vault account
    pub associated_token_program: Program<'info, AssociatedToken>,

}

impl<'info> Take<'info> {
    // transfer the token from the taker to the maker ata
    pub fn deposit(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_b.decimals)?;
        Ok(())
    }

    // transfer the token from the escrow vault ata to the taker
    pub fn release(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let seed_bytes = self.escrow.seed.to_le_bytes();

        // this is the original seeds for the escrow account 
        // [b"escrow", escrow.maker.as_ref(), escrow.seed.to_le_bytes().as_ref()],

        let seeds = &[
            b"escrow", 
            self.escrow.maker.as_ref(), 
            seed_bytes.as_ref(), 
            &[self.escrow.bump]
        ]; 

        let signers_seeds = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signers_seeds);

        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_a.decimals)?;

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        // close the escrow account and the vault account here
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let seed_bytes = self.escrow.seed.to_le_bytes();

        // this seeds needs to match that used in the account struct
        let seeds = &[b"escrow", self.escrow.maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]];

        let signers_seeds = [&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signers_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }

}