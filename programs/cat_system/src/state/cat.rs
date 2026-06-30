use anchor_lang::prelude::*;

pub const MAX_NAME_LEN: usize = 32;
pub const MAX_BREED_LEN: usize = 32;
pub const MAX_COAT_COLOR_LEN: usize = 32;
pub const MAX_EYE_COLOR_LEN: usize = 32;
pub const MAX_PERSONALITY_TRAIT_LEN: usize = 32;
pub const MAX_IMAGE_URL_LEN: usize = 256;
pub const MAX_IMAGE_DESC_LEN: usize = 64;
pub const MAX_IMAGE_COUNT_LEN: u8 = 255;
pub const MAX_DISTINCTIVE_MARKS_LEN: usize = 128;
pub const MAX_SPECIAL_SKILL_LEN: usize = 64;
pub const MAX_LIKES_LEN: usize = 128;
pub const MAX_DISLIKES_LEN: usize = 128;
pub const MAX_ADDITIONAL_NOTES_LEN: usize = 256;

#[account]
#[derive(InitSpace)]
pub struct Cat {
    pub owner: Pubkey,
    pub bump: u8,
    #[max_len(32)]
    pub name: String,
    pub gender: Gender,
    pub date_of_birth: i64,
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
    #[max_len(64)]
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum Gender {
    Male,
    Female,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub struct BioProfile {
    // Physical
    #[max_len(32)]
    pub breed: String,
    #[max_len(32)]
    pub coat_color: String,
    pub pattern_type: PatternType,
    pub coat_length: CoatLength,
    #[max_len(32)]
    pub eye_color: String,
    pub ear_type: EarType,
    pub body_size: BodySize,
    pub body_type: BodyType,
    #[max_len(128)]
    pub distinctive_marks: String,
    pub blood_type: BloodType,

    // Personality
    pub temperament: Temperament,
    pub energy_level: EnergyLevel,
    pub social_behavior: SocialBehavior,
    #[max_len(64)]
    pub special_skill: String,
    #[max_len(128)]
    pub likes: String,
    #[max_len(128)]
    pub dislikes: String,
    #[max_len(32)]
    pub personality_trait: String,
    #[max_len(256)]
    pub additional_notes: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub struct PatternType {
    pub category: PatternCategory,
    pub visual_pattern: VisualPattern,
    pub color: PatternColor,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum PatternCategory {
    Tabby,
    Solid,
    Bicolor,
    Special,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum VisualPattern {
    Mackerel,
    Classic, 
    Solid,    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum PatternColor {
    Hitam,
    Abu,
    Orange,
    Putih,
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
    VeryLarge
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum BodyType {
    Oriental,
    Muscular,
    Lean,
    Ideal,
    Stocky,
    Overweight
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum BloodType {
    A,
    B,
    AB
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum Temperament {
    Friendly,
    Aggressive,
    Calm
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum EnergyLevel {
    Low,
    Calm,
    Balanced,
    Active,
    Hyper
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum SocialBehavior {
    Friendly,
    Playful,
    Independent,
    GoodWithCat,
    Vocal,
}
