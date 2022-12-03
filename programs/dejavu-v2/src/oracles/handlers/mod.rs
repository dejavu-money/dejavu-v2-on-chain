use crate::oracles::accounts::*;
use crate::shared::*;
use anchor_spl::associated_token::{get_associated_token_address, AssociatedToken};
use anchor_spl::token::{self, Transfer};

use anchor_lang::prelude::*;

use super::instructions::{
    CreateOracleInstruction, JoinOracleInstruction, UpdateOracleInstruction,
};

pub fn create_oracle_handler(
    ctx: Context<CreateOracleAccounts>,
    instruction: CreateOracleInstruction,
) -> Result<()> {
    ctx.accounts.oracle.authority = ctx.accounts.user.key();
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
    ctx.accounts.oracle.validate_new_bets()?;

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

pub fn withdraw_from_oracle_handler(ctx: Context<WithdrawFromOracleAccounts>) -> Result<()> {
    let bet_item = match ctx
        .accounts
        .oracle_bets
        .bets
        .get(ctx.accounts.player_bet.index as usize)
    {
        Some(bet) => Ok(bet),
        None => err!(Errors::UnautorizedWithdraw),
    }?;

    if bet_item.game_result() != ctx.accounts.oracle.game.game_result()? {
        return err!(Errors::UnautorizedWithdraw);
    }

    let mut count_winners: u8 = 0;

    for bet in &ctx.accounts.oracle_bets.bets {
        if bet.game_result() == bet_item.game_result() {
            count_winners = count_winners + 1;
        }
    }

    let count_no_winners = (ctx.accounts.oracle_bets.bets.len() as u8 - count_winners) as u64;
    let vault_total_amount = (ctx.accounts.oracle.init_amount * count_no_winners)
        .checked_div(count_winners as u64)
        .ok_or(Errors::UnautorizedWithdraw)?;

    let award = ctx.accounts.oracle.init_amount + vault_total_amount;

    // transfer bet to the vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.oracle.to_account_info(),
    };

    let ctx_transfer = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    let oracle_bump = *ctx.bumps.get("oracle").unwrap();

    token::transfer(
        ctx_transfer.with_signer(&[&&[
            b"oracle".as_ref(),
            &ctx.accounts.organization.key().as_ref(),
            &format!("{}", ctx.accounts.oracle.id).as_bytes().as_ref()[..],
            &[oracle_bump],
        ][..]]),
        award,
    )?;

    Ok(())
}

pub fn update_oracle_handler(
    ctx: Context<UpdateOracleAccounts>,
    instruction: UpdateOracleInstruction,
) -> Result<()> {
    let game_status = GameStatus::from_status_id(ctx.accounts.oracle.game.status_id)?;

    if game_status == GameStatus::Finished {
        return err!(Errors::GameHasAlreadyStarted);
    }

    // verify if instruction game status is valid
    GameStatus::from_status_id(instruction.status_id)?;

    ctx.accounts.oracle.game.status_id = instruction.status_id;
    ctx.accounts.oracle.game.team_a_score = instruction.team_a_score;
    ctx.accounts.oracle.game.team_b_score = instruction.team_b_score;
    Ok(())
}
