use anchor_lang::prelude::*;
use anchor_spl;

#[derive(Accounts)]
pub struct Make<'info> {
    pub maker: Signer<'info>,
    pub mint_a: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    pub escrow: 
    pub associated_token: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>
    pub system_program: Program<'info, System>,
}