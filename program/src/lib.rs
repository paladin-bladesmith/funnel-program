//! Paladin Funnel program.

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;

solana_program::declare_id!("CF2na53jt22yAQpkw2Uqc1KSNyfiR5p1NzV9XGUa7UbR");
