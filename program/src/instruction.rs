//! Program instruction types.

/// Instructions supported by the Paladin Rewards Funnel program.
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
