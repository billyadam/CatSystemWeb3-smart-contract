use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::create_cat::*;
use state::{Gender, CoatLength, EarType, BodySize};

declare_id!("A5NzuxFQmaH1mKM5BUfnj4CRBWMketRrqUsXXY9PCwVG");

#[program]
pub mod cat_system {
    use super::*;

    pub fn create_cat(
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
        instructions::create_cat::handler(
            ctx,
            name,
            gender,
            breed,
            coat_color,
            coat_length,
            eye_color,
            ear_type,
            body_size,
            description,
            image_url_1,
            image_url_2,
        )
    }
}
