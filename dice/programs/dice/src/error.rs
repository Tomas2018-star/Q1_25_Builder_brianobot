use anchor_lang::error_code;


#[error_code]
pub enum DiceError {
    #[msg("Custom Error Goes here")]
    CustomError,
    #[msg("Invalid Program ID")]
    InvalidProgramID,
    #[msg("Invalid Account Counnts")]
    InvalidAccountCount,
    #[msg("Signature is not verifiable")]
    UnverifiableSignature,
    #[msg("Refund Cool Down Time Not Elasped")]
    RefundCooldownNotElapsed,
}