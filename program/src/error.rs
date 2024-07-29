//! Program error types.

use spl_program_error::*;

/// Errors that can be returned by the Paladin Funnel program.
#[spl_program_error]
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
