use anchor_lang::error_code;


#[error_code]
pub enum AmmError {
    #[msg("Invalid Config")]
    InvalidConfig,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("AMM is Locked")]
    AMMLocked,
    #[msg("Insufficien amount of token X")]
    InsufficientTokenX,
    #[msg("Insufficien amount of token Y")]
    InsufficientTokenY,
}