use solana_program::{
    program_pack::{Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref,array_ref,array_refs,mut_array_refs};

pub struct State {
    pub seat1: Pubkey,
    pub sub_gov1: Pubkey,
    pub seat2: Pubkey,
    pub sub_gov2: Pubkey,
}

impl Sealed for State {}

impl Pack for State {
    const LEN: usize = 128;
    fn unpack_from_slice(src: &[u8]) -> Result<Self,ProgramError> {
        let src = array_ref![src,0,State::LEN];
        let (
            seat1,
            sub_gov1,
            seat2,
            sub_gov2,
        ) = array_refs![src, 32, 32, 32, 32];
        Ok(State {
            seat1: Pubkey::new_from_array(*seat1),
            sub_gov1: Pubkey::new_from_array(*sub_gov1),
            seat2: Pubkey::new_from_array(*seat2),
            sub_gov2: Pubkey::new_from_array(*sub_gov2),
        })
    }

    fn pack_into_slice(&self,dst: &mut [u8]) {
        let dst = array_mut_ref![dst,0,State::LEN];
        let (
            seat1_dst,
            sub_gov1_dst,
            seat2_dst,
            sub_gov2_dst,
        ) = mut_array_refs![dst,32,32,32,32];
        let State {
            seat1,
            sub_gov1,
            seat2,
            sub_gov2,
        } = self;
        seat1_dst.copy_from_slice(seat1.as_ref());
        sub_gov1_dst.copy_from_slice(sub_gov1.as_ref());
        seat2_dst.copy_from_slice(seat2.as_ref());
        sub_gov2_dst.copy_from_slice(sub_gov2.as_ref());
    }
}
