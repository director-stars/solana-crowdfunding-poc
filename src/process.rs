use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program::{invoke, invoke_signed},
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack},
  pubkey::Pubkey,
  system_instruction,
  sysvar::{rent::Rent, Sysvar},
};

use crate::{error::WPOError, instruction::WPOInstruction, state::WPO};


pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = WPOInstruction::unpack(instruction_data)?;

        match instruction {
            WPOInstruction::Purchase {amount} => {
                msg!("Instruction: Purchase");
                Self::process_purchase(accounts, amount, program_id)
            }
        }
    }

    fn process_purchase(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        
        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        /// get PDA for gathering funds
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"lootbox"], program_id);

        /// move SOL from user wallet to the lootbox wallet
        invoke(
            &system_instruction::transfer(initializer.key, pda.key, 2000000000),
            &[
                initializer.clone(),
                pda.clone(),
                system_program.clone(),
            ],
        )?;

        Ok(())
    }
}
