use anchor_lang::prelude::*;
use std::mem::size_of;

pub const MAX_LEN_URI: usize = 128;
#[account]
#[derive(Default)]
pub struct User {
    pub authority: Pubkey,
    pub metadata: String,
    pub bump: u8,
}
impl User {
    pub const LEN: usize = 8 + 32 + 1 + MAX_LEN_URI;
}
#[account]
#[derive(Default)]
pub struct ProjectInit {
    pub authority: Pubkey,
    pub metadata: String,
    pub bump: u8,
}
impl ProjectInit {
    pub const LEN: usize = 8 + 32 + 1 + MAX_LEN_URI;
}

#[account]
#[derive(Default)]
pub struct InitChort {
    pub authority: Pubkey,
    pub chort_name: String,
    pub active: bool,
    pub bump: u8,
}
impl InitChort {
    pub const LEN: usize = 8 + 32 + 1 + 2 + 100;
}
#[account]
#[derive(Default)]
pub struct ProjectChortInit {
    pub project: Pubkey,
    pub chort: Pubkey,
    pub bump: u8,
}
impl ProjectChortInit {
    pub const LEN: usize = 8 + 32 + 1 + 32;
}