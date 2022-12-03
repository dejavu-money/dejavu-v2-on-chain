use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    OracleInvalidGameStatus,
    OracleInstructionInvalid,
    OracleTeamStatusInvalid,
    UserTokenAccountInvalid,
    GameHasAlreadyStarted,
    UnautorizedWithdraw,
    GameHasNotFinished,
    BetGameResultInvalid,
}
