use anchor_lang::prelude::*;

/**
 * A New stake account is created each time the user stake a new NFT
 */

#[account]
#[derive(InitSpace)]
pub struct StakeAccountState {
    pub owner: Pubkey, // the user_account that owns the stake account
    pub nft_mint: Pubkey, // the mint of the nft that was staked by the user
    pub staked_at: i64,
    pub bump: u8, // storing the bump here to save Compute Unit in the future
}