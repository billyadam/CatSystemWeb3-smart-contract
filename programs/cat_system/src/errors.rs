use anchor_lang::prelude::*;

#[error_code]
pub enum CatError {
    #[msg("Name must be 32 characters or less")]
    NameTooLong,
    #[msg("Personality trait must be 32 characters or less")]
    PersonalityTraitTooLong,
    #[msg("Breed must be 32 characters or less")]
    BreedTooLong,
    #[msg("Coat color must be 32 characters or less")]
    CoatColorTooLong,
    #[msg("Eye color must be 32 characters or less")]
    EyeColorTooLong,
    #[msg("Distinctive marks must be 128 characters or less")]
    DistinctiveMarksTooLong,
    #[msg("Special skill must be 64 characters or less")]
    SpecialSkillTooLong,
    #[msg("Likes must be 128 characters or less")]
    LikesTooLong,
    #[msg("Dislikes must be 128 characters or less")]
    DislikesTooLong,
    #[msg("Additional notes must be 256 characters or less")]
    AdditionalNotesTooLong,
    #[msg("Image URL must be 256 characters or less")]
    ImageUrlTooLong,
    #[msg("Image description must be 64 characters or less")]
    ImageDescriptionTooLong,
    #[msg("Not the cat owner")]
    NotOwner,
    #[msg("Too many cat images")]
    ImageCountTooMany,
}
