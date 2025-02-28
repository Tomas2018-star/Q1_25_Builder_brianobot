use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::state::{UserAccountState, StakeAccountState, StakeConfig};
use crate::error::StakeError;

/*
    One Majoy difference between staking and nft marketplace is that the nft is not transferred from the owners (holder's)
    wallet, but while it is staked, it can not be transferred from the wallet of the holder

*/

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // this is needed because we need to know the nft the user is staking in the first place
    // when interacting with any type of token, it is good practise to always require it mint for verification
    // purposes
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint, // accounts are treated as pubkey due to rust Deref<Target = Pubkey>
        associated_token::authority = user, // same rules applies here
        // so technically associated_token::authority = user.key() is still correct
        associated_token::token_program = token_program,
    )]
    // this is needed to store the NFT for the user, actually this is needed to hold the nft in question
    pub nft_mint_ata: Account<'info, TokenAccount>,
    // this is needed to verify the authenticty of the nft in question
    pub collection_mint: Account<'info, Mint>, 
    #[account(
        // the Metadata account for all NFT as defined by metaplex must be derived from the ffs seeds
        // "metadata", metadata_program_id and the mint address of the metadata account
        seeds = [
            b"metadata",
            nft_mint.key().as_ref(),
            metadata_program.key().as_ref(),
        ], // the seeds for the metadataAccount is predefined 
        bump,
        // TODO: change the line below to metadata_program and see it the test still works
        // instead of the defaul Crate::ID which is our program id
        seeds::program = metadata_program.key(), // this tells anchor the use the metadata_program key to derive the address
        // TODO: look into the collection verification constraint more and deeply
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        // the above line checks that the metadata collection matches the collection_mint provided
        constraint = metadata.collection.as_ref().unwrap().verified == true
        // this checks that metadata collection is verified
        // TODO: How are metadata verified in Metaplex
    )] 
    pub metadata: Account<'info, MetadataAccount>, // TODO: why do we need this

    #[account(
        seeds = [
            b"metadta",
            nft_mint.key().as_ref(),
            metadata_program.key().as_ref(),
            b"edition",
        ],
        bump,
        seeds::program = metadata_program.key(),
    )]
    // same as the metadata but with the seed called edition
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"stake_config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccountState>,

    #[account(
        init,
        payer = user,
        space = StakeAccountState::INIT_SPACE + 8,
        // here the config key seeds is not strictly needed
        seeds = [b"stake_account", nft_mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccountState>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        // this ensures that the user has not gone over the max staked
        require!(self.user_account.amount_staked < self.config.max_stake, StakeError::MaxStakeReached);

        self.user_account.amount_staked += 1;

        // this is a sysvar, 
        let clock = Clock::get()?;

        self.stake_account.set_inner( StakeAccountState {
            owner: self.user.key(),
            nft_mint: self.nft_mint.key(),
            staked_at: clock.unix_timestamp,
            bump: bumps.stake_account,
        });

        let cpi_program = self.token_program.to_account_info();

        // STUDY: Read on the Approve Account struct and when to use it
        // this allows another authority to manage your token
        let cpi_accounts = Approve {
            to: self.nft_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?; // Amount for NFT is always one
        
        // yayyy at this point in the process, you get authority over the token (NFT)

        // here we want to freeze the nft
        let cpi_program = &self.metadata_program.to_account_info();
        // TODO: why is the metadata_program needed to freeze an token 

        let cpi_accounts = FreezeDelegatedAccountCpiAccounts {
            delegate: &self.stake_account.to_account_info(),
            token_account: &self.nft_mint_ata.to_account_info(),
            edition: &self.edition.to_account_info(),
            mint: &self.nft_mint.to_account_info(),
            token_program: &self.token_program.to_account_info(),
        };
 
        let seeds = [ 
            b"stake_account",
            // TODO: Possible overhead when using to_account_info()
            self.nft_mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];
        
        // why is this different from the other CPI calls
        FreezeDelegatedAccountCpi::new(
            cpi_program,
            cpi_accounts,
        ).invoke_signed(signer_seeds)?;

        Ok(())
    }

}