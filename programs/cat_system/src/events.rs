use anchor_lang::prelude::*;

use crate::state::{BodySize, CoatLength, EarType, Gender};

#[event]
pub struct CatCreated {
    pub cat: Pubkey,
    pub owner: Pubkey,
    pub cat_index: u64,
    pub name: String,
    pub gender: Gender,
    pub breed: String,
    pub coat_color: String,
    pub coat_length: CoatLength,
    pub eye_color: String,
    pub ear_type: EarType,
    pub body_size: BodySize,
    pub description: String,
    pub image_url_1: String,
    pub image_url_2: String,
    pub timestamp: i64,
}
