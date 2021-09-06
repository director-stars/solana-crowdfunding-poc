use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program::{invoke, invoke_signed},
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack},
  pubkey::Pubkey,
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
                Ok(())
            }
        }
    }
}
