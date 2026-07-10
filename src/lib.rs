#![no_std] //it tells rust not to link the standard library, which is not available in embedded systems

use pinocchio::{
    account_info::AccountInfo,
    entrypoint,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};

pub struct DepositAccounts<'a> {

}   