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

use borsh::{BorshDeserialize, BorshSerialize};

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
            },
            WPOInstruction::Withdraw {} => {
                msg!("Instruction: Withdraw");
                Self::process_withdraw(accounts, program_id)
            },
            WPOInstruction::Initialize {} => {
                msg!("Instruction: Initialize");
                Self::process_initialize(accounts, program_id)
            },
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

        let wpo_account = next_account_info(account_info_iter)?;
        if wpo_account.owner != program_id {
            msg!("WPO account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut wpo_info = WPO::try_from_slice(&wpo_account.data.borrow())?;
        if !wpo_info.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        } 

        if wpo_info.supply == 0 {
            msg!("All NFT purchased");
            return Err(WPOError::SoldOut.into());
        }

        wpo_info.supply -= 1;
        wpo_info.serialize(&mut &mut wpo_account.data.borrow_mut()[..])?;


        let pda_account_info = next_account_info(account_info_iter)?;
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"lootbox"], program_id);
        if pda != *pda_account_info.key {
            msg!("Error: Associated address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }        

        let system_program_info = next_account_info(account_info_iter)?;

        /// move SOL from user wallet to the lootbox wallet
        invoke(
            &system_instruction::transfer(initializer.key, &pda, 2000000000),
            &[
                initializer.clone(),
                pda_account_info.clone(),
                system_program_info.clone(),
            ],
        )?;

        Ok(())
    }

    fn process_withdraw(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let reciever_account_info = next_account_info(account_info_iter)?;

        let pda_account_info = next_account_info(account_info_iter)?;
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"lootbox"], program_id);
        if pda != *pda_account_info.key {
            msg!("Error: Associated address does not match seed derivation");
            return Err(ProgramError::InvalidSeeds);
        }

        let wpo_account = next_account_info(account_info_iter)?;
        if wpo_account.owner != program_id {
            msg!("WPO account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }
        
        let wpo_info = WPO::try_from_slice(&wpo_account.data.borrow())?;
        if !wpo_info.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        } 

        if *reciever_account_info.key != wpo_info.admin_wallet {
            msg!("Reciver address not match");
            return Err(WPOError::NotRightfulReceiver.into());
        }

        let system_program_info = next_account_info(account_info_iter)?;

        /// move SOL from user wallet to the lootbox wallet
        invoke_signed(
            &system_instruction::transfer(&pda, reciever_account_info.key, **pda_account_info.lamports.borrow()),
            &[
                pda_account_info.clone(),
                reciever_account_info.clone(),
                system_program_info.clone(),
            ],
            &[&[&b"lootbox"[..], &[_bump_seed]]],
        )?;

        Ok(())
    }

    fn process_initialize(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult { 
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let wpo_account = next_account_info(account_info_iter)?;
        if wpo_account.owner != program_id {
            msg!("WPO account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut wpo_info = WPO::try_from_slice(&wpo_account.data.borrow())?;
        if wpo_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let admin_account_info = next_account_info(account_info_iter)?;

        wpo_info.is_initialized = true;
        wpo_info.supply = 10;
        wpo_info.admin_wallet = *admin_account_info.key;
        wpo_info.serialize(&mut &mut wpo_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
