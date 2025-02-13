use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct UserAccountState {
    pub points: u32, // the number of reward tokens
    pub amount_staked: u8, // number of nfts staked
    pub bump: u8,
}

impl Default for UserAccountState {
    fn default() -> Self {
        Self {
            points: 0,
            amount_staked: 0,
            bump: 255,
        }
    }
}