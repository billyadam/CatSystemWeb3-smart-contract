use anchor_lang::prelude::*;
use crate::errors::CatError;
use crate::events::CatImageAdded;
use crate::state::{Cat, CatImage, MAX_IMAGE_COUNT_LEN, MAX_IMAGE_URL_LEN, MAX_IMAGE_DESC_LEN};

#[derive(Accounts)]
pub struct AddCatImage<'info> {
    #[account(mut)]
    pub cat: Account<'info, Cat>,

    #[account(
        init,
        payer = payer,
        space = 8 + CatImage::INIT_SPACE,
        seeds = [
            b"cat-image",
            cat.key().as_ref(),
            &cat.image_count.to_le_bytes()
        ],
        bump
    )]
    pub cat_image: Account<'info, CatImage>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AddCatImage>,
    image_url: String,
    description: String,
) -> Result<()> {
    require!(ctx.accounts.cat.owner == ctx.accounts.payer.key(), CatError::NotOwner);
    require!(image_url.len() <= MAX_IMAGE_URL_LEN, CatError::ImageUrlTooLong);
    require!(description.len() <= MAX_IMAGE_DESC_LEN, CatError::ImageDescriptionTooLong);
    require!(ctx.accounts.cat.image_count <= MAX_IMAGE_COUNT_LEN, CatError::ImageCountTooMany);

    let cat = &mut ctx.accounts.cat;
    let cat_image = &mut ctx.accounts.cat_image;

    cat_image.cat = cat.key();
    cat_image.index = cat.image_count;
    cat_image.bump = ctx.bumps.cat_image;
    cat_image.image_url = image_url;
    cat_image.description = description;

    cat.image_count += 1;

    emit!(CatImageAdded {
        cat: cat.key(),
        image_pda: cat_image.key(),
        index: cat_image.index,
        image_url: cat_image.image_url.clone(),
        description: cat_image.description.clone(),
    });

    Ok(())
}