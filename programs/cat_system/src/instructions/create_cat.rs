use anchor_lang::prelude::*;
use crate::errors::CatError;
use crate::events::CatCreated;
use crate::state::{
    BioProfile, Cat, Gender,
    MAX_BREED_LEN, MAX_COAT_COLOR_LEN, MAX_EYE_COLOR_LEN,
    MAX_NAME_LEN, MAX_PERSONALITY_TRAIT_LEN, MAX_DISTINCTIVE_MARKS_LEN,
    MAX_SPECIAL_SKILL_LEN, MAX_LIKES_LEN, MAX_DISLIKES_LEN,
    MAX_ADDITIONAL_NOTES_LEN, UserCounter,
};

#[derive(Accounts)]
pub struct CreateCat<'info> {
    #[account(
        init_if_needed,
        payer = owner,
        space = 8 + UserCounter::INIT_SPACE,
        seeds = [b"user_counter", owner.key().as_ref()],
        bump,
    )]
    pub user_counter: Account<'info, UserCounter>,

    #[account(
        init,
        payer = owner,
        space = 8 + Cat::INIT_SPACE,
        seeds = [b"cat", owner.key().as_ref(), &user_counter.cat_count.to_le_bytes()],
        bump,
    )]
    pub cat: Account<'info, Cat>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateCat>,
    name: String,
    gender: Gender,
    date_of_birth: i64,
    bio_profile: BioProfile,
) -> Result<()> {
    // === Validate string lengths ===
    require!(name.len() <= MAX_NAME_LEN, CatError::NameTooLong);
    require!(bio_profile.breed.len() <= MAX_BREED_LEN, CatError::BreedTooLong);
    require!(bio_profile.coat_color.len() <= MAX_COAT_COLOR_LEN, CatError::CoatColorTooLong);
    require!(bio_profile.eye_color.len() <= MAX_EYE_COLOR_LEN, CatError::EyeColorTooLong);
    require!(bio_profile.distinctive_marks.len() <= MAX_DISTINCTIVE_MARKS_LEN, CatError::DistinctiveMarksTooLong);
    require!(bio_profile.special_skill.len() <= MAX_SPECIAL_SKILL_LEN, CatError::SpecialSkillTooLong);
    require!(bio_profile.likes.len() <= MAX_LIKES_LEN, CatError::LikesTooLong);
    require!(bio_profile.dislikes.len() <= MAX_DISLIKES_LEN, CatError::DislikesTooLong);
    require!(bio_profile.personality_trait.len() <= MAX_PERSONALITY_TRAIT_LEN, CatError::PersonalityTraitTooLong);
    require!(bio_profile.additional_notes.len() <= MAX_ADDITIONAL_NOTES_LEN, CatError::AdditionalNotesTooLong);

    let cat_index = ctx.accounts.user_counter.cat_count;

    let cat = &mut ctx.accounts.cat;
    cat.owner = ctx.accounts.owner.key();
    cat.bump = ctx.bumps.cat;
    cat.name = name;
    cat.gender = gender;
    cat.date_of_birth = date_of_birth;

    // Physical
    cat.bio_profile.breed = bio_profile.breed;
    cat.bio_profile.coat_color = bio_profile.coat_color;
    cat.bio_profile.pattern_type = bio_profile.pattern_type;
    cat.bio_profile.coat_length = bio_profile.coat_length;
    cat.bio_profile.eye_color = bio_profile.eye_color;
    cat.bio_profile.ear_type = bio_profile.ear_type;
    cat.bio_profile.body_size = bio_profile.body_size;
    cat.bio_profile.body_type = bio_profile.body_type;
    cat.bio_profile.distinctive_marks = bio_profile.distinctive_marks;
    cat.bio_profile.blood_type = bio_profile.blood_type;

    // Personality
    cat.bio_profile.temperament = bio_profile.temperament;
    cat.bio_profile.energy_level = bio_profile.energy_level;
    cat.bio_profile.social_behavior = bio_profile.social_behavior;
    cat.bio_profile.special_skill = bio_profile.special_skill;
    cat.bio_profile.likes = bio_profile.likes;
    cat.bio_profile.dislikes = bio_profile.dislikes;
    cat.bio_profile.personality_trait = bio_profile.personality_trait;
    cat.bio_profile.additional_notes = bio_profile.additional_notes;

    cat.image_count = 0;

    ctx.accounts.user_counter.cat_count += 1;

    emit!(CatCreated {
        cat: cat.key(),
        owner: cat.owner,
        cat_index,
        name: cat.name.clone(),
        gender: cat.gender.clone(),
        date_of_birth: cat.date_of_birth,
        bio_profile: cat.bio_profile.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
