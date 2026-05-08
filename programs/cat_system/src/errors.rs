use anchor_lang::prelude::*;

#[error_code]
pub enum CatError {
    #[msg("Name must be 32 characters or less")]
    NameTooLong,
    #[msg("Description must be 512 characters or less")]
    DescriptionTooLong,
    #[msg("Not the cat owner")]
    NotOwner,
}
