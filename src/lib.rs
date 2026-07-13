use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

pinocchio_pubkey::declare_id!("11111111111111111111111111111111111111111");

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, _rest) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match discriminator {
        0 => {
            let ctx = IncrementAccounts::try_from(&*accounts)?;
            increment_counter(ctx.counter)
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub struct IncrementAccounts<'a> {
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
            return Err(ProgramError::InvalidAccountData);
        }
        if !authority.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(Self { counter, authority })
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CounterState {
    pub count: u64,
}

fn increment_counter(counter: &AccountView) -> ProgramResult {
    let mut data = counter.try_borrow_mut()?;
    let state = bytemuck::from_bytes_mut::<CounterState>(&mut data);
    state.count += 1;
    Ok(())
}
