use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct EscrowState {
    pub seed: u64, // seeds used to derive the escrow PDA
    pub maker: Pubkey, // the maker of the escrow
    pub mint_a: Pubkey, // the mint of the token being escrowed
    pub mint_b: Pubkey, // the mint of the token being escrowed
    pub receive_amount: u64,
    pub bump: u8,
}