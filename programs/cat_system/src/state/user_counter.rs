use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserCounter {
    pub cat_count: u64,
}
