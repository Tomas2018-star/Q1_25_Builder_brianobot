use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccountState {
    pub owner: Pubkey,
    pub nft_mint: Pubkey, // the mint of the nft that was staked by the user
    pub staked_at: i64,
    pub bump: u8,
}