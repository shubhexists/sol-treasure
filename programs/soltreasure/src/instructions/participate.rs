use crate::error::TreasureError;
use crate::state::Treasure;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct ParticpantStruct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub creator: SystemAccount<'info>,

    #[account(mut, address = game_state.fee_account)]
    pub fee_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"game_state", creator.key().as_ref(), game_state.seed.to_le_bytes().as_ref()],
        bump = game_state.state_bump,
        has_one = creator @ TreasureError::InvalidCreator,
    )]
    pub game_state: Account<'info, Treasure>,
    pub system_program: Program<'info, System>,
}

impl<'info> ParticpantStruct<'info> {
    pub fn participate(&mut self) -> Result<()> {
        require!(
            self.game_state.game_status == true,
            TreasureError::GameNotActive
        );

        require!(
            self.signer.lamports() > self.game_state.current_fee,
            TreasureError::InvalidFee
        );

        let clock = Clock::get()?;
        let current_time: u64 = clock.unix_timestamp.try_into()?;
        let time_elapsed = current_time - self.game_state.last_transaction;

        if time_elapsed > self.game_state.game_duration {
            self.game_state.game_status = false;
            return Ok(());
        }

        let game_fee: u64 = self
            .game_state
            .fees
            .try_into()
            .map_err(|_| TreasureError::ArithmeticError)?;

        let fees = game_fee
            .checked_mul(self.game_state.current_fee)
            .ok_or(TreasureError::ArithmeticError)?
            .checked_div(10000)
            .ok_or(TreasureError::ArithmeticError)?;

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.signer.to_account_info(),
                    to: self.game_state.to_account_info(),
                },
            ),
            self.game_state.current_fee,
        )?;

        let cpi_program = self.system_program.to_account_info();
        let cpi_instructions = Transfer {
            from: self.game_state.to_account_info(),
            to: self.fee_account.to_account_info(),
        };

        let key = self.creator.key();
        let seeds = self.game_state.seed.to_le_bytes();
        let s_seeds: &[&[u8]] = &[
            b"game_state",
            key.as_ref(),
            seeds.as_ref(),
            &[self.game_state.state_bump],
        ];

        let signer_seeds = &[s_seeds];

        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_instructions, signer_seeds);
        transfer(cpi_context, fees)?;

        let clock = Clock::get()?;
        self.game_state.last_transaction = clock.unix_timestamp.try_into()?;

        self.game_state.winner_pubkey = self.signer.key();
        let increment_transactions = self
            .game_state
            .total_transactions
            .checked_add(1)
            .ok_or(TreasureError::ArithmeticError)?;
        self.game_state.total_transactions = increment_transactions;
        let new_fee = self
            .game_state
            .current_fee
            .checked_mul(2)
            .ok_or(TreasureError::ArithmeticError)?;
        self.game_state.current_fee = new_fee;

        Ok(())
    }
}

pub fn participate_handler(ctx: Context<ParticpantStruct>) -> Result<()> {
    ctx.accounts.participate()?;
    Ok(())
}
