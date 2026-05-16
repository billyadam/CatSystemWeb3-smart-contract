use anchor_lang::prelude::*;

#[error_code]
pub enum CatError {
    #[msg("Name must be 32 characters or less")]
    NameTooLong,
    #[msg("Description must be 512 characters or less")]
    DescriptionTooLong,
    #[msg("Breed must be 32 characters or less")]
    BreedTooLong,
    #[msg("Coat color must be 32 characters or less")]
    CoatColorTooLong,
    #[msg("Eye color must be 32 characters or less")]
    EyeColorTooLong,
    #[msg("Image URL must be 256 characters or less")]
    ImageUrlTooLong,
    #[msg("Not the cat owner")]
    NotOwner,
}
