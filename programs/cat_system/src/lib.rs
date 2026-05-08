use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::create_cat::*;
use state::Gender;

declare_id!("2MrZJsTvuSQ3HUtDozeGAm96eSrU1jCaVKjSSiu57Tzz");

#[program]
pub mod cat_system {
    use super::*;

    pub fn create_cat(
        ctx: Context<CreateCat>,
        name: String,
        gender: Gender,
        description: String,
    ) -> Result<()> {
        instructions::create_cat::handler(ctx, name, gender, description)
    }
}
