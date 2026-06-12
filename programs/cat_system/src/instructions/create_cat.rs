use anchor_lang::prelude::*;
use crate::errors::CatError;
use crate::events::CatCreated;
use crate::state::{
    BioProfile, Cat, Gender, MAX_BREED_LEN, MAX_COAT_COLOR_LEN, MAX_DESC_LEN, MAX_EYE_COLOR_LEN, MAX_NAME_LEN, MAX_PERSONALITY_TRAIT_LEN, UserCounter
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
    bio_profile: BioProfile,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, CatError::NameTooLong);
    require!(bio_profile.breed.len() <= MAX_BREED_LEN, CatError::BreedTooLong);
    require!(bio_profile.coat_color.len() <= MAX_COAT_COLOR_LEN, CatError::CoatColorTooLong);
    require!(bio_profile.eye_color.len() <= MAX_EYE_COLOR_LEN, CatError::EyeColorTooLong);
    require!(bio_profile.personality_trait.len() <= MAX_PERSONALITY_TRAIT_LEN, CatError::PersonalityTraitTooLong);
    require!(bio_profile.description.len() <= MAX_DESC_LEN, CatError::DescriptionTooLong);

    let cat_index = ctx.accounts.user_counter.cat_count;

    let cat = &mut ctx.accounts.cat;
    cat.owner = ctx.accounts.owner.key();
    cat.bump = ctx.bumps.cat;
    cat.name = name;
    cat.gender = gender;
    cat.bio_profile.breed = bio_profile.breed;
    cat.bio_profile.coat_color = bio_profile.coat_color;
    cat.bio_profile.coat_length = bio_profile.coat_length;
    cat.bio_profile.eye_color = bio_profile.eye_color;
    cat.bio_profile.ear_type = bio_profile.ear_type;
    cat.bio_profile.body_size = bio_profile.body_size;
    cat.bio_profile.personality_trait = bio_profile.personality_trait;
    cat.bio_profile.description = bio_profile.description;
    cat.image_count = 0;

    ctx.accounts.user_counter.cat_count += 1;

    emit!(CatCreated {
        cat: cat.key(),
        owner: cat.owner,
        cat_index,
        name: cat.name.clone(),
        gender: cat.gender.clone(),
        breed: cat.bio_profile.breed.clone(),
        coat_color: cat.bio_profile.coat_color.clone(),
        coat_length: cat.bio_profile.coat_length.clone(),
        eye_color: cat.bio_profile.eye_color.clone(),
        ear_type: cat.bio_profile.ear_type.clone(),
        body_size: cat.bio_profile.body_size.clone(),
        personality_trait: cat.bio_profile.personality_trait.clone(),
        description: cat.bio_profile.description.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
