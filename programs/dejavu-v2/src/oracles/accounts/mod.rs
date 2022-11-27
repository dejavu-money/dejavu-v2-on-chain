use anchor_lang::prelude::*;

pub enum GameStatus {
    InProgress,
    Finished = 1,
    Invalid = 2,
}

pub enum TeamResult {
    InProgress,
    Won,
    Lose,
    Draw,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Game {
    status_id: u8,
    team_id_a: u8,
    team_id_b: u8,
    team_id_a_status: u8,
    team_id_b_status: u8,
    context_reference: u8,
    context_reference_id: u64,
}

impl Game {
    pub fn status(&self) -> GameStatus {
        match self.status_id {
            1 => GameStatus::Finished,
            2 => GameStatus::Invalid,
            _ => GameStatus::InProgress,
        }
    }

    pub fn team_a_result(&self) -> TeamResult {
        Self::team_status_to_team_result(self.team_id_a_status)
    }

    pub fn team_b_result(&self) -> TeamResult {
        Self::team_status_to_team_result(self.team_id_b_status)
    }

    fn team_status_to_team_result(status: u8) -> TeamResult {
        match status {
            1 => TeamResult::Won,
            2 => TeamResult::Lose,
            3 => TeamResult::Draw,
            _ => TeamResult::InProgress,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GlobalBetItem {
    global_bets_index: u8,
    team_id_a_status: u8,
    team_id_b_status: u8,
}

#[account]
pub struct Oracle {
    pub organization: Pubkey,
    pub game: Game,
    pub global_bets: Vec<GlobalBetItem>,
    pub start_at_utc_unix: i64,
}

#[account]
pub struct PlayerBet {
    pub user: Pubkey,
}

#[derive(Accounts)]
pub struct CreateOracleAccounts {}
