use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::error::TreasureError;
use crate::state::Treasure;






// claim struct

#[derive(Accounts)]
pub struct ClaimStruct <'info> {

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
   

 /// Winner account to receive lamports
   #[account(mut, address = game_state.winner_pubkey)]
   pub winner: SystemAccount<'info>,


   // programs
   pub system_program: Program<'info,System> 
}


impl <'info> ClaimStruct <'info> {
    
 pub fn claim(&self,seed:u64) -> Result<()>{
    
    require!(self.game_state.game_status == true, TreasureError::GameIsActive);

    let signer_seed:&[&[&[u8]]] = &[&[
        b"treasure_pool",
        self.game_state.creator.as_ref(),
        &self.game_state.seed.to_be_bytes()[..],
        &[self.game_state.pool_bump]
        ]];


    transfer(
        CpiContext::new_with_signer(
            self.system_program.to_account_info(),
             Transfer { 
                from: self.treasure_pool.to_account_info(), 
                to: self.winner.to_account_info(), 
            }, 
            signer_seed
        ), 
            self.treasure_pool.lamports()
        )

    }
}