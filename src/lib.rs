use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Counter {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

entrypoint!(counter_contract);

pub fn counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account = next_account_info(&mut accounts.iter())?;

    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("Incrementing counter...");
            counter_data.count += value;
        }

        InstructionType::Decrement(value) => {
            msg!("Decrementing counter...");
            counter_data.count -= value;
        }
    }

    let _ = counter_data.serialize(&mut *account.data.borrow_mut());

    msg!("Counter successfully update to {}", counter_data.count);
    Ok(())
}
