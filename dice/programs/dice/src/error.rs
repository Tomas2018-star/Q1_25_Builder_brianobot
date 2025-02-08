use anchor_lang::error_code;


#[error_code]
pub enum DiceError {
    #[msg("Custom Error Goes here")]
    CustomError
}