use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(counter_contract);

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Counter {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

pub fn counter_contract(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account = next_account_info(&mut accounts.iter())?;
    let mut counter = Counter::try_from_slice(&mut account.data.borrow())?;

    match InstructionType::try_from_slice(&instruction_data)? {
        InstructionType::Increment(value) => {
            msg!("Incrementing counter");
            counter.count += value;
        }
        InstructionType::Decrement(value) => {
            msg!("Decrementing counter");
            counter.count -= value;
        }
    }

    let _ = counter.serialize(&mut *account.data.borrow_mut());

    msg!("Counter updated to {}", counter.count);

    Ok(())
}
