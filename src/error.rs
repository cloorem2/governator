use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error,Debug,Copy,Clone)]
pub enum GovError {
    /// invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Account Already Initialized")]
    AccountAlreadyInitialized,
    #[error("NotRentExempt")]
    NotRentExempt,
}

impl From<GovError> for ProgramError {
    fn from(e: GovError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
