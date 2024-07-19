//! Program processor.

use {
    crate::instruction::PaladinFunnelInstruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_instruction,
    },
};

#[allow(unused)]
struct RewardDistribution {
    treasury_reward: u64,
    stakers_reward: u64,
    holders_reward: u64,
}

// Calculate the distribution of the amount.
//
// * 10% to the treasury.
// * 40% to all token stakers.
// * 50% to all token holders.
fn calculate_distribution(amount: u64) -> Result<RewardDistribution, ProgramError> {
    let stakers_reward = amount
        .checked_mul(2)
        .and_then(|v| v.checked_div(5))
        .ok_or(ProgramError::InvalidInstructionData)?;
    let holders_reward = amount
        .checked_div(2)
        .ok_or(ProgramError::InvalidInstructionData)?;
    let treasury_reward = amount
        .checked_sub(holders_reward)
        .unwrap()
        .checked_sub(stakers_reward)
        .unwrap();

    Ok(RewardDistribution {
        treasury_reward,
        stakers_reward,
        holders_reward,
    })
}

/// Processes a [DistributeRewards](enum.PaladinFunnelInstruction.html)
/// instruction.
fn process_distribute_rewards(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer_info = next_account_info(accounts_iter)?;
    let treasury_info = next_account_info(accounts_iter)?;
    let _stake_program_info = next_account_info(accounts_iter)?;
    let _stake_config_info = next_account_info(accounts_iter)?;
    let _rewards_program_info = next_account_info(accounts_iter)?;
    let _holder_rewards_pool_info = next_account_info(accounts_iter)?;
    let _mint_info = next_account_info(accounts_iter)?;
    let _system_program_info = next_account_info(accounts_iter)?;

    // Ensure the payer account is a signer.
    if !payer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the proper treasury account was provided.
    // TODO: Use the address derivation helper from the Governance program.

    // Ensure the proper stake program was provided.
    // TODO: This needs to be configured somewhere.

    // Ensure the proper stake config account was provided.
    // TODO: Use the address derivation helper from the Stake program.

    // Ensure the proper rewards program was provided.
    // TODO: This needs to be configured somewhere.

    // Ensure the proper holder rewards account was provided.
    // TODO: Use the address derivation helper from the Rewards program.

    // Ensure the proper mint for the holder rewards account was provided.
    // TODO: Use the address derivation helper from the Rewards program.

    let RewardDistribution {
        treasury_reward,
        stakers_reward: _,
        holders_reward: _,
    } = calculate_distribution(amount)?;

    // Transfer to the treasury.
    invoke(
        &system_instruction::transfer(payer_info.key, treasury_info.key, treasury_reward),
        &[payer_info.clone(), treasury_info.clone()],
    )?;

    // CPI to the Stake program to distribute staker rewards.
    // TODO: CPI...

    // CPI to the Rewards program to distribute holder rewards.
    // TODO: CPI...

    Ok(())
}

/// Processes a
/// [PaladinFunnelInstruction](enum.PaladinFunnelInstruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = PaladinFunnelInstruction::unpack(input)?;
    match instruction {
        PaladinFunnelInstruction::DistributeRewards { amount } => {
            msg!("Instruction: DistributeRewards");
            process_distribute_rewards(program_id, accounts, amount)
        }
    }
}
