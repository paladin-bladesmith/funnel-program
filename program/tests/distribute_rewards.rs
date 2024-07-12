#![cfg(feature = "test-sbf")]

use {
    paladin_funnel_program::instruction::distribute_rewards,
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

fn setup() -> ProgramTest {
    ProgramTest::new(
        "paladin_funnel_program",
        paladin_funnel_program::id(),
        processor!(paladin_funnel_program::processor::process),
    )
}

#[tokio::test]
async fn fail_payer_not_signer() {
    let payer = Keypair::new();
    let treasury_address = Pubkey::new_unique(); // TODO!
    let stake_program_address = Pubkey::new_unique(); // TODO!
    let stake_config_address = Pubkey::new_unique(); // TODO!
    let rewards_program_address = Pubkey::new_unique(); // TODO!
    let holder_rewards_pool_address = Pubkey::new_unique(); // TODO!
    let token_mint_address = Pubkey::new_unique();

    let amount = 1_000_000;

    let mut context = setup().start_with_context().await;

    let mut instruction = distribute_rewards(
        &payer.pubkey(),
        &treasury_address,
        &stake_program_address,
        &stake_config_address,
        &rewards_program_address,
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

#[cfg(skip)]
#[tokio::test]
fn fail_incorrect_treasury_address() {}

#[cfg(skip)]
#[tokio::test]
fn fail_incorrect_stake_program_id() {}

#[cfg(skip)]
#[tokio::test]
fn fail_incorrect_stake_config_address() {}

#[cfg(skip)]
#[tokio::test]
fn fail_incorrect_rewards_program_id() {}

#[cfg(skip)]
#[tokio::test]
fn fail_incorrect_holder_rewards_pool_address() {}

#[cfg(skip)]
#[tokio::test]
fn fail_incorrect_mint_address() {}

#[test_case(0)]
#[test_case(1_000)]
#[test_case(100_000)]
#[test_case(100_000_000)]
#[tokio::test]
async fn success(amount: u64) {
    let payer = Keypair::new();
    let treasury_address = Pubkey::new_unique(); // TODO!
    let stake_program_address = Pubkey::new_unique(); // TODO!
    let stake_config_address = Pubkey::new_unique(); // TODO!
    let rewards_program_address = Pubkey::new_unique(); // TODO!
    let holder_rewards_pool_address = Pubkey::new_unique(); // TODO!
    let token_mint_address = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    context.set_account(
        &payer.pubkey(),
        &AccountSharedData::new(1_000_000 + amount, 0, &system_program::id()),
    );
    context.set_account(
        &treasury_address,
        &AccountSharedData::new(1_000_000, 0, &Pubkey::new_unique()), // TODO!
    );

    // For checks later.
    let payer_beginning_lamports = context
        .banks_client
        .get_account(payer.pubkey())
        .await
        .unwrap()
        .unwrap()
        .lamports;
    let treasury_beginning_lamports = context
        .banks_client
        .get_account(treasury_address)
        .await
        .unwrap()
        .unwrap()
        .lamports;

    let instruction = distribute_rewards(
        &payer.pubkey(),
        &treasury_address,
        &stake_program_address,
        &stake_config_address,
        &rewards_program_address,
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

    // Assert the payer was debited.
    let payer_end_lamports = context
        .banks_client
        .get_account(payer.pubkey())
        .await
        .unwrap()
        .unwrap()
        .lamports;
    assert_eq!(payer_end_lamports, payer_beginning_lamports - (amount / 10)); // TODO!

    // Assert the treasury was credited.
    let treasury_end_lamports = context
        .banks_client
        .get_account(treasury_address)
        .await
        .unwrap()
        .unwrap()
        .lamports;
    assert_eq!(
        treasury_end_lamports,
        treasury_beginning_lamports + (amount / 10)
    ); // TODO!
}
