use anchor_lang::prelude::*;
mod instructions;
pub use instructions::*;
mod error;
mod state;

declare_id!("8eYTRiHGoDg2keVLEZj59UUEmU4FmzWvneE2deLUbjmt");

#[program]
pub mod soltreasure {
    use super::*;

    pub fn initialize(
        ctx: Context<IntializeStruct>,
        seed: u64,
        gameduration: u64,
        currentfee: u64,
        initialamount: u64,
        fee: u16,
    ) -> Result<()> {
        initialize_handler(ctx, seed, gameduration, currentfee, initialamount, fee)?;
        Ok(())
    }

    pub fn participate(ctx: Context<ParticpantStruct>) -> Result<()> {
        participate_handler(ctx)?;
        Ok(())
    }

    pub fn claim(ctx: Context<ClaimStruct>) -> Result<()> {
        claim_handler(ctx)?;
        Ok(())
    }
}
