use anchor_lang::prelude::*;



#[error_code]
pub enum TreasureError {
    #[msg("Invalid game duration! duration is not vaild it should be greater the 120 seconds!")]
    InvalidGameDuration,
    #[msg("Invalid fee amount!")]
    InvalidFee,
    #[msg("Invalid initial amount it should be greater then 2 sol")]
    InvalidInitialAmount,
    #[msg("Invalid mint")]
    InvalidMint,
}