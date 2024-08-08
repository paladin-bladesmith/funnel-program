#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_funnel_program::{error::PaladinFunnelError, instruction::distribute_rewards},
    paladin_governance_program_client::{pdas::find_treasury_pda, ID as GOVERNANCE_PROGRAM_ID},
    paladin_rewards_program_client::{accounts::HolderRewardsPool, ID as REWARDS_PROGRAM_ID},
    paladin_stake_program_client::ID as STAKE_PROGRAM_ID,
    setup::{setup, setup_holder_rewards_pool_account, setup_mint, setup_stake_config_account},
    solana_program_test::*,
    solana_sdk::{
        account::AccountSharedData,
        instruction::InstructionError,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_program,
        transaction::{Transaction, TransactionError},
    },
    test_case::test_case,
};

#[tokio::test]
async fn fail_payer_not_signer() {
    let payer = Pubkey::new_unique();
    let stake_config_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();

    let treasury_address = find_treasury_pda(&stake_config_address).0;
    let holder_rewards_pool_address = HolderRewardsPool::find_pda(&token_mint_address).0;

    let amount = 1_000_000;

    let mut context = setup().start_with_context().await;

    let mut instruction = distribute_rewards(
        &payer,
        &treasury_address,
        &STAKE_PROGRAM_ID,
        &stake_config_address,
        &REWARDS_PROGRAM_ID,
        &holder_rewards_pool_address,
        &token_mint_address,
        amount,
    );
    instruction.accounts[0].is_signer = false; // Not signer.

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer], // Missing payer.
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(0, InstructionError::MissingRequiredSignature)
    );
}

#[tokio::test]
async fn fail_incorrect_treasury_address() {
    let stake_config_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();

    let treasury_address = Pubkey::new_unique(); // Incorrect treasury address.
    let holder_rewards_pool_address = HolderRewardsPool::find_pda(&token_mint_address).0;

    let amount = 1_000_000;

    let mut context = setup().start_with_context().await;

    let instruction = distribute_rewards(
        &context.payer.pubkey(),
        &treasury_address,
        &STAKE_PROGRAM_ID,
        &stake_config_address,
        &REWARDS_PROGRAM_ID,
        &holder_rewards_pool_address,
        &token_mint_address,
        amount,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(PaladinFunnelError::IncorrectTreasuryAddress as u32)
        )
    );
}

#[tokio::test]
async fn fail_incorrect_stake_program_id() {
    let stake_config_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();

    let treasury_address = find_treasury_pda(&stake_config_address).0;
    let holder_rewards_pool_address = HolderRewardsPool::find_pda(&token_mint_address).0;

    let amount = 1_000_000;

    let mut context = setup().start_with_context().await;

    let instruction = distribute_rewards(
        &context.payer.pubkey(),
        &treasury_address,
        &Pubkey::new_unique(), // Incorrect stake program ID.
        &stake_config_address,
        &REWARDS_PROGRAM_ID,
        &holder_rewards_pool_address,
        &token_mint_address,
        amount,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(PaladinFunnelError::IncorrectStakeProgramAddress as u32)
        )
    );
}

#[tokio::test]
async fn fail_incorrect_rewards_program_id() {
    let stake_config_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();

    let treasury_address = find_treasury_pda(&stake_config_address).0;
    let holder_rewards_pool_address = HolderRewardsPool::find_pda(&token_mint_address).0;

    let amount = 1_000_000;

    let mut context = setup().start_with_context().await;

    let instruction = distribute_rewards(
        &context.payer.pubkey(),
        &treasury_address,
        &STAKE_PROGRAM_ID,
        &stake_config_address,
        &Pubkey::new_unique(), // Incorrect rewards program ID.
        &holder_rewards_pool_address,
        &token_mint_address,
        amount,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(PaladinFunnelError::IncorrectRewardsProgramAddress as u32)
        )
    );
}

async fn get_balance(context: &mut ProgramTestContext, address: &Pubkey) -> u64 {
    context
        .banks_client
        .get_account(*address)
        .await
        .unwrap()
        .unwrap()
        .lamports
}

#[test_case(0)]
#[test_case(1_000)]
#[test_case(100_000)]
#[test_case(100_000_000)]
#[tokio::test]
async fn success(amount: u64) {
    // Use a separate payer so its easier to track lamports.
    let payer = Keypair::new();
    let stake_config_address = Pubkey::new_unique();
    let token_mint_address = Pubkey::new_unique();

    let treasury_address = find_treasury_pda(&stake_config_address).0;
    let holder_rewards_pool_address = HolderRewardsPool::find_pda(&token_mint_address).0;

    let mut context = setup().start_with_context().await;
    context.set_account(
        &payer.pubkey(),
        &AccountSharedData::new(1_000_000_000 + amount, 0, &system_program::id()),
    );
    context.set_account(
        &treasury_address,
        &AccountSharedData::new(1_000_000, 0, &GOVERNANCE_PROGRAM_ID),
    );
    setup_stake_config_account(&mut context, &stake_config_address).await;
    setup_holder_rewards_pool_account(
        &mut context,
        &holder_rewards_pool_address,
        /* excess_lamports */ 0,
        /* accumulated_rewards_per_token */ 0,
    )
    .await;
    setup_mint(
        &mut context,
        &token_mint_address,
        &Pubkey::new_unique(),
        1_000_000_000,
    )
    .await;

    // For checks later.
    let payer_beginning_lamports = get_balance(&mut context, &payer.pubkey()).await;
    let treasury_beginning_lamports = get_balance(&mut context, &treasury_address).await;
    let stake_config_beginning_lamports = get_balance(&mut context, &stake_config_address).await;
    let holder_rewards_pool_beginning_lamports =
        get_balance(&mut context, &holder_rewards_pool_address).await;

    let instruction = distribute_rewards(
        &payer.pubkey(),
        &treasury_address,
        &STAKE_PROGRAM_ID,
        &stake_config_address,
        &REWARDS_PROGRAM_ID,
        &holder_rewards_pool_address,
        &token_mint_address,
        amount,
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let payer_end_lamports = get_balance(&mut context, &payer.pubkey()).await;
    let treasury_end_lamports = get_balance(&mut context, &treasury_address).await;
    let stake_config_end_lamports = get_balance(&mut context, &stake_config_address).await;
    let holder_rewards_pool_end_lamports =
        get_balance(&mut context, &holder_rewards_pool_address).await;

    // Assert the payer was debited the full amount.
    assert_eq!(payer_end_lamports, payer_beginning_lamports - amount);

    // Assert the treasury was credited 10% of the amount.
    assert_eq!(
        treasury_end_lamports,
        treasury_beginning_lamports + (amount / 10)
    );

    // Assert the stake config account was credited 40% of the amount.
    assert_eq!(
        stake_config_end_lamports,
        stake_config_beginning_lamports + (amount * 2 / 5)
    );

    // Assert the holder rewards pool was credited 50% of the amount.
    assert_eq!(
        holder_rewards_pool_end_lamports,
        holder_rewards_pool_beginning_lamports + (amount / 2)
    );
}
