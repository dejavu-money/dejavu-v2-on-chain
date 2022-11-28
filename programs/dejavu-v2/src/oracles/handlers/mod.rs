use crate::oracles::accounts::*;
use crate::shared::*;
use anchor_spl::associated_token::{get_associated_token_address, AssociatedToken};
use anchor_spl::token::{self, Transfer};

use anchor_lang::prelude::*;

use super::instructions::{CreateOracleInstruction, JoinOracleInstruction};

pub fn create_oracle_handler(
    ctx: Context<CreateOracleAccounts>,
    instruction: CreateOracleInstruction,
) -> Result<()> {
    ctx.accounts.oracle.organization = ctx.accounts.organization.key();
    ctx.accounts.oracle.id = instruction.id;
    ctx.accounts.oracle.game.status_id = 0;
    ctx.accounts.oracle.game.team_a_id = instruction.team_id_a;
    ctx.accounts.oracle.game.team_b_id = instruction.team_id_b;
    ctx.accounts.oracle.game.context_reference = instruction.context_reference;
    ctx.accounts.oracle.game.context_reference_id = instruction.context_reference_id;
    ctx.accounts.oracle.start_at_utc_unix = instruction.start_at_utc_unix;
    ctx.accounts.oracle.init_amount = instruction.init_amount;

    Ok(())
}

pub fn join_oracle_handler(
    ctx: Context<JoinOracleAccounts>,
    instruction: JoinOracleInstruction,
) -> Result<()> {
    instruction.validate()?;
    ctx.accounts.oracle.validate_bets()?;

    let user_token_account =
        get_associated_token_address(ctx.accounts.user.key, &ctx.accounts.mint.key());

    if user_token_account != ctx.accounts.user_token_account.key() {
        return err!(Errors::UserTokenAccountInvalid);
    }

    ctx.accounts.player_bet.index = instruction.bet_index;
    ctx.accounts.player_bet.user = ctx.accounts.user.key();
    ctx.accounts.player_bet.oracle = ctx.accounts.oracle.key();

    // add bet

    ctx.accounts.oracle_bets.bets.push(instruction.bet_item());

    // transfer bet to the vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let ctx_transfer = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(ctx_transfer, ctx.accounts.oracle.init_amount)?;

    Ok(())
}
