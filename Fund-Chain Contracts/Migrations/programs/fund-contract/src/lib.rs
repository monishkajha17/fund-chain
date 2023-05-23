use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cubic_contract {
    use super::*;

    pub fn create_user(
        ctx: Context<CreateUserContext>,
        _user_id: String,
        metadata: String,
    ) -> Result<()> {
        create_user::handler(ctx, _user_id, metadata)
    }
    pub fn project_init(
        ctx: Context<InitProjectContext>,
        _projectname: String,
        metadata: String,
    ) -> Result<()> {
        project_init::handler(ctx, metadata, _projectname)
    }

    pub fn init_chort(ctx: Context<InitChortContext>, _chortname: String) -> Result<()> {
        init_chort::handler(ctx, _chortname)
    }
    pub fn join_chort(
        ctx: Context<JoinChortContext>,
        _chortname: String,
        _projectname: String,
    ) -> Result<()> {
        join_chort::handler(ctx, _projectname, _chortname)
    }
}