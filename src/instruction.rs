use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use arrayref::{array_mut_ref,array_ref,array_refs,mut_array_refs};

use crate::error::GovError::InvalidInstruction;

pub enum GovInstruction {

    /// we need an initial initializer
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` i've got to start the whole thing
    /// 1. `[writable]` state pda that we will create
    /// 2. `[]` system program since create_account?
    /// we need to make the rep coin, and a currency coin with atas
    /// we need to provide for seats with associated sub-daos that can change
    ///   the sitter
    InitDao {
        seed_bump: u8
    },

    TakeSeat1 { },
    TakeSeat2 { },

}

impl GovInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // let (&discrim, data) = array_refs![input, 1; ..;];
        let (discrim, data) = input.split_first().ok_or(InvalidInstruction)?;
        // let (bump,rest2) = rest.split_first().ok_or(InvalidInstruction)?;
        Ok(match discrim {
            0 => {
                Self::InitDao {
                    // seed_bump: Self::unpack_bump(rest)?,
                    seed_bump: data[0]
                }
            },
            1 => Self::TakeSeat1 {

            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_bump(input: &[u8]) -> Result<u8, ProgramError> {
        let bump = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u8::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(bump)
    }
}
