use crate::organizations::instructions::CreateOrganizationInstruction;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[account]
pub struct Organization {
    pub authority: Pubkey, // 32
    pub mint: Pubkey,      // 32
    pub fee: u64,          // 8
    pub id: i64,           //8
}

#[derive(Accounts)]
#[instruction(instruction: CreateOrganizationInstruction)]
pub struct CreateOrganizationAccounts<'info> {
    #[account(
      init,
      payer = user,
      space = 8 + 32 + 32 + 8 + 8,
      seeds = [
          "organization".as_bytes().as_ref(), 
          format!("{}", instruction.id).as_bytes().as_ref()
      ],
      bump
    )]
    pub organization: Account<'info, Organization>,
    #[account(
      init,
      payer = user,
      token::mint = mint,
      token::authority = organization,
      seeds = [organization.key().as_ref(), b"vault".as_ref()],
      bump
  )]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub mint: Account<'info, Mint>,
}

#[derive(Accounts)]
pub struct UpdateOrganizationAccounts<'info> {
    #[account(
      mut,
      constraint = organization.authority == *user.to_account_info().key
  )]
    pub organization: Account<'info, Organization>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawFromOrganizationAccounts<'info> {
    #[account(
      mut,
      seeds = [
        "organization".as_bytes().as_ref(), 
        format!("{}", organization.id).as_bytes().as_ref()
      ],
      bump,
      constraint = organization.authority == *user.to_account_info().key
    )]
    pub organization: Account<'info, Organization>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub organization_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
}
