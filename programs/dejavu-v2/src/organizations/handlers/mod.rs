use crate::organizations::accounts::*;
use crate::organizations::instructions::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};


pub fn create_organization_handler(
  ctx: Context<CreateOrganizationAccounts>,
  instruction: CreateOrganizationInstruction,
) -> Result<()> {
  ctx.accounts.organization.authority = *ctx.accounts.user.key;
  ctx.accounts.organization.mint = ctx.accounts.mint.key();
  ctx.accounts.organization.fee = instruction.fee;
  ctx.accounts.organization.id = instruction.id;
  Ok(())
}

pub fn update_organization_handler(
  ctx: Context<UpdateOrganizationAccounts>,
  instruction: UpdateOrganizationInstruction,
) -> Result<()> {
  ctx.accounts.organization.fee = instruction.fee;
  Ok(())
}

pub fn withdraw_from_organization_handler(
  ctx: Context<WithdrawFromOrganizationAccounts>,
  instruction: WithdrawFromOrganizationInstruction,
) -> Result<()> {
  let cpi_accounts = Transfer {
      from: ctx.accounts.organization_token_account.to_account_info(),
      to: ctx.accounts.user_token_account.to_account_info(),
      authority: ctx.accounts.organization.to_account_info(),
  };

  let ctx_transfer = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
  let organization_seed = *ctx.bumps.get("organization").unwrap();

  token::transfer(
      ctx_transfer.with_signer(&[&&[
          "organization".as_bytes().as_ref(),
          &format!("{}", ctx.accounts.organization.id)
              .as_bytes()
              .as_ref(),
          &[organization_seed],
      ][..]]),
      instruction.amount,
  )?;

  Ok(())
}
