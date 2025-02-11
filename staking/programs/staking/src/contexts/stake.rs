use anchor_lang::prelude::*;
use anchor_spl::token::{approve, Approve, FreezeAccount, Mint, Token, TokenAccount};

use crate::state::{UserAccountState, StakeAccountState, StakeConfig};
use crate::error::StakeError;


#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>,
    
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"stake_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        init,
        payer = user,
        space = StakeAccountState::INIT_SPACE + 8,
        seeds = [b"stake_account", nft_mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccountState>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, points: u32, amount_staked: u8, bumps: &StakeBumps) -> Result<()> {
        require!(self.user.amount_staked > self.config.max_stake, StakeError::MaxStakeReached);

        // self.stake_account 

        let clock = Clock::get()?;

        self.stake_account.set_inner(StakeAccountState {
            owner: self.user.key(),
            nft_mint: self.nft_mint.key(),
            staked_at: clock.unix_timestamp,
            bump: bumps.stake_account,
        });

        let cpi_program = self.token_program.to_account_info();

        // STUDY: Read on the Approve Account struct and when to use it
        let cpi_accounts = Approve {
            to: self.nft_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;
        
        // yayyy you get authority

        // here we want to freeze the nft
        let cpi_program = self.metadata_program.to_account_info();

        let cpi_accounts = FreezeDeletegatedAccountCpiAccounts {

        };

        let seeds = [
            b"stake_account",
            self.nft_mint.key().as_ref(),
            self.config.key().as_ref(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];
        
        // why is this different from the other CPI calls
        FreezeDelegatedAccountCpi::new(
            cpi_program,
            cpi_accounts,
        ).invoked_signed(signer_seeds)?;

        Ok(())
    }
}