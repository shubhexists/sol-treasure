use crate::error::TreasureError;
use crate::state::Treasure;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct IntializeStruct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = Treasure::INIT_SPACE + Treasure::DISCRIMINATOR.len(),
        seeds = [b"game_state",signer.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub game_state: Account<'info, Treasure>,

    pub fee_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> IntializeStruct<'info> {
    pub fn gamestateset(
        &mut self,
        seed: u64,
        gameduration: u64,
        starting_fee: u64,
        state_bump: u8,
        fees: u16,
    ) -> Result<()> {
        require!(gameduration > 119, TreasureError::InvalidGameDuration);
        require!(fees < 1000, TreasureError::FeesTooHigh);

        let clock = Clock::get()?;
        let current_time: u64 = clock.unix_timestamp.try_into()?;

        self.game_state.set_inner(Treasure {
            creator: self.signer.key(),
            fees: fees,
            fee_account: self.fee_account.key(),
            winner_pubkey: self.signer.key(),
            last_transaction: current_time,
            current_fee: starting_fee,
            total_transactions: 0,
            game_duration: gameduration,
            game_status: true,
            seed: seed,
            state_bump: state_bump,
        });

        Ok(())
    }

    pub fn startgame(&self, initialamount: u64) -> Result<()> {
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.signer.to_account_info(),
                    to: self.game_state.to_account_info(),
                },
            ),
            initialamount,
        )?;

        Ok(())
    }
}

pub fn initialize_handler(
    ctx: Context<IntializeStruct>,
    seed: u64,
    gameduration: u64,
    currentfee: u64,
    initialamount: u64,
    fee: u16,
) -> Result<()> {
    ctx.accounts
        .gamestateset(seed, gameduration, currentfee, ctx.bumps.game_state, fee)?;

    if initialamount > 0 {
        ctx.accounts.startgame(initialamount)?;
    }

    Ok(())
}
