//! Program error types.

use spl_program_error::*;

/// Errors that can be returned by the Paladin Funnel program.
#[spl_program_error]
pub enum PaladinFunnelError {
    /// This is a placeholder error.
    #[error("This is a placeholder error.")]
    Placeholder,
}
