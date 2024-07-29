#![cfg(feature = "test-sbf")]
#![allow(dead_code)]

use {
    borsh::BorshSerialize,
    paladin_rewards_program_client::{accounts::HolderRewardsPool, ID as REWARDS_PROGRAM_ID},
    paladin_stake_program_client::{accounts::Config as StakeConfig, ID as STAKE_PROGRAM_ID},
    solana_program_test::*,
    solana_sdk::{
        account::{Account, AccountSharedData},
        program_option::COption,
        pubkey::Pubkey,
    },
    spl_token_2022::{
        extension::{
            transfer_hook::TransferHook, BaseStateWithExtensionsMut, ExtensionType,
            StateWithExtensionsMut,
        },
        state::Mint,
    },
};

pub fn setup() -> ProgramTest {
    let mut pt = ProgramTest::new(
        "paladin_funnel_program",
        paladin_funnel_program::id(),
        processor!(paladin_funnel_program::processor::process),
    );
    pt.add_program("paladin_rewards_program", REWARDS_PROGRAM_ID, None);
    pt.add_program("paladin_stake_program", STAKE_PROGRAM_ID, None);
    pt
}

pub async fn setup_mint(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    mint_authority: &Pubkey,
    supply: u64,
) {
    let account_size =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::TransferHook]).unwrap();

    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(account_size);

    let mut data = vec![0; account_size];
    {
        let mut state = StateWithExtensionsMut::<Mint>::unpack_uninitialized(&mut data).unwrap();
        state
            .init_extension::<TransferHook>(true)
            .unwrap()
            .program_id = Some(REWARDS_PROGRAM_ID).try_into().unwrap();
        state.base = Mint {
            mint_authority: COption::Some(*mint_authority),
            is_initialized: true,
            supply,
            ..Mint::default()
        };
        state.pack_base();
        state.init_account_type().unwrap();
    }

    context.set_account(
        mint,
        &AccountSharedData::from(Account {
            lamports,
            data,
            owner: spl_token_2022::id(),
            ..Account::default()
        }),
    );
}

pub async fn setup_stake_config_account(
    context: &mut ProgramTestContext,
    stake_config_address: &Pubkey,
) {
    // Have to do this manually until the Stake program client is ready.
    let preimage = solana_program::hash::hash(b"stake::state::config");
    let discriminator: [u8; 8] = preimage.to_bytes()[..8].try_into().unwrap();
    let state = StakeConfig {
        discriminator,
        authority: Pubkey::new_unique().into(),
        slash_authority: Pubkey::new_unique().into(),
        vault: Pubkey::new_unique(),
        cooldown_time_seconds: 0,
        token_amount_delegated: 0,
        accumulated_stake_rewards_per_token: 0,
        max_deactivation_basis_points: 0,
        vault_authority_bump: 0,
        padding: [0; 5],
    };
    let data = BorshSerialize::try_to_vec(&state).unwrap();

    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(data.len());

    context.set_account(
        stake_config_address,
        &AccountSharedData::from(Account {
            lamports,
            data,
            owner: STAKE_PROGRAM_ID,
            ..Account::default()
        }),
    );
}

#[allow(clippy::arithmetic_side_effects)]
pub async fn setup_holder_rewards_pool_account(
    context: &mut ProgramTestContext,
    holder_rewards_pool_address: &Pubkey,
    excess_lamports: u64,
    accumulated_rewards_per_token: u128,
) {
    let state = HolderRewardsPool {
        accumulated_rewards_per_token,
    };
    let data = BorshSerialize::try_to_vec(&state).unwrap();

    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(data.len()) + excess_lamports;

    context.set_account(
        holder_rewards_pool_address,
        &AccountSharedData::from(Account {
            lamports,
            data,
            owner: REWARDS_PROGRAM_ID,
            ..Account::default()
        }),
    );
}
