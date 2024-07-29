//! Paladin Rewards Funnel program.
//!
//! Manages the dispersal of rewards paid into the entire Paladin system.
//!
//! Reward shares are distributed across the governance, staking, and holder
//! ecosystems according to the formula found in the program's processor.

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("Dbf7u6x15DhjMrBMunY3XoRWdByrCCt2dbyoPrCXN6SQ");
