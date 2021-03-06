use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};



/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WPO {
    pub is_initialized: bool,
    pub supply: u8,
    pub admin_wallet: Pubkey,
}

impl IsInitialized for WPO {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}