// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum WPOError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<WPOError> for ProgramError {
    fn from(e: WPOError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
