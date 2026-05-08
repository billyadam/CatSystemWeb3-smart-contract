use anchor_lang::prelude::*;

pub const MAX_NAME_LEN: usize = 32;
pub const MAX_DESC_LEN: usize = 512;

#[account]
#[derive(InitSpace)]
pub struct Cat {
    pub owner: Pubkey,
    pub bump: u8,
    #[max_len(32)]
    pub name: String,
    pub gender: Gender,
    #[max_len(512)]
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, InitSpace)]
pub enum Gender {
    Male,
    Female,
}
