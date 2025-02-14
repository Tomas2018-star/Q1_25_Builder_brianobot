use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Maximum Staked NFT Reached")]
    MaxStakeReached
}