/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlyAccount,
  type TransactionSigner,
  type WritableAccount,
  type WritableSignerAccount,
} from '@solana/web3.js';
import { PALADIN_FUNNEL_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export type DistributeRewardsInstruction<
  TProgram extends string = typeof PALADIN_FUNNEL_PROGRAM_ADDRESS,
  TAccountPayer extends string | IAccountMeta<string> = string,
  TAccountPaladinStakeProgram extends string | IAccountMeta<string> = string,
  TAccountStakeConfig extends string | IAccountMeta<string> = string,
  TAccountPaladinRewardsProgram extends string | IAccountMeta<string> = string,
  TAccountHolderRewardsPool extends string | IAccountMeta<string> = string,
  TAccountTokenMint extends string | IAccountMeta<string> = string,
  TAccountSystemProgram extends
    | string
    | IAccountMeta<string> = '11111111111111111111111111111111',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountPayer extends string
        ? WritableSignerAccount<TAccountPayer> &
            IAccountSignerMeta<TAccountPayer>
        : TAccountPayer,
      TAccountPaladinStakeProgram extends string
        ? ReadonlyAccount<TAccountPaladinStakeProgram>
        : TAccountPaladinStakeProgram,
      TAccountStakeConfig extends string
        ? WritableAccount<TAccountStakeConfig>
        : TAccountStakeConfig,
      TAccountPaladinRewardsProgram extends string
        ? ReadonlyAccount<TAccountPaladinRewardsProgram>
        : TAccountPaladinRewardsProgram,
      TAccountHolderRewardsPool extends string
        ? WritableAccount<TAccountHolderRewardsPool>
        : TAccountHolderRewardsPool,
      TAccountTokenMint extends string
        ? ReadonlyAccount<TAccountTokenMint>
        : TAccountTokenMint,
      TAccountSystemProgram extends string
        ? ReadonlyAccount<TAccountSystemProgram>
        : TAccountSystemProgram,
      ...TRemainingAccounts,
    ]
  >;

export type DistributeRewardsInstructionData = {
  discriminator: number;
  amount: bigint;
};

export type DistributeRewardsInstructionDataArgs = { amount: number | bigint };

export function getDistributeRewardsInstructionDataEncoder(): Encoder<DistributeRewardsInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['amount', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 0 })
  );
}

export function getDistributeRewardsInstructionDataDecoder(): Decoder<DistributeRewardsInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['amount', getU64Decoder()],
  ]);
}

export function getDistributeRewardsInstructionDataCodec(): Codec<
  DistributeRewardsInstructionDataArgs,
  DistributeRewardsInstructionData
> {
  return combineCodec(
    getDistributeRewardsInstructionDataEncoder(),
    getDistributeRewardsInstructionDataDecoder()
  );
}

export type DistributeRewardsInput<
  TAccountPayer extends string = string,
  TAccountPaladinStakeProgram extends string = string,
  TAccountStakeConfig extends string = string,
  TAccountPaladinRewardsProgram extends string = string,
  TAccountHolderRewardsPool extends string = string,
  TAccountTokenMint extends string = string,
  TAccountSystemProgram extends string = string,
> = {
  /** Payer account */
  payer: TransactionSigner<TAccountPayer>;
  /** Paladin Stake program */
  paladinStakeProgram: Address<TAccountPaladinStakeProgram>;
  /** Stake config account */
  stakeConfig: Address<TAccountStakeConfig>;
  /** Paladin Rewards program */
  paladinRewardsProgram: Address<TAccountPaladinRewardsProgram>;
  /** Holder rewards pool account */
  holderRewardsPool: Address<TAccountHolderRewardsPool>;
  /** Token mint */
  tokenMint: Address<TAccountTokenMint>;
  /** System program */
  systemProgram?: Address<TAccountSystemProgram>;
  amount: DistributeRewardsInstructionDataArgs['amount'];
};

export function getDistributeRewardsInstruction<
  TAccountPayer extends string,
  TAccountPaladinStakeProgram extends string,
  TAccountStakeConfig extends string,
  TAccountPaladinRewardsProgram extends string,
  TAccountHolderRewardsPool extends string,
  TAccountTokenMint extends string,
  TAccountSystemProgram extends string,
>(
  input: DistributeRewardsInput<
    TAccountPayer,
    TAccountPaladinStakeProgram,
    TAccountStakeConfig,
    TAccountPaladinRewardsProgram,
    TAccountHolderRewardsPool,
    TAccountTokenMint,
    TAccountSystemProgram
  >
): DistributeRewardsInstruction<
  typeof PALADIN_FUNNEL_PROGRAM_ADDRESS,
  TAccountPayer,
  TAccountPaladinStakeProgram,
  TAccountStakeConfig,
  TAccountPaladinRewardsProgram,
  TAccountHolderRewardsPool,
  TAccountTokenMint,
  TAccountSystemProgram
> {
  // Program address.
  const programAddress = PALADIN_FUNNEL_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    payer: { value: input.payer ?? null, isWritable: true },
    paladinStakeProgram: {
      value: input.paladinStakeProgram ?? null,
      isWritable: false,
    },
    stakeConfig: { value: input.stakeConfig ?? null, isWritable: true },
    paladinRewardsProgram: {
      value: input.paladinRewardsProgram ?? null,
      isWritable: false,
    },
    holderRewardsPool: {
      value: input.holderRewardsPool ?? null,
      isWritable: true,
    },
    tokenMint: { value: input.tokenMint ?? null, isWritable: false },
    systemProgram: { value: input.systemProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.systemProgram.value) {
    accounts.systemProgram.value =
      '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.payer),
      getAccountMeta(accounts.paladinStakeProgram),
      getAccountMeta(accounts.stakeConfig),
      getAccountMeta(accounts.paladinRewardsProgram),
      getAccountMeta(accounts.holderRewardsPool),
      getAccountMeta(accounts.tokenMint),
      getAccountMeta(accounts.systemProgram),
    ],
    programAddress,
    data: getDistributeRewardsInstructionDataEncoder().encode(
      args as DistributeRewardsInstructionDataArgs
    ),
  } as DistributeRewardsInstruction<
    typeof PALADIN_FUNNEL_PROGRAM_ADDRESS,
    TAccountPayer,
    TAccountPaladinStakeProgram,
    TAccountStakeConfig,
    TAccountPaladinRewardsProgram,
    TAccountHolderRewardsPool,
    TAccountTokenMint,
    TAccountSystemProgram
  >;

  return instruction;
}

export type ParsedDistributeRewardsInstruction<
  TProgram extends string = typeof PALADIN_FUNNEL_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Payer account */
    payer: TAccountMetas[0];
    /** Paladin Stake program */
    paladinStakeProgram: TAccountMetas[1];
    /** Stake config account */
    stakeConfig: TAccountMetas[2];
    /** Paladin Rewards program */
    paladinRewardsProgram: TAccountMetas[3];
    /** Holder rewards pool account */
    holderRewardsPool: TAccountMetas[4];
    /** Token mint */
    tokenMint: TAccountMetas[5];
    /** System program */
    systemProgram: TAccountMetas[6];
  };
  data: DistributeRewardsInstructionData;
};

export function parseDistributeRewardsInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedDistributeRewardsInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 7) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      payer: getNextAccount(),
      paladinStakeProgram: getNextAccount(),
      stakeConfig: getNextAccount(),
      paladinRewardsProgram: getNextAccount(),
      holderRewardsPool: getNextAccount(),
      tokenMint: getNextAccount(),
      systemProgram: getNextAccount(),
    },
    data: getDistributeRewardsInstructionDataDecoder().decode(instruction.data),
  };
}
