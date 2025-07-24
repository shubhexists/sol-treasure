use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::error::TreasureError;
use crate::state::Treasure;






// particpant struct
#[derive(Accounts)]
pub struct ParticpantStruct <'info> {

    #[account(mut)]
    pub signer:Signer<'info>,

    // creator
    pub creator:SystemAccount<'info>,

    // game_state account
    #[account(
        mut,
        seeds = [b"game_state", creator.key().as_ref(), game_state.seed.to_le_bytes().as_ref()],
        bump = game_state.state_bump,
        has_one = creator @ TreasureError::InvalidCreator,
    )]
    pub game_state:Account<'info,Treasure>,
   
   // pool account
   #[account(
    mut,
    seeds = [b"treasure_pool", creator.key().as_ref(),game_state.seed.to_le_bytes().as_ref()],
    bump = game_state.pool_bump
   )]
   pub treasure_pool: SystemAccount<'info>,
   

   // programs
   pub system_program: Program<'info,System> 
}




impl <'info> ParticpantStruct <'info> {

 pub fn participate(&self) -> Result<()> {
      
      require!(self.signer.lamports() > self.game_state.current_fee, TreasureError::InvalidFee);

      transfer(CpiContext::new(
        self.system_program.to_account_info(), 
        Transfer { 
            from: self.signer.to_account_info(), 
            to: self.treasure_pool.to_account_info() 
        }
      ), self.game_state.current_fee)?;

      Ok(())
 }
}




