use solana_program::{
    account_info::{next_account_info,AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::{invoke, invoke_signed},
    bpf_loader_upgradeable,
    system_program
};

use crate::{
    instruction::GovInstruction,
    error::GovError,
    state::State
};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction = GovInstruction::unpack(instruction_data)?;
        match instruction {
            GovInstruction::InitDao { seed_bump } => {
                msg!("Instruction: InitDao");
                Self::process_init_dao(accounts, seed_bump, program_id)
            },
            GovInstruction::TakeSeat1 {} => {
                msg!("Instruction: TakeSeat1");
                Self::take_seat1(accounts, program_id)
            },
            GovInstruction::TakeSeat2 {} => {
                msg!("Instruction: TakeSeat2");
                Self::take_seat1(accounts, program_id)
            }
        }
    }

    fn process_init_dao(
        accounts: &[AccountInfo],
        seed_bump: u8,
        program_id: &Pubkey,
    ) -> ProgramResult {
        // const THIS_SEED: &str = "this_seed";
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let state_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        assert!(payer.is_signer);
        assert!(payer.is_writable);
        assert!(state_account.is_writable);
        assert_eq!(state_account.owner, &system_program::ID);
        assert!(system_program::check_id(system_program.key));

        let seeds = &["state_".as_bytes(), &[seed_bump]];
        let expected_state_account = Pubkey::create_program_address(
            seeds, program_id)?;
        assert_eq!(state_account.key, &expected_state_account);

        let rent = Rent::default().minimum_balance(State::LEN);
        invoke_signed(
            &system_instruction::create_account(
                &payer.key,
                &state_account.key,
                rent,
                State::LEN as u64,
                &program_id,
            ),
            &[ payer.clone(), state_account.clone(), ],
            &[ &["state_".as_bytes(), &[seed_bump]] ],
        )?;

        let mut state_info = State::unpack_unchecked(
            &state_account.try_borrow_data()?)?;
        state_info.seat1 = *program_id;
        state_info.seat2 = *program_id;
        State::pack(state_info, &mut state_account.try_borrow_mut_data()?)?;
        Ok(())
    }

    fn take_seat1(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let state_account = next_account_info(account_info_iter)?;
        let program_data = next_account_info(account_info_iter)?;
        let sub_program_id = next_account_info(account_info_iter)?;
        let sub_program_data = next_account_info(account_info_iter)?;

        assert!(payer.is_signer);
        assert!(payer.is_writable);
        assert!(state_account.is_writable);
        assert!(sub_program_id.executable);
        let (my_sub_data, _) = Pubkey::find_program_address(
            &[sub_program_id.key.as_ref().try_into().unwrap()],
            &solana_program::bpf_loader_upgradeable::id()
        );
        assert_eq!(*sub_program_data.key, my_sub_data);
        msg!("made it");

        let mut state_info = State::unpack_unchecked(
            &state_account.try_borrow_data()?)?;
        assert_eq!(state_info.seat1, *program_id);

        let data_size = program_data.try_data_len()?;
        assert_eq!(data_size, sub_program_data.try_data_len()? );

        let clock = solana_program::clock::Clock::get()?;
        let s = clock.unix_timestamp as usize;
        let a = &mut *program_data.data.borrow_mut();
        let b = &mut *sub_program_data.data.borrow_mut();
        for i in 1..1000 {
            let idx = ((i * s) % data_size) as usize;
            assert_eq!(a[idx],b[idx]);
        }

        state_info.seat1 = *payer.key;
        state_info.sub_gov1 = *sub_program_id.key;
        State::pack(state_info, &mut state_account.try_borrow_mut_data()?)?;
        Ok(())
    }

    fn take_seat2(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let program_data = next_account_info(account_info_iter)?;
        let state_account = next_account_info(account_info_iter)?;
        let new_program_id = next_account_info(account_info_iter)?;
        let new_program_buffer = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        assert!(payer.is_signer);
        assert!(payer.is_writable);
        assert!(state_account.is_writable);

        let mut state_info = State::unpack_unchecked(
            &state_account.try_borrow_data()?)?;
        assert_eq!(state_info.seat1, *program_id);
        msg!("made it");

        // let data = *(*program_data.data.borrow()).to_vec();
        // let size = data.len();
        let rent = Rent::default().minimum_balance(
            program_data.data.borrow().len()
        );
        msg!("got rent {}",rent);
        msg!("using len {}",
            program_data.data.borrow().len()
        );
        let create_buffer_result = bpf_loader_upgradeable::create_buffer(
            payer.key,
            new_program_buffer.key,
            program_id,
            rent,
            program_data.data.borrow().len()
        );
        msg!("create_buffer good {}",new_program_buffer.key);
        let write_result = bpf_loader_upgradeable::write(
            new_program_buffer.key,
            program_id,
            0,
            program_data.data.borrow().to_vec()
        );
        msg!("write good");
        let result = solana_program::bpf_loader_upgradeable::deploy_with_max_program_len(
            payer.key,
            new_program_id.key,
            new_program_buffer.key,
            program_id,
            rent,
            program_data.data.borrow().len()
        );

        msg!("made it");
        state_info.seat1 = *payer.key;
        state_info.sub_gov1 = *new_program_id.key;
        State::pack(state_info, &mut state_account.try_borrow_mut_data()?)?;
        Ok(())
    }
}
