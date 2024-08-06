//! Program processor.

use {
    crate::{error::PaladinFunnelError, instruction::PaladinFunnelInstruction},
    paladin_governance_program::{state::get_treasury_address, ID as GOVERNANCE_PROGRAM_ID},
    paladin_rewards_program_client::ID as REWARDS_PROGRAM_ID,
    paladin_stake_program_client::ID as STAKE_PROGRAM_ID,
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

#[derive(Debug, PartialEq)]
struct RewardDistribution {
    treasury_reward: u64,
    stakers_reward: u64,
    holders_reward: u64,
}

// Calculate the distribution of the amount.
// Throws for an amount > u64::MAX / 2.
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
        .checked_sub(stakers_reward)
        .unwrap()
        .checked_sub(holders_reward)
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
    let stake_program_info = next_account_info(accounts_iter)?;
    let stake_config_info = next_account_info(accounts_iter)?;
    let rewards_program_info = next_account_info(accounts_iter)?;
    let holder_rewards_pool_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let _system_program_info = next_account_info(accounts_iter)?;

    // Ensure the payer account is a signer.
    if !payer_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the proper treasury account was provided.
    if get_treasury_address(&GOVERNANCE_PROGRAM_ID) != *treasury_info.key {
        return Err(PaladinFunnelError::IncorrectTreasuryAddress.into());
    }

    // Ensure the proper stake program was provided.
    if stake_program_info.key != &STAKE_PROGRAM_ID {
        return Err(PaladinFunnelError::IncorrectStakeProgramAddress.into());
    }

    // Ensure the proper rewards program was provided.
    if rewards_program_info.key != &REWARDS_PROGRAM_ID {
        return Err(PaladinFunnelError::IncorrectRewardsProgramAddress.into());
    }

    let RewardDistribution {
        treasury_reward,
        stakers_reward,
        holders_reward,
    } = calculate_distribution(amount)?;

    // Transfer to the treasury.
    invoke(
        &system_instruction::transfer(payer_info.key, treasury_info.key, treasury_reward),
        &[payer_info.clone(), treasury_info.clone()],
    )?;

    // CPI to the Stake program to distribute staker rewards.
    invoke(
        &paladin_stake_program_client::instructions::DistributeRewardsBuilder::new()
            .payer(*payer_info.key)
            .config(*stake_config_info.key)
            .amount(stakers_reward)
            .instruction(),
        &[payer_info.clone(), stake_config_info.clone()],
    )?;

    // CPI to the Rewards program to distribute holder rewards.
    invoke(
        &paladin_rewards_program_client::instructions::DistributeRewardsBuilder::new()
            .payer(*payer_info.key)
            .holder_rewards_pool(*holder_rewards_pool_info.key)
            .mint(*mint_info.key)
            .args(holders_reward)
            .instruction(),
        &[
            payer_info.clone(),
            holder_rewards_pool_info.clone(),
            mint_info.clone(),
        ],
    )?;

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

#[cfg(test)]
mod tests {
    use {super::*, proptest::prelude::*};

    proptest! {
        #[test]
        fn test_calculate_distribution(
            amount in 0..u64::MAX / 2,
        ) {
            // Calculate.
            let result = calculate_distribution(amount).unwrap();
            // Evaluate.

            // The calculation consists of three steps, so evaluate each step
            // one at a time.
            //
            // 1. stakers rewards
            // 2. holders rewards
            // 3. treasury rewards

            // Step 1.
            // Since the amount input is limited to u64::MAX / 2, the
            // multiplication by 2 should never overflow `u64`, and since we're
            // always dividing by 5, the division should never return `None`,
            // so we can unwrap here.
            let stakers_reward = amount.checked_mul(2).and_then(|p| p.checked_div(5)).unwrap();

            // Step 2.
            //
            // Since we're always dividing by 2, the division should never
            // return `None`, so we can unwrap here.
            let holders_reward = amount.checked_div(2).unwrap();

            // Step 3.
            //
            // Since we're always subtracting the stakers and holders rewards
            // from the total amount, the subtraction should never return
            // `None`, so we can unwrap here.
            //
            // This is because, if the above math succeeds, we know that the
            // stakers rewards are 40% of the amount and the holders rewards are
            // 50% of the amount.
            let treasury_reward = amount
                .checked_sub(stakers_reward)
                .and_then(|d| d.checked_sub(holders_reward))
                .unwrap();

            // The function should return the correct distribution.
            prop_assert_eq!(result, RewardDistribution {
                treasury_reward,
                stakers_reward,
                holders_reward,
            });
        }
    }
}
