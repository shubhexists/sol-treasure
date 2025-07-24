use anchor_lang::prelude::*;
mod instructions;
pub use instructions::*;
mod state;
mod error;
declare_id!("8eYTRiHGoDg2keVLEZj59UUEmU4FmzWvneE2deLUbjmt");

#[program]
pub mod soltreasure {
    use super::*;

    pub fn initialize(ctx: Context<IntializeStruct>,seed:u64, gameduration:u64,currentfee:u64,initialamount:u64) -> Result<()> {

        instructions::initialize::handler(ctx, seed, gameduration, currentfee, initialamount)?;
        Ok(())
    }


    pub fn participate(ctx: Context<ParticpantStruct>) -> Result<()> {
          instructions::participate::handler(ctx)?;

          Ok(())
    }


    pub fn claim(ctx: Context<ClaimStruct>) -> Result<()>{
        instructions::claim::handler(ctx)?;

        Ok(())
    }
}

