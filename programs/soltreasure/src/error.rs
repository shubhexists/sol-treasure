use anchor_lang::prelude::*;

#[error_code]
pub enum TreasureError {
    #[msg("Invalid game duration! duration is not vaild it should be greater the 120 seconds!")]
    InvalidGameDuration,
    #[msg("Invalid fee amount!")]
    InvalidFee,
    #[msg("Invalid initial amount it should be greater then 2 sol")]
    InvalidInitialAmount,
    #[msg("Invalid creator account")]
    InvalidCreator,
    #[msg("Game is not active anymore")]
    GameNotActive,
    #[msg("Game is still active you can not claim")]
    GameIsActive,
    #[msg("Arithmetic Error")]
    ArithmeticError,
    #[msg("Fee can't be more than 1000 Basis Points")]
    FeesTooHigh,
}
