use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u128, 
    pub slot: u64, 
    pub amoutn: u64,
    pub roll: 
    pub 
}