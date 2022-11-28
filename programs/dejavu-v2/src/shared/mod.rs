use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    OracleInstructionInvalid,
    OracleTeamStatusInvalid,
    UserTokenAccountInvalid,
    GameHasAlreadyStarted,
    UnautorizedWithdraw,
    GameHasNotFinished,
    BetGameResultInvalid,
}
