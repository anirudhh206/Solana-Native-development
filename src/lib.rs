use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // everything happens from here
    Ok(())
}
pub struct IncrementAccounts<'a> {
    //we needs this function to increment the counter as the counter helps to keep track of the number of times the program has been called
    pub counter: &'a AccountView,
    pub authority: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for IncrementAccounts<'a> {
    type Error = ProgramError;
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [counter, authority, _rest @ ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        if counter.owner() != &crate::ID {
            return Err(ProgramError::IncorrectProgramId);
        }
        if !counter.is_writable() {
            return Err(ProgramError::AccountNotWritable);
        }
        if !authority.is_signer() {
            return Err(ProgramError::NotSigner);
        }
        Ok(Self { counter, authority })
    }
}
#[repr(C)]
pub struct CounterState {
    pub count: u64,
}
fn IncrementCounter(counter: &AccountView) -> ProgramResult {
    let mut data = counter.try_borrow_mut_data()?;
    let mut state = bytemuck::from_bytes_mut::<CounterState>(&mut data);
    state.count += 1;
    Ok(())
}
