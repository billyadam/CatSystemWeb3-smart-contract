use anchor_lang::prelude::*;

pub const MAX_NAME_LEN: usize = 32;
pub const MAX_DESC_LEN: usize = 512;
pub const MAX_BREED_LEN: usize = 32;
pub const MAX_COAT_COLOR_LEN: usize = 32;
pub const MAX_EYE_COLOR_LEN: usize = 32;
pub const MAX_PERSONALITY_TRAIT_LEN: usize = 32;
pub const MAX_IMAGE_URL_LEN: usize = 256;
pub const MAX_IMAGE_COUNT_LEN: u8 = 255;

#[account]
#[derive(InitSpace)]
pub struct Cat {
    pub owner: Pubkey,
    pub bump: u8,
    #[max_len(32)]
    pub name: String,
    pub gender: Gender,
    pub bio_profile: BioProfile,
    pub image_count: u8,
}

#[account]
#[derive(InitSpace)]
pub struct CatImage {
    pub cat: Pubkey,
    pub bump: u8,
    pub index: u8,
    #[max_len(256)]
    pub image_url: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum Gender {
    Male,
    Female,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub struct BioProfile {
    #[max_len(32)]
    pub breed: String,
    #[max_len(32)]
    pub coat_color: String,
    pub coat_length: CoatLength,
    #[max_len(32)]
    pub eye_color: String,
    pub ear_type: EarType,
    pub body_size: BodySize,
    #[max_len(32)]
    pub personality_trait: String,
    #[max_len(512)]
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum CoatLength {
    Short,
    Medium,
    Long,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum EarType {
    Pointed,
    Rounded,
    Folded,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum BodySize {
    Small,
    Medium,
    Large,
}
