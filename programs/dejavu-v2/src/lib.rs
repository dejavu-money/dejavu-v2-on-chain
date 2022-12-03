mod oracles;
mod organizations;
mod shared;

use anchor_lang::prelude::*;

use crate::organizations::accounts::*;
use crate::organizations::handlers::*;
use crate::organizations::instructions::*;

use crate::oracles::accounts::*;
use crate::oracles::handlers::*;
use crate::oracles::instructions::*;

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

    /* Oracles Instructions */
    pub fn create_oracle(
        ctx: Context<CreateOracleAccounts>,
        instruction: CreateOracleInstruction,
    ) -> Result<()> {
        create_oracle_handler(ctx, instruction)
    }

    pub fn update_oracle(
        ctx: Context<UpdateOracleAccounts>,
        instruction: UpdateOracleInstruction,
    ) -> Result<()> {
        update_oracle_handler(ctx, instruction)
    }

    pub fn join_oracle(
        ctx: Context<JoinOracleAccounts>,
        instruction: JoinOracleInstruction,
    ) -> Result<()> {
        join_oracle_handler(ctx, instruction)
    }

    pub fn withdraw_from_oracle(ctx: Context<WithdrawFromOracleAccounts>) -> Result<()> {
        withdraw_from_oracle_handler(ctx)
    }
    /* Oracles Instructions */
}
