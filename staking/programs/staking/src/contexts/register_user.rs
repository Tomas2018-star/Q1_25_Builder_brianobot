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
        seeds = [b"user_account", user.key().as_ref()], // linking the user key in the seeds
        // constaints the user to having only one user_account, the user_account seed does not count
        // since it is static (constant) for all the accounts
        bump,
    )]
    pub user_account: Account<'info, UserAccountState>,
    pub system_program: Program<'info, System>, // needed because we are initializing an account
}

impl<'info> RegisterUser<'info> {
    pub fn init(&mut self, bumps: &RegisterUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccountState {
            // no need to initiate the points and amount_staked the default values would make more sense 
            // since this os the first time the user is registering on the staking protocol
            // points,
            // amount_staked,
            bump: bumps.user_account,
            ..Default::default()
        });

        msg!("User Account State: {:?}", self.user_account);

        Ok(())
    }
}