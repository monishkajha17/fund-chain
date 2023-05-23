use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

use crate::state::ProjectInit;

#[derive(Accounts)]
#[instruction(_projectname:String)]
pub struct InitProjectContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init,
        payer = authority,
        space = ProjectInit::LEN,
        seeds = [b"project".as_ref(),_projectname.as_ref(),authority.key().as_ref()],
        bump
    )]
    pub project_account: Account<'info, ProjectInit>,

    // Misc Accounts
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<InitProjectContext>,
    metadata: String,
    _projectname: String,
) -> Result<()> {
    let project_account = &mut ctx.accounts.project_account;
    project_account.metadata = metadata;

    project_account.bump = *ctx.bumps.get("project_account").unwrap();

    Ok(())
}