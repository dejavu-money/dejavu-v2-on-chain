mod oracles;
mod organizations;

use anchor_lang::prelude::*;

use crate::organizations::accounts::*;
use crate::organizations::handlers::*;
use crate::organizations::instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dejavu_v2 {
    use super::*;

    /* Organization Instructions */
    pub fn create_organization(
        ctx: Context<CreateOrganizationAccounts>,
        instruction: CreateOrganizationInstruction,
    ) -> Result<()> {
        create_organization_handler(ctx, instruction)
    }

    pub fn update_organization(
        ctx: Context<UpdateOrganizationAccounts>,
        instruction: UpdateOrganizationInstruction,
    ) -> Result<()> {
        update_organization_handler(ctx, instruction)
    }

    pub fn withdraw_from_organization(
        ctx: Context<WithdrawFromOrganizationAccounts>,
        instruction: WithdrawFromOrganizationInstruction,
    ) -> Result<()> {
        withdraw_from_organization_handler(ctx, instruction)
    }
    /* Organization Instructions */
}

#[derive(Accounts)]
pub struct Initialize {}
