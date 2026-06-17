use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::create_cat::*;
use instructions::add_cat_image::*;
use state::{Gender, BioProfile};

declare_id!("6hAoHrSuAg3eA55igUeXYS8ckVTih6RheK2E2N9Zvca");

#[program]
pub mod cat_system {
    use super::*;

    pub fn create_cat(
        ctx: Context<CreateCat>,
        name: String,
        gender: Gender,
        bio_profile: BioProfile,
    ) -> Result<()> {
        instructions::create_cat::handler(
            ctx,
            name,
            gender,
            bio_profile,
        )
    }

    pub fn add_cat_image(
        ctx: Context<AddCatImage>,
        image_url: String,
    ) -> Result<()> {
        instructions::add_cat_image::handler(
            ctx,
            image_url,
        )
    }
}
