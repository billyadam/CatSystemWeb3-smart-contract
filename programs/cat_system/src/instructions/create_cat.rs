use anchor_lang::prelude::*;
use crate::errors::CatError;
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
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, CatError::NameTooLong);
    require!(breed.len() <= MAX_BREED_LEN, CatError::BreedTooLong);
    require!(coat_color.len() <= MAX_COAT_COLOR_LEN, CatError::CoatColorTooLong);
    require!(eye_color.len() <= MAX_EYE_COLOR_LEN, CatError::EyeColorTooLong);
    require!(description.len() <= MAX_DESC_LEN, CatError::DescriptionTooLong);

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

    ctx.accounts.user_counter.cat_count += 1;

    Ok(())
}
