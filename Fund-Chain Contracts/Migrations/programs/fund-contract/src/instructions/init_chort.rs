use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};
#[derive(Accounts)]
#[instruction(_chortname:String)]
pub struct InitChortContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = InitChort::LEN,
        seeds = [b"init-chort".as_ref(),_chortname.as_ref(),authority.key().as_ref()],
        bump
    )]
    pub chort_init_account: Account<'info, InitChort>,
    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitChortContext>, _chortname: String) -> Result<()> {
    let chort_init = &mut ctx.accounts.chort_init_account;
    chort_init.chort_name = _chortname;
    chort_init.active = true;
    chort_init.bump = *ctx.bumps.get("chort_init_account").unwrap();

    Ok(())
}