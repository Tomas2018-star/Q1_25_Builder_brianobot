use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u128, 
    pub slot: u64, 
    pub amount: u64,
    pub roll: u8, 
    pub bump: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        // this is a way to convert all the data (fields) in the struct to bytes
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.amount.to_le_bytes());
        // TODO: why didn't we call the to_le_bytes on the two fields below 
        // SOLUTION: u8 are bytes already, 
        s.extend_from_slice(&[self.roll, self.bump]);
        s
    }
}