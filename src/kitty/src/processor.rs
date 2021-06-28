use solana_program::{
    system_instruction,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar,clock::{Clock}},
    config::program::id,
};
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use spl_token::state::Account as TokenAccount;

use crate::{error::EscrowError, instruction::EscrowInstruction, state::{Escrow,Kitty,OwnKitty}};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
            EscrowInstruction::Exchange { amount } => {
                msg!("Instruction: Exchange");
                Self::process_exchange(accounts, amount, program_id)
            }
            EscrowInstruction::Create => {
                msg!("Instruction: Create");
                Self::create(program_id,  accounts, instruction_data)
            }
            EscrowInstruction::Kittydeserialize => {
                msg!("Instruction: Create");
                Self::kittydeserialize(program_id,  accounts, instruction_data)
            }

        }
    }

    fn kittydeserialize(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer = next_account_info(account_info_iter)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let system = next_account_info(account_info_iter)?;
        let new_key = next_account_info(account_info_iter)?;
        let new_kitty_dna = Pubkey::create_with_seed(
            payer.key,
            &timer.unix_timestamp.to_string(),
            payer.key,
        )?;  
        let new_kitty = Kitty{
            dna : new_kitty_dna,
            owner : *payer.key,
            price : 0u32,
            parents : None,
        };
        msg!("2");
        kitty_vec.push(new_kitty);msg!("1");
        // let data : OwnKitty= BorshDeserialize::deserialize(&mut &abc.data.borrow_mut()[..]).unwrap_or(0);
        match data : OwnKitty= BorshDeserialize::deserialize(&mut &abc.data.borrow_mut()[..]){
            Ok(owner_data) => {
                *owner_data.num = owner_data.num + 1;

            },
            Err => {

            }
        }
        let mut size = 1;
        match data{
            _ => {
                size = data.num + size;
            },
            0 =>{ }   
        }
        msg!("size {:?}",size);
       
        
        let mut kitty_vec : Vec<Kitty> =  Vec::new();
       
        let KittyPush = OwnKitty{
            num : size,
            own : kitty_vec,
        };
        msg!("Kittytest {:?}",KittyPush);
        let mut data = bincode::serialize(&KittyPush).unwrap();
        let size = data.len();
        msg!("size {:?}",size);
        let mut rent = Rent::default().minimum_balance(size).min(10000000);
        // let mut datase = bincode::serialize(&Kittytest).unwrap();
        // cc.serialize(&mut &mut abc.data.borrow_mut()[..])?;
        msg!("abc {:?}", abc);
        if abc.owner != program_id{
            msg!("1");
            invoke(
                &system_instruction::create_account(
                    &payer.key,
                    &abc.key,
                    rent,
                    //size as u64,
                    10240,
                    &program_id,
                ),
                
                &[
                    abc.clone(),
                    system.clone(),
                    payer.clone(),
                ],
            )?;
        }
        msg!("abc {:?}", abc);
        Kittytest.serialize(&mut &mut abc.data.borrow_mut()[..])?;
        let bb : OwnKitty= BorshDeserialize::deserialize(&mut &abc.data.borrow_mut()[..]).unwrap();
        msg!("bb {:?}", bb);
        let mut cc = OwnKitty::try_from_slice(&abc.data.borrow());
        msg!("1{:?}",cc);
        let mut  abb =cc?;
        abb = Kittytest;
        msg!("2");
        abb.serialize(&mut &mut abc.data.borrow_mut()[..])?;msg!("1");

        msg!("cc{:?}", abb);
        msg!("abc {:?}", abc);
        msg!("2");
        Ok(())
    }


    fn create(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
                // Create in iterator to safety reference accounts in the slice
                msg!("1");
        let account_info_iter = &mut accounts.iter();
        let transfer_one = next_account_info(account_info_iter)?;
        // Account info to allocate
        let transfer_two = next_account_info(account_info_iter)?;
        let payer = next_account_info(account_info_iter)?;
        let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let system = next_account_info(account_info_iter)?;
        let abc = next_account_info(account_info_iter)?;
        let mut buffer = [ 0u8; 32];
        let transfer_one_key = transfer_one.key.to_bytes();
        let transfer_two_key = transfer_two.key.to_bytes();
        for i in 0..16{
            buffer[i] = transfer_one_key[i];
        };
        for i in 16..32{
            buffer[i] = transfer_two_key[i];
        };
        let test_pubkey = Pubkey::new_from_array(buffer);
        
        let test = Pubkey::create_with_seed(
            &test_pubkey,
            &timer.unix_timestamp.to_string(),
            payer.key,
        )?;  
        // let ix = Instruction::new_with_bincode(key, &[2], vec![]);
        let test_kitty = Kitty{
            dna : test,
            owner : *payer.key,
            price : 0u32,
            parents : Some([*transfer_one.key,*transfer_two.key]),
        };
        // let mut data = bincode::serialize(&test_kitty).unwrap();
        // let size = data.len();
        //let size = test_kitty.try_to_vec().unwrap().len();
        msg!("1");
   

        // invoke(
        //     &system_instruction::assign(
        //         &abc.key,
        //         &program_id,
        //     ),
        //     &[
        //         abc.clone(),
        //         // system.clone(),
        //         // payer.clone(),
        //     ],
        // )?;
        let mut datavec : Vec<Kitty> =  Vec::new();
        msg!("2");
        datavec.push(test_kitty);msg!("1");
       // datavec.push(test_kitty);msg!("1");
        let Kittytest = OwnKitty{
            num : 1,
            own : datavec,
        };
        msg!("Kittytest {:?}",Kittytest);
        let mut data = bincode::serialize(&Kittytest).unwrap();
        let size = data.len();
        msg!("size {:?}",size);
        let mut rent = Rent::default().minimum_balance(size).min(10000000);
        // let mut datase = bincode::serialize(&Kittytest).unwrap();
        // cc.serialize(&mut &mut abc.data.borrow_mut()[..])?;
        msg!("abc {:?}", abc);
        if abc.owner != program_id{
            msg!("1");
            invoke(
                &system_instruction::create_account(
                    &payer.key,
                    &abc.key,
                    rent,
                    //size as u64,
                    10240,
                    &program_id,
                ),
                
                &[
                    abc.clone(),
                    system.clone(),
                    payer.clone(),
                ],
            )?;
        }
        msg!("abc {:?}", abc);
        Kittytest.serialize(&mut &mut abc.data.borrow_mut()[..])?;
        let bb : OwnKitty= BorshDeserialize::deserialize(&mut &abc.data.borrow_mut()[..]).unwrap();
        msg!("bb {:?}", bb);
        let mut cc = OwnKitty::try_from_slice(&abc.data.borrow());
        msg!("1{:?}",cc);
        let mut  abb =cc?;
        abb = Kittytest;
        msg!("2");
        abb.serialize(&mut &mut abc.data.borrow_mut()[..])?;msg!("1");

        msg!("cc{:?}", abb);
       // msg!("datase {:?}",datase.len());
       // let datase2 :OwnKitty = bincode::deserialize(&datase).unwrap();
       // msg!("datase2 {:?}",datase2);
      
        // let bb : OwnKitty= bincode::deserialize(&abc.data.borrow_mut()[..]).unwrap();
        // msg!("data111111{:?}",bb );
        msg!("abc {:?}", abc);
        msg!("2");
        Ok(())

    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;

        Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    fn process_exchange(
        accounts: &[AccountInfo],
        amount_expected_by_taker: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let taker = next_account_info(account_info_iter)?;

        if !taker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let takers_sending_token_account = next_account_info(account_info_iter)?;

        let takers_token_to_receive_account = next_account_info(account_info_iter)?;

        let pdas_temp_token_account = next_account_info(account_info_iter)?;
        let pdas_temp_token_account_info =
            TokenAccount::unpack(&pdas_temp_token_account.data.borrow())?;
        let (pda, nonce) = Pubkey::find_program_address(&[b"escrow"], program_id);

        if amount_expected_by_taker != pdas_temp_token_account_info.amount {
            return Err(EscrowError::ExpectedAmountMismatch.into());
        }

        let initializers_main_account = next_account_info(account_info_iter)?;
        let initializers_token_to_receive_account = next_account_info(account_info_iter)?;
        let escrow_account = next_account_info(account_info_iter)?;

        let escrow_info = Escrow::unpack(&escrow_account.data.borrow())?;

        if escrow_info.temp_token_account_pubkey != *pdas_temp_token_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow_info.initializer_pubkey != *initializers_main_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        if escrow_info.initializer_token_to_receive_account_pubkey
            != *initializers_token_to_receive_account.key
        {
            return Err(ProgramError::InvalidAccountData);
        }

        let token_program = next_account_info(account_info_iter)?;

        let transfer_to_initializer_ix = spl_token::instruction::transfer(
            token_program.key,
            takers_sending_token_account.key,
            initializers_token_to_receive_account.key,
            taker.key,
            &[&taker.key],
            escrow_info.expected_amount,
        )?;
        msg!("Calling the token program to transfer tokens to the escrow's initializer...");
        invoke(
            &transfer_to_initializer_ix,
            &[
                takers_sending_token_account.clone(),
                initializers_token_to_receive_account.clone(),
                taker.clone(),
                token_program.clone(),
            ],
        )?;

        let pda_account = next_account_info(account_info_iter)?;

        let transfer_to_taker_ix = spl_token::instruction::transfer(
            token_program.key,
            pdas_temp_token_account.key,
            takers_token_to_receive_account.key,
            &pda,
            &[&pda],
            pdas_temp_token_account_info.amount,
        )?;
        msg!("Calling the token program to transfer tokens to the taker...");
        invoke_signed(
            &transfer_to_taker_ix,
            &[
                pdas_temp_token_account.clone(),
                takers_token_to_receive_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        let close_pdas_temp_acc_ix = spl_token::instruction::close_account(
            token_program.key,
            pdas_temp_token_account.key,
            initializers_main_account.key,
            &pda,
            &[&pda],
        )?;
        msg!("Calling the token program to close pda's temp account...");
        invoke_signed(
            &close_pdas_temp_acc_ix,
            &[
                pdas_temp_token_account.clone(),
                initializers_main_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"escrow"[..], &[nonce]]],
        )?;

        msg!("Closing the escrow account...");
        **initializers_main_account.lamports.borrow_mut() = initializers_main_account
            .lamports()
            .checked_add(escrow_account.lamports())
            .ok_or(EscrowError::AmountOverflow)?;
        **escrow_account.lamports.borrow_mut() = 0;
        *escrow_account.data.borrow_mut() = &mut [];

        Ok(())
    }
}
