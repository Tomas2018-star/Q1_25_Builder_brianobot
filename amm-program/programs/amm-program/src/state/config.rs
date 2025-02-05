use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed: u64, // seeds to be able to create different pools
    pub authority: Option<Pubkey>, // if we want an authority to lock the config account
    pub mint_x: Pubkey, // Token X
    pub mint_y: Pubkey, // Token Y
    pub fee: u16, // Swap fees in basis point
    pub locked: bool,
    pub config_bump: u8, // seed bump for the config account
    pub lp_bump: u8, // seed bump for the lp token account
}

impl Space for Config {
    // why add a 1 + 32 to the Option<Pubkey> type
    // Reason: Pubkey is 32 bytes, and Option is 1 byte

    // u8 -> 1 byte
    // Option -> 1 byte
    
    // u16 -> 2 bytes
    // Option<u16> -> 3 bytes but since this is odd, it must be padded to an even number
    const INIT_SPACE: usize = 8 + 8 + (1 + 32) + 32 + 32 + 2 + 1 + 1 + 1;
}