// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum GenerativeNftError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<GenerativeNftError> for ProgramError {
    fn from(e: GenerativeNftError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for GenerativeNftError {
    fn type_of() -> &'static str {
        "GenerativeNftError"
    }
}

impl PrintProgramError for GenerativeNftError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            GenerativeNftError::InvalidInstruction => msg!("Error: Invalid instruction"),
            GenerativeNftError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            GenerativeNftError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            GenerativeNftError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            GenerativeNftError::InvalidAccountLen => msg!("Error: Invalid account length"),
            GenerativeNftError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}