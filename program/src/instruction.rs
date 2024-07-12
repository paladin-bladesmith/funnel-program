//! Program instruction types.

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};

/// Instructions supported by the Paladin Rewards Funnel program.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaladinFunnelInstruction {
    /// Distribute rewards to the entire Paladin system.
    ///
    /// Facilitates the distribution of rewards according to the following:
    ///
    /// * 10% to the treasury.
    /// * 40% to all token stakers.
    /// * 50% to all token holders.
    ///
    /// This instruction will transfer directly to the treasury account, but
    /// for the rest of the system it will CPI to the Paladin Stake program and
    /// the Paladin Rewards program to distribute staker and holder rewards,
    /// respectively.
    ///
    /// As such, all accounts required for each CPI instruction are required
    /// by this instruction.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w, s]` Payer account.
    /// 1. `[w]` Treasury account.
    /// 2. `[ ]` Paladin Stake program.
    /// 3. `[w]` Stake config account.
    /// 4. `[ ]` Paladin Rewards program.
    /// 5. `[w]` Holder rewards pool account.
    /// 6. `[ ]` Token mint.
    /// 7. `[ ]` System program.
    DistributeRewards { amount: u64 },
}

impl PaladinFunnelInstruction {
    /// Packs a
    /// [PaladinFunnelInstruction](enum.PaladinFunnelInstruction.html)
    /// into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        match self {
            Self::DistributeRewards { amount } => {
                let mut buf = Vec::with_capacity(9);
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf
            }
        }
    }

    /// Unpacks a byte buffer into a
    /// [PaladinFunnelInstruction](enum.PaladinFunnelInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            Some((&0, rest)) if rest.len() == 8 => {
                let amount = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                Ok(Self::DistributeRewards { amount })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Creates a
/// [DistributeRewards](enum.PaladinFunnelInstruction.html)
/// instruction.
#[allow(clippy::too_many_arguments)]
pub fn distribute_rewards(
    payer_address: &Pubkey,
    treasury_address: &Pubkey,
    stake_program_address: &Pubkey,
    stake_config_address: &Pubkey,
    rewards_program_address: &Pubkey,
    holder_rewards_pool_address: &Pubkey,
    token_mint_address: &Pubkey,
    amount: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*payer_address, true),
        AccountMeta::new(*treasury_address, false),
        AccountMeta::new_readonly(*stake_program_address, false),
        AccountMeta::new(*stake_config_address, false),
        AccountMeta::new_readonly(*rewards_program_address, false),
        AccountMeta::new(*holder_rewards_pool_address, false),
        AccountMeta::new_readonly(*token_mint_address, false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let data = PaladinFunnelInstruction::DistributeRewards { amount }.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pack_unpack(instruction: PaladinFunnelInstruction) {
        let packed = instruction.pack();
        let unpacked = PaladinFunnelInstruction::unpack(&packed).unwrap();
        assert_eq!(instruction, unpacked);
    }

    #[test]
    fn test_pack_unpack_distribute_rewards() {
        test_pack_unpack(PaladinFunnelInstruction::DistributeRewards { amount: 42 });
    }
}
