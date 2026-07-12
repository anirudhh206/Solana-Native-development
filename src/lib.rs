#![no_std]

use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
pub struct IncrementAccounts<'a> {
    //we needs this function to increment the counter as the counter helps to keep track of the number of times the program has been called
    pub counter: &'a AccountView,
    pub authority: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for IncrementAccounts<'a> {
    type Error = ProgramError;
    fn tryfrom(account: &'a [AAccountView]) -> Result<Self, Self::Error> {
        let [counter, authority, _rest @ ..] = account else {
            return Err(ProgramResult::NotEnoughAccountkeys);
        };
    }
}
