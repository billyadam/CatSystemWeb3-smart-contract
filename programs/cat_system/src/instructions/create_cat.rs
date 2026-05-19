use anchor_lang::prelude::*;
use crate::errors::CatError;
use crate::events::CatCreated;
use crate::state::{
    Cat,
    UserCounter,
    Gender,
    CoatLength,
    EarType,
    BodySize,
    MAX_NAME_LEN,
    MAX_DESC_LEN,
    MAX_BREED_LEN,
    MAX_COAT_COLOR_LEN,
    MAX_EYE_COLOR_LEN,
    MAX_IMAGE_URL_LEN,
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
    breed: String,
    coat_color: String,
    coat_length: CoatLength,
    eye_color: String,
    ear_type: EarType,
    body_size: BodySize,
    description: String,
    image_url_1: String,
    image_url_2: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, CatError::NameTooLong);
    require!(breed.len() <= MAX_BREED_LEN, CatError::BreedTooLong);
    require!(coat_color.len() <= MAX_COAT_COLOR_LEN, CatError::CoatColorTooLong);
    require!(eye_color.len() <= MAX_EYE_COLOR_LEN, CatError::EyeColorTooLong);
    require!(description.len() <= MAX_DESC_LEN, CatError::DescriptionTooLong);
    require!(image_url_1.len() <= MAX_IMAGE_URL_LEN, CatError::ImageUrlTooLong);
    require!(image_url_2.len() <= MAX_IMAGE_URL_LEN, CatError::ImageUrlTooLong);

    let cat_index = ctx.accounts.user_counter.cat_count;

    let cat = &mut ctx.accounts.cat;
    cat.owner = ctx.accounts.owner.key();
    cat.bump = ctx.bumps.cat;
    cat.name = name;
    cat.gender = gender;
    cat.breed = breed;
    cat.coat_color = coat_color;
    cat.coat_length = coat_length;
    cat.eye_color = eye_color;
    cat.ear_type = ear_type;
    cat.body_size = body_size;
    cat.description = description;
    cat.image_url_1 = image_url_1;
    cat.image_url_2 = image_url_2;

    ctx.accounts.user_counter.cat_count += 1;

    emit!(CatCreated {
        cat: cat.key(),
        owner: cat.owner,
        cat_index,
        name: cat.name.clone(),
        gender: cat.gender.clone(),
        breed: cat.breed.clone(),
        coat_color: cat.coat_color.clone(),
        coat_length: cat.coat_length.clone(),
        eye_color: cat.eye_color.clone(),
        ear_type: cat.ear_type.clone(),
        body_size: cat.body_size.clone(),
        description: cat.description.clone(),
        image_url_1: cat.image_url_1.clone(),
        image_url_2: cat.image_url_2.clone(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
