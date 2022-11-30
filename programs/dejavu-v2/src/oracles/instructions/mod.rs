use crate::shared::*;
use anchor_lang::prelude::*;

use super::accounts::{BetItem, GameStatus};
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateOracleInstruction {
    pub id: i64,
    pub team_id_a: u8,
    pub team_id_b: u8,
    pub context_reference: u8,
    pub context_reference_id: i64,
    pub start_at_utc_unix: i64,
    pub init_amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct JoinOracleInstruction {
    pub bet_index: u8,
    pub game_result: u8,
}

impl JoinOracleInstruction {
    pub fn validate(&self) -> Result<()> {
        // TEAM_STATUSES

        // 0 -> TeamAWin
        // 1 -> TeamBWin
        // 2 -> DRAW

        if self.game_result > 2 {
            return err!(Errors::BetGameResultInvalid);
        }

        Ok(())
    }

    pub fn bet_item(&self) -> BetItem {
        BetItem {
            global_bets_index: self.bet_index,
            game_result: self.game_result,
            withdrew: false,
        }
    }
}


#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawFromOracleInstruction {
    
}
