use anchor_lang::prelude::*;

use crate::state::{BioProfile, Gender};

#[event]
pub struct CatCreated {
    pub cat: Pubkey,
    pub owner: Pubkey,
    pub cat_index: u64,
    pub name: String,
    pub gender: Gender,
    pub date_of_birth: i64,
    pub bio_profile: BioProfile,
    pub timestamp: i64,
}

#[event]
pub struct CatImageAdded {
    pub cat: Pubkey,
    pub image_pda: Pubkey,
    pub index: u8,
    pub image_url: String,
    pub description: String,
}
