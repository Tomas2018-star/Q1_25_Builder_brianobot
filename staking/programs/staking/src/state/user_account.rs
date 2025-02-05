use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccountState {
    pub points: u32, // the number of reward tokens
    pub amount_staked: u8, // number of nfts staked
    pub bump: u8,
}