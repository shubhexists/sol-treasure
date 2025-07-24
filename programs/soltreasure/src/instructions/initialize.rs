use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use crate::state::Treasure;
use crate::error::TreasureError;


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct IntializeStruct <'info> {

    // signer
    #[account(mut)]
    pub signer:Signer<'info>,
    
    // State
    #[account(
        init,
        payer = signer,
        space = Treasure::INIT_SPACE + Treasure::DISCRIMINATOR.len(),
        seeds = [b"game_state",signer.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub game_state:Account<'info,Treasure>,

    // POOL
    #[account(
        mut,
        seeds = [b"treasure_pool", signer.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
     )]

     pub treasure_pool:SystemAccount<'info>,

     // programs
     pub system_program:Program<'info,System>
}


impl <'info> IntializeStruct <'info>  {

      pub fn gamestateset(&mut self,gameduration:u64) -> Result<()> {

        require!(gameduration > 119, TreasureError::InvalidGameDuration);

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;
        
        self.game_state.set_inner(Treasure 
            { winner_pubkey: self.signer.key(),
              last_transaction: current_time, 
              current_fee: 100_000_000, 
              total_transactions: 0, 
              game_duration: gameduration, 
              game_status: true
             });

        Ok(())
      } 


      pub fn startgame(&self, initialamount:u64) -> Result<()>{
        
        require!(initialamount > 2_000_000_000, TreasureError::InvalidInitialAmount);

             transfer(
                CpiContext::new(self.system_program.to_account_info(),
                Transfer { 
                    from: self.signer.to_account_info(), 
                    to: self.treasure_pool.to_account_info()
                }), 
                initialamount
            )?;

            Ok(())
      }
}