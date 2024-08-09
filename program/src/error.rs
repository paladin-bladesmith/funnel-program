//! Program error types.

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that can be returned by the Paladin Funnel program.
// Note: Shank does not export the type when we use `spl_program_error`.
#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum PaladinFunnelError {
    /// Incorrect treasury address.
    #[error("Incorrect treasury address.")]
    IncorrectTreasuryAddress,
    /// Incorrect stake program address.
    #[error("Incorrect stake program address.")]
    IncorrectStakeProgramAddress,
    /// Incorrect rewards program address.
    #[error("Incorrect rewards program address.")]
    IncorrectRewardsProgramAddress,
}

impl PrintProgramError for PaladinFunnelError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<PaladinFunnelError> for ProgramError {
    fn from(e: PaladinFunnelError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PaladinFunnelError {
    fn type_of() -> &'static str {
        "PaladinFunnelError"
    }
}
