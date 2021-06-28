//! Program instruction processor
//use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Deserialize};
use solana_program::{
    system_instruction,
    clock::Epoch,
    rent::Rent,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke,invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    instruction::{Instruction, AccountMeta},msg,
    sysvar::{clock::{Clock}, Sysvar},
};

/// 
#[derive( Deserialize, Serialize, PartialEq, Debug)]
pub struct Kitty {
    /// kitty dna
    pub dna: Pubkey,
    /// owner pubkey
    pub owner: Pubkey,
    /// price
    pub price: u32,
}
/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Create in iterator to safety reference accounts in the slice
    let account_info_iter = &mut accounts.iter();

    // Account info for the program being invoked
    let be_allocated_program_info = next_account_info(account_info_iter)?;
    msg!("system_program_info {:?}",be_allocated_program_info);
    // Account info to allocate
    let allocated_info = next_account_info(account_info_iter)?;
    msg!("allocated_info {:?}",allocated_info);
    let transfer_one = next_account_info(account_info_iter)?;
    msg!("transfer_one {:?}",transfer_one);
    // Account info to allocate
    let transfer_two = next_account_info(account_info_iter)?;
    msg!("transfer_two {:?}",transfer_two);
    let expected_allocated_key =
        Pubkey::create_program_address(&[b"You pass butter", &[instruction_data[0]]], program_id)?;
    if *allocated_info.key != expected_allocated_key {
        // allocated key does not match the derived address
        return Err(ProgramError::InvalidArgument);
    }
    msg!("1");
    let payer = next_account_info(account_info_iter)?;
    msg!("1");
    let timer = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
    msg!("timer{:?}", timer);
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
    msg!("1");
    msg!("test {:?}", test);
    let test_kitty = Kitty{
        dna : test,
        owner : *payer.key,
        price : 0u32,
    };
    msg!("1");
    msg!("test_kitty {:?}", test_kitty);
    let mut data = bincode::serialize(&test_kitty).unwrap();
    // let data2 : Kitty= bincode::deserialize(&data).unwrap();
    // msg!("data2 {:?}", data2);
    let size = data.len();
    msg!("1");
    //let size = test_kitty.try_to_vec().unwrap().len();
    msg!("size {:?}",size);
    let mut rent = Rent::default().minimum_balance(size).min(10000000);
    let rent2 = rent.clone();
    let test_account = AccountInfo::new(
        &test,
        false,
        false,
        & mut rent,
        & mut data,
        &payer.key,
        false,
        Epoch::default(),
    );
    //let ix = Instruction::new_with_bincode(test, &[2], vec![]);
    // let te_is = solana_program::system_instruction::create_account(
    //     &payer.key,
    //     &test_account.key,
    //     rent2,
    //     size as u64,
    //     &payer.key,
    // );
    msg!("test_account{:?}",test_account);
    msg!("abc {:?}", abc);
    invoke(
        &system_instruction::create_account(
            &payer.key,
            &abc.key,
            rent2,
            size as u64,
            &payer.key,
        ),
        &[
            abc.clone(),
            system.clone(),
            payer.clone(),
        ],
    )?;
    msg!("2");
    // let test_ix = solana_program::system_instruction::create_account_with_seed(
    //     &payer.key,
    //     &test,
    //     &test_pubkey,
    //     &timer.unix_timestamp.to_string(),
    //     rent,
    //     size as u64,
    //     &payer.key,
    // );
    // invoke(
    //     &test_ix,
    //     &[
    //         test_account.clone(),
    //         payer.clone(),
    //     ],
    // );
    let instruction = Instruction::new_with_bincode(
        *be_allocated_program_info.key,
        &[0],
        vec![AccountMeta::new(*transfer_one.key, false), 
        AccountMeta::new(*transfer_two.key, false)],
        
    );
    msg!("1");
    // Invoke the system program to allocate account data
    invoke_signed(
        &instruction,
        // Order doesn't matter and this slice could include all the accounts and be:
        // `&accounts`
        &[
            be_allocated_program_info.clone(), // program being invoked also needs to be included
            allocated_info.clone(),
            transfer_one.clone(),
            transfer_two.clone(),
        ],
        &[&[b"You pass butter", &[instruction_data[0]]]],
    )?;
    Ok(())
}
