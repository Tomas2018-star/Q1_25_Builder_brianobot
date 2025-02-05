use anchor_lang::prelude::*;

use crate::state::UserAccountState;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        payer = user,
        space = UserAccountState::INIT_SPACE + 8,
        seeds = [b"user_account", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccountState>,

    pub system_program: Program<'info, System>,
}

impl<'info> RegisterUser<'info> {
    pub fn init(&mut self, points: u32, amount_staked: u8, bumps: &RegisterUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccountState {
            points,
            amount_staked,
            bump: bumps.user_account,
        });

        Ok(())
    }
}