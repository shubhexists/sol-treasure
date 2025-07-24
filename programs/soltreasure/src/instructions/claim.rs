use crate::error::TreasureError;
use crate::state::Treasure;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimStruct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub creator: SystemAccount<'info>,

    #[account(
        mut,
        close = winner,
        seeds = [b"game_state", creator.key().as_ref(), game_state.seed.to_le_bytes().as_ref()],
        bump = game_state.state_bump,
        has_one = creator @ TreasureError::InvalidCreator,
    )]
    pub game_state: Account<'info, Treasure>,

    #[account(mut, address = game_state.winner_pubkey)]
    pub winner: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimStruct<'info> {
    pub fn claim(&self) -> Result<()> {
        require!(
            self.game_state.game_status != true,
            TreasureError::GameIsActive
        );
        Ok(())
    }
}

pub fn claim_handler(ctx: Context<ClaimStruct>) -> Result<()> {
    ctx.accounts.claim()?;
    Ok(())
}
