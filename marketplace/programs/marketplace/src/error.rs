use anchor_lang::error_code;


#[error_code]
pub enum MarketplaceError {
    #[msg("Name is Too Long")]
    NameTooLong,
    #[msg("Collectio is not Valid")]
    InvalidCollection,
    #[msg("Collectio is not Verified")]
    UnverifedCollection,
}