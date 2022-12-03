use super::instructions::{CreateOracleInstruction, JoinOracleInstruction};
use crate::organizations::accounts::Organization;
use crate::shared::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(PartialEq)]
pub enum GameStatus {
    InProgress = 0,
    Finished = 1,
    Invalid = 2,
    Unknown,
}

impl GameStatus {
    pub fn from_status_id(id: u8) -> Result<Self> {
        match id {
            0 => Ok(Self::InProgress),
            1 => Ok(Self::Finished),
            2 => Ok(Self::Invalid),
            _ => err!(Errors::OracleInvalidGameStatus),
        }
    }

    pub fn get_status_id(&self) -> Result<u8> {
        match self {
            Self::InProgress => Ok(0),
            Self::Finished => Ok(1),
            Self::Invalid => Ok(2),
            _ => err!(Errors::OracleInvalidGameStatus),
        }
    }
}

#[derive(PartialEq)]
pub enum GameResult {
    TeamAWin,
    TeamBWin,
    Draw,
    NoResult,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Game {
    pub status_id: u8,
    pub team_a_id: u8,
    pub team_b_id: u8,
    pub team_a_score: u8,
    pub team_b_score: u8,
    pub context_reference: u8,
    pub context_reference_id: i64,
}

impl Game {
    pub fn game_status(&self) -> Result<GameStatus> {
        GameStatus::from_status_id(self.status_id)
    }

    pub fn game_result(&self) -> Result<GameResult> {
        let game_status = self.game_status()?;

        if game_status != GameStatus::Finished {
            return Ok(GameResult::NoResult);
        }

        if self.team_a_score > self.team_b_score {
            return Ok(GameResult::TeamAWin);
        } else if self.team_b_score > self.team_a_score {
            return Ok(GameResult::TeamBWin);
        } else {
            return Ok(GameResult::Draw);
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BetItem {
    pub withdrew: bool,
    pub global_bets_index: u8,
    pub game_result: u8,
}

impl BetItem {
    pub fn game_result(&self) -> GameResult {
        match self.game_result {
            0 => GameResult::TeamAWin,
            1 => GameResult::TeamBWin,
            2 => GameResult::Draw,
            _ => GameResult::NoResult,
        }
    }
}

#[account]
pub struct Oracle {
    pub authority: Pubkey,
    pub organization: Pubkey,   // 32
    pub game: Game,             // 14
    pub start_at_utc_unix: i64, // 8
    pub id: i64,                // 8
    pub init_amount: u64,       //8
}

impl Oracle {
    pub fn validate_new_bets(&self) -> Result<()> {
        let game_status = self.game.game_status()?;
        let current_timestamp = Clock::get()?.unix_timestamp;

        if current_timestamp >= self.start_at_utc_unix || game_status == GameStatus::Finished {
            return err!(Errors::GameHasAlreadyStarted);
        }

        Ok(())
    }

    // pub fn withdraw(
    //     &self,
    //     _user: &Pubkey,
    //     player_bet: &PlayerBet,
    //     bet_items: &Vec<BetItem>,
    // ) -> Result<()> {
    //     let mut count_winners: u8 = 0;
    //     let bet_item = match bet_items.get(player_bet.index as usize) {
    //         Some(bet) => Ok(bet),
    //         None => err!(Errors::UnautorizedWithdraw),
    //     }?;

    //     if bet_item.game_result() != self.game.game_result()?
    //         || self.game.game_result() == GameResult::NoResult
    //     {
    //         return err!(Errors::UnautorizedWithdraw);
    //     }

    //     for bet in bet_items {
    //         if bet.game_result() == bet_item.game_result() {
    //             count_winners = count_winners + 1;
    //         }
    //     }

    //     let count_no_winners = (bet_items.len() as u8 - count_winners) as u64;
    //     let vault_total_amount = (self.init_amount * count_no_winners)
    //         .checked_div(count_winners as u64)
    //         .ok_or(Errors::UnautorizedWithdraw)?;

    //     let award = self.init_amount + vault_total_amount;

    //     Ok(())
    // }
}

#[account]
pub struct Bets {
    pub bets: Vec<BetItem>, // 4 bytes (length)
}

impl Bets {
    pub fn calculate_bets_new_space(&self) -> usize {
        let delimiter_space = 8;
        let vec_length_space = 4;
        let global_bet_space = 4;
        let new_space = global_bet_space * self.bets.len();

        delimiter_space + new_space + global_bet_space + vec_length_space
    }
}

#[account]
pub struct PlayerBet {
    pub index: u8,      // 1
    pub user: Pubkey,   // 32
    pub oracle: Pubkey, // 32
}

impl PlayerBet {}

#[derive(Accounts)]
#[instruction(instruction: CreateOracleInstruction)]
pub struct CreateOracleAccounts<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 14 + 8 + 8 + 8,
        seeds = [
            "oracle".as_bytes().as_ref(), 
            organization.key().as_ref(),
            format!("{}", instruction.id).as_bytes().as_ref()
        ],
        bump,
        constraint = organization.authority.key() == user.key()
      )]
    pub oracle: Account<'info, Oracle>,
    #[account(
        init,
        payer = user,
        space = 8 + 4,
        seeds = [
            oracle.key().as_ref(),
            b"bets".as_ref()
        ],
        bump,
      )]
    pub oracle_bets: Account<'info, Bets>,
    pub organization: Account<'info, Organization>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        constraint = organization.mint.key() == mint.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = user,
        token::mint = mint,
        token::authority = oracle,
        seeds = [oracle.key().as_ref(), b"vault".as_ref()],
        bump
      )]
    pub vault: Box<Account<'info, TokenAccount>>,
}

#[derive(Accounts)]
#[instruction(instruction: JoinOracleInstruction)]
pub struct JoinOracleAccounts<'info> {
    #[account(
        mut,
        seeds = [
            "oracle".as_bytes().as_ref(), 
            organization.key().as_ref(),
            format!("{}", oracle.id).as_bytes().as_ref()
        ],
        bump
      )]
    pub oracle: Account<'info, Oracle>,
    #[account(
        init,
        space = 8 + 1 + 32 + 32,
        payer = payer,
        seeds = [
            oracle.key().as_ref(),
            format!("player-{}", instruction.bet_index).as_bytes().as_ref()
        ],
        bump,
      )]
    pub player_bet: Account<'info, PlayerBet>,
    #[account(
        mut,
        seeds = [oracle.key().as_ref(), b"bets"], 
        bump,
        realloc = oracle_bets.calculate_bets_new_space(),
        realloc::payer = payer,
        realloc::zero = false,
    )]
    pub oracle_bets: Account<'info, Bets>,
    pub organization: Account<'info, Organization>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        constraint = organization.mint.key() == mint.key(),
        constraint = organization.key() == oracle.organization.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [oracle.key().as_ref(), b"vault".as_ref()],
        bump
      )]
    pub vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
}

#[derive(Accounts)]
pub struct UpdateOracleAccounts<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(
        constraint = organization.key() == oracle.organization.key(),
        constraint = oracle.authority.key() == user.key()
    )]
    pub organization: Account<'info, Organization>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawFromOracleAccounts<'info> {
    #[account(
        mut,
        seeds = [
            "oracle".as_bytes().as_ref(), 
            organization.key().as_ref(),
            format!("{}", oracle.id).as_bytes().as_ref()
        ],
        bump
      )]
    pub oracle: Account<'info, Oracle>,
    #[account(
        constraint = player_bet.oracle.key() == oracle.key(),
        constraint = player_bet.user.key() == user.key()
      )]
    pub player_bet: Account<'info, PlayerBet>,
    #[account(
        seeds = [oracle.key().as_ref(), b"bets"], 
        bump
    )]
    pub oracle_bets: Account<'info, Bets>,
    #[account(
        constraint = organization.key() == oracle.organization.key(),
    )]
    pub organization: Account<'info, Organization>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    #[account(
        constraint = organization.mint.key() == mint.key(),
        constraint = organization.key() == oracle.organization.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [oracle.key().as_ref(), b"vault".as_ref()],
        bump
      )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
}
