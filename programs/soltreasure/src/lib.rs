use anchor_lang::prelude::*;

declare_id!("8eYTRiHGoDg2keVLEZj59UUEmU4FmzWvneE2deLUbjmt");

#[program]
pub mod soltreasure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
