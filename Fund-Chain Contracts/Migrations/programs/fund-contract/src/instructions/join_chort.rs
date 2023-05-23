use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self, system_program, sysvar::rent::Rent};

#[derive(Accounts)]
#[instruction(_chortname:String,_projectname:String)]
pub struct JoinChortContext<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"init-chort",_chortname.as_ref(),authority.key().as_ref()],
        bump = init_chort_account.bump
    )]
    pub init_chort_account : Account<'info,InitChort>,

    #[account(mut,
        seeds = [b"project".as_ref(),_projectname.as_ref(),authority.key().as_ref()],
        bump = project_account.bump
    )]
    pub project_account : Account<'info, ProjectInit>,


    #[account(init,
        payer = authority,
        space = ProjectChortInit::LEN,
        seeds = [b"project_chort".as_ref(),_chortname.as_ref(),authority.key().as_ref()],
        bump 
    )]
    pub project_chort_account: Account<'info, ProjectChortInit>,


      // Misc Accounts
      #[account(address = system_program::ID)]
      pub system_program: Program<'info, System>,
      #[account(address = solana_program::sysvar::rent::ID)]
      pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<JoinChortContext>,
    _projectname: String,
    _chortname: String,
) -> Result<()> {
    let project_chort_account = &mut ctx.accounts.project_chort_account;
    let project_account = &mut ctx.accounts.project_account;
    let chort_account = &mut ctx.accounts.init_chort_account;

    project_chort_account.project = project_account.key();
    project_chort_account.chort = chort_account.key();
    project_chort_account.bump = *ctx.bumps.get("project_chort_account").unwrap();

    Ok(())
}