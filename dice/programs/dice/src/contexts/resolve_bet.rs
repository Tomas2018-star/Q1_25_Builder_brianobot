use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,  
};

use crate::state::Bet;
use crate::error::DiceError;


#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    #[account(mut)]
    pub player: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info,  Bet>,
    #[account( 
        address = solana_program::sysvar::instructions::ID
    )]
    /// CHECK: this is the instructions sysvar acccount
    pub instructions_sysvar: AccountInfo<'info>,
    // this sysvar allows for instruction introspection
    pub system_program: Program<'info, System>,
}


impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // this allows us to get the first instruction of the current transaction
        // TODO: if the Instruction Sysvar is constant, why do we need to pass the account info into functions like the one below
        let ix = load_instruction_at_checked(0, &self.instructions_sysvar.to_account_info())?;

        // this checks that the instruction is from the current program
        require_keys_eq!(ix.program_id, ed25519_program::ID, DiceError::InvalidProgramID);

        // this checks that there is no account in the first instruction
        require_eq!(ix.accounts.len(), 0, DiceError::InvalidAccountCount);

        // unpack the binary data from the instruction and then parse the signatures into a vec
        // this can be used to ensure an instruction was signed by a particular key
        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;
        
        // ensure the is only one signer for the instruction
        require_eq!(signatures.len(), 1, DiceError::CustomError);

        let signature = &signatures[0];

        // checks that the signature is verifiable
        require!(signature.is_verifiable, DiceError::UnverifiableSignature);

        // checks that the signature matches that of the signer in the account struct
        require!(signature.public_key == Some(self.house.key()), DiceError::UnverifiableSignature);

        // checks that the signaure in the instruction matches the sig passed into the instruction
        require!(signature.signature.unwrap() == sig, DiceError::UnverifiableSignature);

        // this checks that the signature message matches the bet data
        require!(signature.message.as_ref().unwrap().eq(&self.bet.to_slice()), DiceError::CustomError);

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        // in order to make working the sig less painful, we hash it to get a 32 byte array
        // think of the hash as a public in the case here (basically to make life easier)
        let hash = hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);

        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // here, upper is added to lower and in case of an overflow, 
        // it is allowed to wrapped back from 
        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        // this winning logic here is intentionally for educational purppose, it is silly at best
        if self.bet.roll > roll {
            let payout = (self.bet.amount as u128)
                .checked_mul(10000 - 150 as u128)
                .unwrap()
                .checked_div(self.bet.roll as u128 - 1)
                .unwrap()
                .checked_div(100)
                .unwrap() as u64;
            
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };
            // since the vault can not be a signer, it is a PDA, we derive the seeds needed to
            // sign with the program on behalf of the vault

            let seeds = [
                b"vault", 
                self.house.to_account_info().key.as_ref(),
                &[bumps.vault],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, payout)?;
        }

        Ok(())
    }
}