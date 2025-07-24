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

      pub fn gamestateset(&mut self,seed:u64,gameduration:u64,currentfee:u64,state_bump:u8,pool_bump:u8) -> Result<()> {

        require!(gameduration > 119, TreasureError::InvalidGameDuration);

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;
        
        self.game_state.set_inner(Treasure 
            { 
             creator: self.signer.key(),
              winner_pubkey: self.signer.key(),
              last_transaction: current_time, 
              current_fee: currentfee,
              total_transactions: 0, 
              game_duration: gameduration, 
              game_status: true,
              seed: seed,
              state_bump: state_bump,
              pool_bump: pool_bump
             });

             // 120 second // tab time - last > duration 
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



pub fn handler(ctx:Context<IntializeStruct>,seed:u64, gameduration:u64,currentfee:u64,initialamount:u64) -> Result<()>{

      ctx.accounts.gamestateset(seed,gameduration,currentfee,ctx.bumps.game_state,ctx.bumps.treasure_pool)?;
      ctx.accounts.startgame(initialamount)?;

      Ok(())
}