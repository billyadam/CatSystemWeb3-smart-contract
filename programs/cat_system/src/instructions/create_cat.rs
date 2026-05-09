use anchor_lang::prelude::*;
use crate::errors::CatError;
use crate::state::{Cat, UserCounter, Gender, MAX_NAME_LEN, MAX_DESC_LEN};

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
    description: String,
) -> Result<()> {
    require!(name.len() <= MAX_NAME_LEN, CatError::NameTooLong);
    require!(description.len() <= MAX_DESC_LEN, CatError::DescriptionTooLong);

    let cat = &mut ctx.accounts.cat;
    cat.owner = ctx.accounts.owner.key();
    cat.bump = ctx.bumps.cat;
    cat.name = name;
    cat.gender = gender;
    cat.description = description;

    ctx.accounts.user_counter.cat_count += 1;

    Ok(())
}
