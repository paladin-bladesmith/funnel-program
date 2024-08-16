//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct DistributeRewards {
    /// Payer account
    pub payer: solana_program::pubkey::Pubkey,
    /// Paladin Stake program
    pub paladin_stake_program: solana_program::pubkey::Pubkey,
    /// Stake config account
    pub stake_config: solana_program::pubkey::Pubkey,
    /// Paladin Rewards program
    pub paladin_rewards_program: solana_program::pubkey::Pubkey,
    /// Holder rewards pool account
    pub holder_rewards_pool: solana_program::pubkey::Pubkey,
    /// Token mint
    pub token_mint: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
}

impl DistributeRewards {
    pub fn instruction(
        &self,
        args: DistributeRewardsInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: DistributeRewardsInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.paladin_stake_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.stake_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.paladin_rewards_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.holder_rewards_pool,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = DistributeRewardsInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::PALADIN_FUNNEL_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct DistributeRewardsInstructionData {
    discriminator: u8,
}

impl DistributeRewardsInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 0 }
    }
}

impl Default for DistributeRewardsInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DistributeRewardsInstructionArgs {
    pub amount: u64,
}

/// Instruction builder for `DistributeRewards`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[]` paladin_stake_program
///   2. `[writable]` stake_config
///   3. `[]` paladin_rewards_program
///   4. `[writable]` holder_rewards_pool
///   5. `[]` token_mint
///   6. `[optional]` system_program (default to
///      `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct DistributeRewardsBuilder {
    payer: Option<solana_program::pubkey::Pubkey>,
    paladin_stake_program: Option<solana_program::pubkey::Pubkey>,
    stake_config: Option<solana_program::pubkey::Pubkey>,
    paladin_rewards_program: Option<solana_program::pubkey::Pubkey>,
    holder_rewards_pool: Option<solana_program::pubkey::Pubkey>,
    token_mint: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl DistributeRewardsBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Payer account
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// Paladin Stake program
    #[inline(always)]
    pub fn paladin_stake_program(
        &mut self,
        paladin_stake_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.paladin_stake_program = Some(paladin_stake_program);
        self
    }
    /// Stake config account
    #[inline(always)]
    pub fn stake_config(&mut self, stake_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.stake_config = Some(stake_config);
        self
    }
    /// Paladin Rewards program
    #[inline(always)]
    pub fn paladin_rewards_program(
        &mut self,
        paladin_rewards_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.paladin_rewards_program = Some(paladin_rewards_program);
        self
    }
    /// Holder rewards pool account
    #[inline(always)]
    pub fn holder_rewards_pool(
        &mut self,
        holder_rewards_pool: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.holder_rewards_pool = Some(holder_rewards_pool);
        self
    }
    /// Token mint
    #[inline(always)]
    pub fn token_mint(&mut self, token_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_mint = Some(token_mint);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = DistributeRewards {
            payer: self.payer.expect("payer is not set"),
            paladin_stake_program: self
                .paladin_stake_program
                .expect("paladin_stake_program is not set"),
            stake_config: self.stake_config.expect("stake_config is not set"),
            paladin_rewards_program: self
                .paladin_rewards_program
                .expect("paladin_rewards_program is not set"),
            holder_rewards_pool: self
                .holder_rewards_pool
                .expect("holder_rewards_pool is not set"),
            token_mint: self.token_mint.expect("token_mint is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = DistributeRewardsInstructionArgs {
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `distribute_rewards` CPI accounts.
pub struct DistributeRewardsCpiAccounts<'a, 'b> {
    /// Payer account
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Paladin Stake program
    pub paladin_stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake config account
    pub stake_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Paladin Rewards program
    pub paladin_rewards_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Holder rewards pool account
    pub holder_rewards_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// Token mint
    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `distribute_rewards` CPI instruction.
pub struct DistributeRewardsCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer account
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Paladin Stake program
    pub paladin_stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Stake config account
    pub stake_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Paladin Rewards program
    pub paladin_rewards_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Holder rewards pool account
    pub holder_rewards_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// Token mint
    pub token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: DistributeRewardsInstructionArgs,
}

impl<'a, 'b> DistributeRewardsCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: DistributeRewardsCpiAccounts<'a, 'b>,
        args: DistributeRewardsInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            payer: accounts.payer,
            paladin_stake_program: accounts.paladin_stake_program,
            stake_config: accounts.stake_config,
            paladin_rewards_program: accounts.paladin_rewards_program,
            holder_rewards_pool: accounts.holder_rewards_pool,
            token_mint: accounts.token_mint,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.paladin_stake_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.stake_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.paladin_rewards_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.holder_rewards_pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = DistributeRewardsInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::PALADIN_FUNNEL_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(7 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.paladin_stake_program.clone());
        account_infos.push(self.stake_config.clone());
        account_infos.push(self.paladin_rewards_program.clone());
        account_infos.push(self.holder_rewards_pool.clone());
        account_infos.push(self.token_mint.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `DistributeRewards` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` payer
///   1. `[]` paladin_stake_program
///   2. `[writable]` stake_config
///   3. `[]` paladin_rewards_program
///   4. `[writable]` holder_rewards_pool
///   5. `[]` token_mint
///   6. `[]` system_program
#[derive(Clone, Debug)]
pub struct DistributeRewardsCpiBuilder<'a, 'b> {
    instruction: Box<DistributeRewardsCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> DistributeRewardsCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(DistributeRewardsCpiBuilderInstruction {
            __program: program,
            payer: None,
            paladin_stake_program: None,
            stake_config: None,
            paladin_rewards_program: None,
            holder_rewards_pool: None,
            token_mint: None,
            system_program: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Payer account
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// Paladin Stake program
    #[inline(always)]
    pub fn paladin_stake_program(
        &mut self,
        paladin_stake_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.paladin_stake_program = Some(paladin_stake_program);
        self
    }
    /// Stake config account
    #[inline(always)]
    pub fn stake_config(
        &mut self,
        stake_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.stake_config = Some(stake_config);
        self
    }
    /// Paladin Rewards program
    #[inline(always)]
    pub fn paladin_rewards_program(
        &mut self,
        paladin_rewards_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.paladin_rewards_program = Some(paladin_rewards_program);
        self
    }
    /// Holder rewards pool account
    #[inline(always)]
    pub fn holder_rewards_pool(
        &mut self,
        holder_rewards_pool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.holder_rewards_pool = Some(holder_rewards_pool);
        self
    }
    /// Token mint
    #[inline(always)]
    pub fn token_mint(
        &mut self,
        token_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_mint = Some(token_mint);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool`
    /// indicating whether the account is writable or not, and a `bool`
    /// indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = DistributeRewardsInstructionArgs {
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = DistributeRewardsCpi {
            __program: self.instruction.__program,

            payer: self.instruction.payer.expect("payer is not set"),

            paladin_stake_program: self
                .instruction
                .paladin_stake_program
                .expect("paladin_stake_program is not set"),

            stake_config: self
                .instruction
                .stake_config
                .expect("stake_config is not set"),

            paladin_rewards_program: self
                .instruction
                .paladin_rewards_program
                .expect("paladin_rewards_program is not set"),

            holder_rewards_pool: self
                .instruction
                .holder_rewards_pool
                .expect("holder_rewards_pool is not set"),

            token_mint: self.instruction.token_mint.expect("token_mint is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct DistributeRewardsCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    paladin_stake_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    stake_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    paladin_rewards_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    holder_rewards_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
