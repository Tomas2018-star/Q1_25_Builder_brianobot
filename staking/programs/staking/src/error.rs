use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("Maximum Stake Reached")]
    MaxStakeReached
}