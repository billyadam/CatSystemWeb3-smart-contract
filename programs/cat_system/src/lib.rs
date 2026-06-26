use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::create_cat::*;
use instructions::add_cat_image::*;
use state::{Gender, BioProfile};

declare_id!("A6aoUvfhcf2ZQ5qtBiHw2d8QLhUTugZytgX421NwKcjj");

#[program]
pub mod cat_system {
    use super::*;

    pub fn create_cat(
        ctx: Context<CreateCat>,
        name: String,
        gender: Gender,
        date_of_birth: i64,
        bio_profile: BioProfile,
    ) -> Result<()> {
        instructions::create_cat::handler(
            ctx,
            name,
            gender,
            date_of_birth,
            bio_profile,
        )
    }

    pub fn add_cat_image(
        ctx: Context<AddCatImage>,
        image_url: String,
        description: String,
    ) -> Result<()> {
        instructions::add_cat_image::handler(
            ctx,
            image_url,
            description,
        )
    }
}
