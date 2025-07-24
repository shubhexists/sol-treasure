use anchor_lang::prelude::*;





#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Treasure{
   pub winner_pubkey:Pubkey,
   pub last_transaction:u64,
   pub current_fee:u64,
   pub total_transactions:u64,
   pub game_duration:u64,
   pub game_status: bool,
}