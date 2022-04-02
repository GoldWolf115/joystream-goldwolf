//! This pallet works with crowd funded bounties that allows a member, or the council, to crowd
//! fund work on projects with a public benefit.
//!
//! ### Bounty stages
//! - Funding - a bounty is being funded.
//! - FundingExpired - a bounty is expired. It can be only canceled.
//! - WorkSubmission - interested participants can submit their work.
//! - Judgment - working periods ended and the oracle should provide their judgment,
//!     winner work entrants receive their rewards, losers are slashed.
//! Oracle receives a reward for his work.
//! - SuccessfulBountyWithdrawal - contributors' state bloat bonds can be withdrawn,
//!     none judged work entrants can unlock their stakes.
//! - FailedBountyWithdrawal - contributors' funds can be withdrawn along with a split cherry,
//!     none judged work entrants can unlock their stakes.
//!
//! A detailed description could be found [here](https://github.com/Joystream/joystream/issues/1998).
//!
//! ### Supported extrinsics
//! - [create_bounty](./struct.Module.html#method.create_bounty) - creates a bounty
//!
//! #### Funding stage
//! - [cancel_bounty](./struct.Module.html#method.cancel_bounty) - cancels a bounty
//! - [veto_bounty](./struct.Module.html#method.veto_bounty) - vetoes a bounty
//! - [fund_bounty](./struct.Module.html#method.fund_bounty) - provide funding for a bounty
//! - [switch_oracle](./struct.Module.html#method.switch_oracle) - switch the current oracle by another one.
//!
//! #### FundingExpired stage
//! - [cancel_bounty](./struct.Module.html#method.cancel_bounty) - cancels a bounty
//! - [switch_oracle](./struct.Module.html#method.switch_oracle) - switch the current oracle by another one.
//!
//! #### Work submission stage
//! - [announce_work_entry](./struct.Module.html#method.announce_work_entry) - announce
//! work entry for a successful bounty.
//! - [switch_oracle](./struct.Module.html#method.switch_oracle) - switch the current oracle
//! by another one.
//! - [submit_work](./struct.Module.html#method.submit_work) - submit work for a bounty.
//! - [end_working_period](./struct.Module.html#method.end_working_period) - end working period by oracle.
//! - [terminate_bounty](./struct.Module.html#method.terminate_bounty) - terminate bounty
//! (into failed stage) by council.
//!
//! #### Judgment stage
//! - [submit_oracle_judgment](./struct.Module.html#method.submit_oracle_judgment) - submits an
//! oracle judgment for a bounty.
//!  - [switch_oracle](./struct.Module.html#method.switch_oracle) - switch the current oracle
//! by another one.
//!  - [terminate_bounty](./struct.Module.html#method.terminate_bounty) - terminate bounty (into failed stage).
//!
//! #### SuccessfulBountyWithdrawal stage
//! - [unlock_work_entrant_stake](./struct.Module.html#method.unlock_work_entrant_stake) -
//! unlock stake accounts refering to none judged work entries.
//!  - [withdraw_funder_state_bloat_bond_amount](./struct.Module.html#method.withdraw_funder_state_bloat_bond_amount) -
//! withdraw contributor's state bloat bond.
//!
//! #### FailedBountyWithdrawal stage
//!  - [unlock_work_entrant_stake](./struct.Module.html#method.unlock_work_entrant_stake) -
//! unlock stake accounts refering to none judged work entries.
//! - [withdraw_funding](./struct.Module.html#method.withdraw_funding) - Contributors can withdraw
//! funding for a failed bounty + a cherry fraction + state bloat bond.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
pub(crate) mod tests;

mod actors;
mod stages;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// pallet_bounty WeightInfo.
/// Note: This was auto generated through the benchmark CLI using the `--weight-trait` flag
pub trait WeightInfo {
    fn create_bounty_by_council(i: u32, j: u32) -> Weight;
    fn create_bounty_by_member(i: u32, j: u32) -> Weight;
    fn cancel_bounty_by_member() -> Weight;
    fn cancel_bounty_by_council() -> Weight;
    fn terminate_bounty() -> Weight;
    fn end_working_period() -> Weight;
    fn switch_oracle_to_council_by_council_approval_successful() -> Weight;
    fn switch_oracle_to_council_by_oracle_member() -> Weight;
    fn switch_oracle_to_member_by_oracle_member() -> Weight;
    fn switch_oracle_to_member_by_oracle_council() -> Weight;
    fn switch_oracle_to_member_by_council_not_oracle() -> Weight;
    fn fund_bounty_by_member() -> Weight;
    fn fund_bounty_by_council() -> Weight;
    fn withdraw_funding_by_member() -> Weight;
    fn withdraw_funding_by_council() -> Weight;
    fn announce_work_entry(i: u32) -> Weight;
    fn submit_work(i: u32) -> Weight;
    fn submit_oracle_judgment_by_council_all_winners(i: u32) -> Weight;
    fn submit_oracle_judgment_by_council_all_rejected(i: u32, j: u32) -> Weight;
    fn submit_oracle_judgment_by_member_all_winners(i: u32) -> Weight;
    fn submit_oracle_judgment_by_member_all_rejected(i: u32, j: u32) -> Weight;
    fn unlock_work_entrant_stake() -> Weight;
    fn withdraw_funder_state_bloat_bond_amount_by_council() -> Weight;
    fn withdraw_funder_state_bloat_bond_amount_by_member() -> Weight;
    fn withdraw_oracle_reward_by_oracle_council() -> Weight;
    fn withdraw_oracle_reward_by_oracle_member() -> Weight;
}

type WeightInfoBounty<T> = <T as Trait>::WeightInfo;

pub(crate) use actors::BountyActorManager;
// use council::Balance;
pub(crate) use stages::BountyStageCalculator;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::{Currency, ExistenceRequirement, Get, LockIdentifier};
use frame_support::weights::Weight;
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, Parameter};
use frame_system::ensure_root;
use sp_arithmetic::traits::{One, Saturating, Zero};
use sp_runtime::{traits::AccountIdConversion, ModuleId};
use sp_runtime::{Perbill, SaturatedConversion};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::vec::Vec;

use common::council::CouncilBudgetManager;
use common::membership::{
    MemberId, MemberOriginValidator, MembershipInfoProvider, StakingAccountValidator,
};
use staking_handler::StakingHandler;

/// Main pallet-bounty trait.
pub trait Trait:
    frame_system::Trait + balances::Trait + common::membership::MembershipTypes
{
    /// Events
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

    /// The bounty's module id, used for deriving its sovereign account ID.
    type ModuleId: Get<ModuleId>;

    /// Bounty Id type
    type BountyId: From<u32> + Parameter + Default + Copy;

    /// Validates staking account ownership for a member, member ID and origin combination and
    /// providers controller id for a member.
    type Membership: StakingAccountValidator<Self>
        + MembershipInfoProvider<Self>
        + MemberOriginValidator<Self::Origin, MemberId<Self>, Self::AccountId>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;

    /// Provides an access for the council budget.
    type CouncilBudgetManager: CouncilBudgetManager<BalanceOf<Self>>;

    /// Provides stake logic implementation.
    type StakingHandler: StakingHandler<
        Self::AccountId,
        BalanceOf<Self>,
        MemberId<Self>,
        LockIdentifier,
    >;

    /// Work entry Id type
    type EntryId: From<u32> + Parameter + Default + Copy + Ord + One;

    /// Defines max work entry number for a closed assurance type contract bounty.
    type ClosedContractSizeLimit: Get<u32>;

    /// Defines min work entrant stake for a bounty.
    type MinWorkEntrantStake: Get<BalanceOf<Self>>;
}

/// Alias type for the BountyParameters.
pub type BountyCreationParameters<T> = BountyParameters<
    BalanceOf<T>,
    <T as frame_system::Trait>::BlockNumber,
    <T as common::membership::MembershipTypes>::MemberId,
>;

/// Defines who can submit the work.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum AssuranceContractType<MemberId: Ord> {
    /// Anyone can submit the work.
    Open,

    /// Only specific members can submit the work.
    Closed(BTreeSet<MemberId>),
}

impl<MemberId: Ord> Default for AssuranceContractType<MemberId> {
    fn default() -> Self {
        AssuranceContractType::Open
    }
}

/// Defines funding conditions.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum FundingType<BlockNumber, Balance> {
    /// Funding has no time limits.
    Perpetual {
        /// Desired funding.
        target: Balance,
    },

    /// Funding has a time limitation.
    Limited {
        /// Desired funding.
        target: Balance,

        /// target allowed funding period.
        funding_period: BlockNumber,
    },
}

impl<BlockNumber, Balance: Default> Default for FundingType<BlockNumber, Balance> {
    fn default() -> Self {
        Self::Perpetual {
            target: Default::default(),
        }
    }
}

/// Defines parameters for the bounty creation.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct BountyParameters<Balance, BlockNumber, MemberId: Ord> {
    /// Origin that will select winner(s), is either a given member or a council.
    pub oracle: BountyActor<MemberId>,

    /// Contract type defines who can submit the work.
    pub contract_type: AssuranceContractType<MemberId>,

    /// Bounty creator: could be a member or a council.
    pub creator: BountyActor<MemberId>,

    /// An amount of funding provided by the creator which will be split among all other
    /// contributors should the bounty be successful. If not successful, cherry is returned to
    /// the creator. When council is creating bounty, this comes out of their budget, when a member
    /// does it, it comes from an account.
    pub cherry: Balance,

    /// A reward provided by the creator which will be attributed to the
    /// oracle should the oracle submit a Judgment. even if this Judgment is negative, this reward should be attributed to
    /// the oracle. When council is creating bounty, this comes out of their budget, when a member
    /// does it, it comes from an account.
    pub oracle_reward: Balance,

    /// Amount of stake required to enter bounty as entrant.
    pub entrant_stake: Balance,

    /// Defines parameters for different funding types.
    pub funding_type: FundingType<BlockNumber, Balance>,
}

/// Bounty actor to perform operations for a bounty.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum BountyActor<MemberId> {
    /// Council performs operations for a bounty.
    Council,

    /// Member performs operations for a bounty.
    Member(MemberId),
}

impl<MemberId> Default for BountyActor<MemberId> {
    fn default() -> Self {
        BountyActor::Council
    }
}

/// Defines current bounty stage.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, Copy)]
pub enum BountyStage {
    /// Bounty funding stage.
    Funding {
        /// Bounty has already some contributions.
        has_contributions: bool,
    },

    /// Bounty funding period expired with no contributions.
    FundingExpired,

    /// Bounty cancelled in funding period expired with no contributions.
    Cancelled,

    /// A bounty has gathered necessary funds and ready to accept work submissions.
    WorkSubmission,

    /// Working periods ended and the oracle should provide their judgment.
    Judgment,

    /// Indicates a withdrawal on bounty success. Workers get rewards and their stake.
    SuccessfulBountyWithdrawal,

    /// Indicates a withdrawal on bounty failure. Workers get their stake back. Funders
    /// get their contribution back as well as part of the cherry.
    FailedBountyWithdrawal,
}

/// Defines current bounty state.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum BountyMilestone<BlockNumber> {
    /// Bounty was created at given block number.
    /// Boolean value defines whether the bounty has some funding contributions.
    Created {
        /// Bounty creation block.
        created_at: BlockNumber,
        /// Bounty has already some contributions.
        has_contributions: bool,
    },

    /// A bounty funding was successful and it exceeded max funding amount.
    BountyMaxFundingReached {
        ///  A bounty funding was successful on the provided block.
        target_funding_reached_at: BlockNumber,
    },

    /// Bounty cancelled in funding period expired with no contributions.
    CancelledBounty,

    /// Oracle ended the work submission stage.
    WorkSubmitted,

    /// Council terminated this bounty
    OraclePerpetualWorkingInterruption,

    /// A judgment was submitted for a bounty.
    JudgmentSubmitted {
        ///This flag indicates the judgment result (there is at least one work entrant winner),
        ///this is necessary in the stage logic, to determine if the bounty is in
        ///the SuccessfulBountyWithdrawal or FailedBountyWithdrawal stage.
        successful_bounty: bool,
    },
}

impl<BlockNumber: Default> Default for BountyMilestone<BlockNumber> {
    fn default() -> Self {
        BountyMilestone::Created {
            created_at: Default::default(),
            has_contributions: false,
        }
    }
}

/// Alias type for the Bounty.
pub type Bounty<T> = BountyRecord<
    BalanceOf<T>,
    <T as frame_system::Trait>::BlockNumber,
    <T as common::membership::MembershipTypes>::MemberId,
>;

/// Crowdfunded bounty record.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct BountyRecord<Balance, BlockNumber, MemberId: Ord> {
    /// Bounty creation parameters.
    pub creation_params: BountyParameters<Balance, BlockNumber, MemberId>,

    /// Total funding balance reached so far.
    /// Includes initial funding by a creator and other members funding.
    pub total_funding: Balance,

    /// Bounty current milestone(state). It represents fact known about the bounty, eg.:
    /// it was canceled or max funding amount was reached.
    pub milestone: BountyMilestone<BlockNumber>,

    /// Current active work entry counter.
    pub active_work_entry_count: u32,

    ///This flag is set to true, if oracle called withdraw_oracle_reward.
    pub oracle_withdrew_reward: bool,
}

impl<Balance: PartialOrd + Clone, BlockNumber: Clone, MemberId: Ord>
    BountyRecord<Balance, BlockNumber, MemberId>
{
    // Increments bounty active work entry counter.
    fn increment_active_work_entry_counter(&mut self) {
        self.active_work_entry_count += 1;
    }

    // Decrements bounty active work entry counter. Nothing happens on zero counter.
    fn decrement_active_work_entry_counter(&mut self) {
        if self.active_work_entry_count > 0 {
            self.active_work_entry_count -= 1;
        }
    }

    // Defines whether the target funding amount will be reached for the current funding type.
    fn is_target_funding_reached(&self, total_funding: Balance) -> bool {
        let target = match self.creation_params.funding_type {
            FundingType::Perpetual { ref target } => target.clone(),
            FundingType::Limited { ref target, .. } => target.clone(),
        };

        total_funding >= target
    }

    // Returns the target funding amount for the current funding type.
    pub(crate) fn target_funding(&self) -> Balance {
        match self.creation_params.funding_type {
            FundingType::Perpetual { ref target } => target.clone(),
            FundingType::Limited { ref target, .. } => target.clone(),
        }
    }
}

/// Alias type for the Entry.
pub type Entry<T> = EntryRecord<
    <T as frame_system::Trait>::AccountId,
    <T as common::membership::MembershipTypes>::MemberId,
    <T as frame_system::Trait>::BlockNumber,
>;

/// Work entry.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug)]
pub struct EntryRecord<AccountId, MemberId, BlockNumber> {
    /// Work entrant member ID.
    pub member_id: MemberId,

    /// Account ID for staking lock.
    pub staking_account_id: AccountId,

    /// Work entry submission block.
    pub submitted_at: BlockNumber,

    /// Signifies that an entry has at least one submitted work.
    pub work_submitted: bool,
}

/// Defines the oracle judgment for the work entry.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub enum OracleWorkEntryJudgment<Balance> {
    /// The work entry is selected as a winner.
    Winner { reward: Balance },

    /// The work entry is considered harmful. The stake will be slashed.
    Rejected {
        ///The percent share (0 - 1) to slash.
        slashing_share: Perbill,

        /// After slash takes place the rest of the locked balance will be unlocked,
        /// the council has the option to give description why slash happened.
        action_justification: Vec<u8>,
    },
}

impl<Balance> OracleWorkEntryJudgment<Balance> {
    // Work entry judgment helper. Returns true for winners.
    pub(crate) fn is_winner(&self) -> bool {
        matches!(*self, Self::Winner { .. })
    }
}

/// Balance alias for `balances` module.
pub type BalanceOf<T> = <T as balances::Trait>::Balance;

// Entrant stake helper struct.
struct RequiredStakeInfo<T: Trait> {
    // stake amount
    amount: BalanceOf<T>,
    // staking_account_id
    account_id: T::AccountId,
}

// Current state bloat bond a funder has to pay to contribute (one time payment).
// The funder can withdraw the bond after deleting the BountyContributions entry belonging to him
const FUNDER_STATE_BLOAT_BOND_AMOUNT: u32 = 10;
const CREATOR_STATE_BLOAT_BOND_AMOUNT: u32 = 10;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct Contribution<T: Trait> {
    // contribution amount
    amount: BalanceOf<T>,
    // amount of bloat bond a funder had to paid
    funder_state_bloat_bond_amount: BalanceOf<T>,
}
impl<T: Trait> Contribution<T> {
    fn add_funds(&self, amount: BalanceOf<T>) -> Contribution<T> {
        Contribution::<T> {
            amount: self.amount.saturating_add(amount),
            funder_state_bloat_bond_amount: T::Balance::from(FUNDER_STATE_BLOAT_BOND_AMOUNT),
        }
    }
    ///Get the sum of state bloat and amount if it's the the first contribution (zero amount)
    fn get_total_amount(&self) -> BalanceOf<T> {
        self.amount
            .saturating_add(self.funder_state_bloat_bond_amount)
    }
}

impl<T: Trait> Default for Contribution<T> {
    fn default() -> Self {
        Self {
            amount: Zero::zero(),
            funder_state_bloat_bond_amount: T::Balance::from(FUNDER_STATE_BLOAT_BOND_AMOUNT),
        }
    }
}
/// An alias for the OracleJudgment.
pub type OracleJudgmentOf<T> = OracleJudgment<<T as Trait>::EntryId, BalanceOf<T>>;

/// The collection of the oracle judgments for the work entries.
pub type OracleJudgment<EntryId, Balance> = BTreeMap<EntryId, OracleWorkEntryJudgment<Balance>>;

decl_storage! {
    trait Store for Module<T: Trait> as Bounty {
        /// Bounty storage.
        pub Bounties get(fn bounties) : map hasher(blake2_128_concat) T::BountyId => Bounty<T>;

        /// Double map for bounty funding. It stores a member or council funding for bounties.
        pub BountyContributions get(fn contribution_by_bounty_by_actor): double_map
            hasher(blake2_128_concat) T::BountyId,
            hasher(blake2_128_concat) BountyActor<MemberId<T>> => Contribution<T>;

        /// Count of all bounties that have been created.
        pub BountyCount get(fn bounty_count): u32;

        /// Work entry storage map.
        pub Entries get(fn entries): map hasher(blake2_128_concat) T::EntryId => Entry<T>;

        /// Count of all work entries that have been created.
        pub EntryCount get(fn entry_count): u32;
    }
}

decl_event! {
    pub enum Event<T>
    where
        <T as Trait>::BountyId,
        <T as Trait>::EntryId,
        Balance = BalanceOf<T>,
        MemberId = MemberId<T>,
        <T as frame_system::Trait>::AccountId,
        BountyCreationParameters = BountyCreationParameters<T>,
        OracleJudgment = OracleJudgmentOf<T>,
    {
        /// A bounty was created.
        /// Params:
        /// - bounty ID
        /// - creation parameters
        /// - bounty metadata
        BountyCreated(BountyId, BountyCreationParameters, Vec<u8>),

        /// Bounty Oracle Switching by current oracle.
        /// Params:
        /// - bounty ID
        /// - Previous oracle
        /// - New oracle
        BountyOracleSwitchingByCurrentOracle(BountyId, BountyActor<MemberId>, BountyActor<MemberId>),

        /// Bounty Oracle Switching by council approval.
        /// Params:
        /// - bounty ID
        /// - Previous oracle
        /// - New oracle
        BountyOracleSwitchingByCouncilApproval(BountyId, BountyActor<MemberId>, BountyActor<MemberId>),

        /// A bounty was canceled.
        /// Params:
        /// - bounty ID
        /// - bounty creator
        BountyCanceled(BountyId, BountyActor<MemberId>),

        /// A bounty was terminated by council.
        /// Params:
        /// - bounty ID
        /// - bounty creator
        /// - bounty oracle
        BountyTerminatedByCouncil(BountyId, BountyActor<MemberId>, BountyActor<MemberId>),

        /// A bounty was funded by a member or a council.
        /// Params:
        /// - bounty ID
        /// - bounty funder
        /// - funding amount
        BountyFunded(BountyId, BountyActor<MemberId>, Balance),

        /// A bounty has reached its target funding amount.
        /// Params:
        /// - bounty ID
        BountyMaxFundingReached(BountyId),

        /// A member or a council has withdrawn the funding.
        /// Params:
        /// - bounty ID
        /// - bounty funder
        BountyFundingWithdrawal(BountyId, BountyActor<MemberId>),

        /// A bounty creator has withdrawn the cherry (member or council).
        /// Params:
        /// - bounty ID
        /// - bounty creator
        BountyCreatorCherryWithdrawal(BountyId, BountyActor<MemberId>),

        /// A bounty creator has withdrawn the oracle reward (member or council).
        /// Params:
        /// - bounty ID
        /// - bounty creator
        BountyCreatorOracleRewardWithdrawal(BountyId, BountyActor<MemberId>),

        /// A Oracle has withdrawn the oracle reward (member or council).
        /// Params:
        /// - bounty ID
        /// - bounty creator
        /// - Oracle Reward
        BountyOracleRewardWithdrawal(BountyId, BountyActor<MemberId>, Balance),

        /// A bounty was removed.
        /// Params:
        /// - bounty ID
        BountyRemoved(BountyId),

        /// Work entry was announced.
        /// Params:
        /// - bounty ID
        /// - created entry ID
        /// - entrant member ID
        /// - staking account ID
        WorkEntryAnnounced(BountyId, EntryId, MemberId, AccountId),

        /// Submit work.
        /// Params:
        /// - bounty ID
        /// - created entry ID
        /// - entrant member ID
        /// - work data (description, URL, BLOB, etc.)
        WorkSubmitted(BountyId, EntryId, MemberId, Vec<u8>),

        /// Submit oracle judgment.
        /// Params:
        /// - bounty ID
        /// - oracle
        /// - judgment data
        OracleJudgmentSubmitted(BountyId, BountyActor<MemberId>, OracleJudgment),

        /// Work entry was slashed.
        /// Params:
        /// - bounty ID
        /// - entry ID
        /// - entrant member ID
        WorkEntrantFundsWithdrawn(BountyId, EntryId, MemberId),

        /// Work entry was slashed.
        /// Params:
        /// - bounty ID
        /// - oracle (caller)
        WorkSubmissionPeriodEnded(BountyId, BountyActor<MemberId>),

        /// Work entry stake unlocked.
        /// Params:
        /// - bounty ID
        /// - entry ID
        /// - stake account
        WorkEntrantStakeUnlocked(
            BountyId,
            EntryId,
            AccountId),

        /// A member or a council funder has withdrawn the funder state bloat bond.
        /// Params:
        /// - bounty ID
        /// - bounty funder
        /// - funder State bloat bond amount
        FunderStateBloatBondWithdrawn(
            BountyId,
            BountyActor<MemberId>,
            Balance),

        /// A member or a council creator has withdrawn the creator state bloat bond.
        /// Params:
        /// - bounty ID
        /// - bounty creator
        /// - Creator State bloat bond amount
        CreatorStateBloatBondWithdrawn(
            BountyId,
            BountyActor<MemberId>,
            Balance),
    }
}

decl_error! {
    /// Bounty pallet predefined errors
    pub enum Error for Module<T: Trait> {

        /// Min funding amount cannot be greater than max amount.
        MinFundingAmountCannotBeGreaterThanMaxAmount,

        /// Bounty doesnt exist.
        BountyDoesntExist,

        /// Origin is root, so switching oracle is not allowed in this extrinsic. (call switch_oracle_as_root)
        SwitchOracleOriginIsRoot,

        /// Unexpected bounty stage for an operation: Funding.
        InvalidStageUnexpectedFunding,

        /// Unexpected bounty stage for an operation: FundingExpired.
        InvalidStageUnexpectedFundingExpired,

        /// Unexpected bounty stage for an operation: Cancelled.
        InvalidStageUnexpectedCancelled,

        /// Unexpected bounty stage for an operation: WorkSubmission.
        InvalidStageUnexpectedWorkSubmission,

        /// Unexpected bounty stage for an operation: Judgment.
        InvalidStageUnexpectedJudgment,

        /// Unexpected bounty stage for an operation: SuccessfulBountyWithdrawal.
        InvalidStageUnexpectedSuccessfulBountyWithdrawal,

        /// Unexpected bounty stage for an operation: FailedBountyWithdrawal.
        InvalidStageUnexpectedFailedBountyWithdrawal,

        /// Insufficient balance for a bounty cherry.
        InsufficientBalanceForBounty,

        /// Cannot found bounty contribution.
        NoBountyContributionFound,

        /// There is not enough balance for a stake.
        InsufficientBalanceForStake,

        /// The conflicting stake discovered. Cannot stake.
        ConflictingStakes,

        /// Work entry doesnt exist.
        WorkEntryDoesntExist,

        /// Cherry less than minimum allowed.
        CherryLessThenMinimumAllowed,

        /// Incompatible assurance contract type for a member: cannot submit work to the 'closed
        /// assurance' bounty contract.
        CannotSubmitWorkToClosedContractBounty,

        /// Cannot create a 'closed assurance contract' bounty with empty member list.
        ClosedContractMemberListIsEmpty,

        /// Cannot create a 'closed assurance contract' bounty with member list larger
        /// than allowed max work entry limit.
        ClosedContractMemberListIsTooLarge,

        /// Staking account doesn't belong to a member.
        InvalidStakingAccountForMember,

        /// Cannot set zero reward for winners.
        ZeroWinnerReward,

        /// The total reward for winners should be equal to total bounty funding.
        TotalRewardShouldBeEqualToTotalFunding,

        /// Cannot create a bounty with an entrant stake is less than required minimum.
        EntrantStakeIsLessThanMininum,

        /// Cannot create a bounty with zero funding amount parameter.
        FundingAmountCannotBeZero,

        /// Cannot create a bounty with zero funding period parameter.
        FundingPeriodCannotBeZero,

        /// Invalid judgment - all winners should have work submissions.
        WinnerShouldHasWorkSubmission,

        ///Cannot withdraw oracle reward from bounty account, because of insufficient balance,
        NoBountyBalanceToOracleRewardWithdrawal,

        ///Worker tried to access a work entry that doesn't belong to him
        WorkEntryDoesntBelongToWorker,

        ///Cannot withdraw a zero amount reward
        OracleRewardIsZero,

        ///Oracle have already been withdrawn
        OracleRewardAlreadyWithdrawn
    }
}

decl_module! {
    /// Bounty pallet Substrate Module
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        /// Predefined errors
        type Error = Error<T>;

        /// Emits an event. Default substrate implementation.
        fn deposit_event() = default;

        /// Exports const - max work entry number for a closed assurance type contract bounty.
        const ClosedContractSizeLimit: u32 = T::ClosedContractSizeLimit::get();

        /// Exports const - min work entrant stake for a bounty.
        const MinWorkEntrantStake: BalanceOf<T> = T::MinWorkEntrantStake::get();

        /// Creates a bounty. Metadata stored in the transaction log but discarded after that.
        /// <weight>
        ///
        /// ## Weight
        /// `O (W)` where:
        /// - `W` is the _metadata length.
        /// - `M` is closed contract member list length.
        /// - DB:
        ///    - O(M) (O(1) on open contract)
        /// # </weight>
        #[weight = Module::<T>::create_bounty_weight(&params, &metadata)]
        pub fn create_bounty(origin, params: BountyCreationParameters<T>, metadata: Vec<u8>) {

            let bounty_creator_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                params.creator.clone()
            )?;

            Self::ensure_create_bounty_parameters_valid(&params)?;

            let amount = params.cherry
            .saturating_add(params.oracle_reward)
            .saturating_add(T::Balance::from(CREATOR_STATE_BLOAT_BOND_AMOUNT));
            bounty_creator_manager.validate_balance_sufficiency(amount)?;

            //
            // == MUTATION SAFE ==
            //

            let next_bounty_count_value = Self::bounty_count() + 1;
            let bounty_id = T::BountyId::from(next_bounty_count_value);

            bounty_creator_manager.transfer_funds_to_bounty_account(bounty_id, amount);

            let created_bounty_milestone = BountyMilestone::Created {
                created_at: Self::current_block(),
                has_contributions: false, // just created - no contributions
            };

            let bounty = Bounty::<T> {
                total_funding: Zero::zero(),
                creation_params: params.clone(),
                milestone: created_bounty_milestone,
                active_work_entry_count: 0,
                oracle_withdrew_reward: false
            };

            <Bounties<T>>::insert(bounty_id, bounty);
            BountyCount::mutate(|count| {
                *count = next_bounty_count_value
            });
            Self::deposit_event(RawEvent::BountyCreated(bounty_id, params, metadata));
        }

        /// Cancels a bounty.
        /// It returns a cherry to creator and removes bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::cancel_bounty_by_member()
              .max(WeightInfoBounty::<T>::cancel_bounty_by_council())]
        pub fn cancel_bounty(origin, bounty_id: T::BountyId) {

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;
            let bounty_creator_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                bounty.creation_params.creator.clone(),
            )?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage_for_canceling(current_bounty_stage)?;

            //
            // == MUTATION SAFE ==
            //

            Self::return_bounty_cherry_to_creator(bounty_id, &bounty, &bounty_creator_manager);
            let creator = bounty.creation_params.creator.clone();
            Self::deposit_event(RawEvent::BountyCanceled(bounty_id, creator));

            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::CancelledBounty;
            });

            if Self::can_remove_bounty(&bounty_id, &bounty) {
                Self::remove_bounty(
                    &bounty_id,
                    &bounty,
                    &bounty_creator_manager
                );
            }
        }

        /// Provides bounty funding.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::fund_bounty_by_member()
              .max(WeightInfoBounty::<T>::fund_bounty_by_council())]
        pub fn fund_bounty(
            origin,
            funder: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
            amount: BalanceOf<T>
        ) {
            let bounty_funder_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                funder.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(
                matches!(current_bounty_stage, BountyStage::Funding{..}),
                Self::unexpected_bounty_stage_error(current_bounty_stage),
            );

            //contribution is adjusted to prevent exceeding target funding.
            let (is_target_funding_reached,
                is_first_contribution,
                adjusted_amount,
                adjusted_contribution) =
                Self::get_adjusted_contribution(&bounty_id, &bounty, &funder, amount);
            let state_bloat_bond_amount = adjusted_contribution.funder_state_bloat_bond_amount;

            let transfer_amount = match is_first_contribution {
                //If It's the first contribution, the transfer amount includes the state bloat bond amount.
                true => adjusted_amount.saturating_add(state_bloat_bond_amount),
                false => adjusted_amount
            };

            bounty_funder_manager.validate_balance_sufficiency(transfer_amount)?;

            //
            // == MUTATION SAFE ==
            //

            bounty_funder_manager
                .transfer_funds_to_bounty_account(bounty_id, transfer_amount);

            let new_milestone = Self::get_bounty_milestone_on_funding(
                is_target_funding_reached,
                bounty.milestone);

            // Update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                //Updates only the funds not the bloat bond.
                bounty.total_funding = bounty.total_funding.saturating_add(adjusted_amount);
                bounty.milestone = new_milestone;
            });

            // Update member funding record
            <BountyContributions<T>>::insert(bounty_id, funder.clone(), adjusted_contribution);

            // Fire events.
            Self::deposit_event(RawEvent::BountyFunded(bounty_id, funder, adjusted_amount));
            if is_target_funding_reached{
                Self::deposit_event(RawEvent::BountyMaxFundingReached(bounty_id));
            }
        }

        /// Terminates a bounty with perpetual working period.
        /// It returns a oracle cherry to creator.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::terminate_bounty()]
        pub fn terminate_bounty(origin, bounty_id: T::BountyId) {

            ensure_root(origin)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let bounty_creator_manager = Self::ensure_creator_actor_manager(&bounty)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(matches!(current_bounty_stage,
                BountyStage::Funding { .. }|
                BountyStage::WorkSubmission|
                BountyStage::Judgment),
                Self::unexpected_bounty_stage_error(current_bounty_stage));

            //
            // == MUTATION SAFE ==
            //

            Self::deposit_event(
                RawEvent::BountyTerminatedByCouncil(
                    bounty_id,
                    bounty.creation_params.creator.clone(),
                    bounty.creation_params.oracle.clone()));

            if let BountyStage::Funding { has_contributions: false } = current_bounty_stage{
                Self::return_bounty_cherry_to_creator(bounty_id, &bounty, &bounty_creator_manager);
            }
            // Update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::OraclePerpetualWorkingInterruption
            });
        }

        ///Council switches the oracle to a new one
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        ///
        #[weight = WeightInfoBounty::<T>::switch_oracle_to_member_by_oracle_council()
        .max(WeightInfoBounty::<T>::switch_oracle_to_member_by_council_not_oracle())
        .max(WeightInfoBounty::<T>::switch_oracle_to_council_by_council_approval_successful())]
        pub fn switch_oracle_as_root(
            origin,
            new_oracle: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
        ) {
            ensure_root(origin)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_oracle = bounty.creation_params.oracle.clone();

            //Check if new oracle is a member
            BountyActorManager::<T>::get_bounty_actor_manager(new_oracle.clone())?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(
                matches!(current_bounty_stage,
                    BountyStage::Funding{..} |
                    BountyStage::WorkSubmission |
                    BountyStage::Judgment
                ),
                Self::unexpected_bounty_stage_error(current_bounty_stage)
            );

            //
            // == MUTATION SAFE ==
            //

            //Mutates the bounty params replacing the current oracle
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.creation_params.oracle  = new_oracle.clone()
            });

            Self::deposit_event(RawEvent::BountyOracleSwitchingByCouncilApproval (
                bounty_id,
                current_oracle,
                new_oracle));
        }

        ///Oracle switches himself to a new one
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        ///
        #[weight = WeightInfoBounty::<T>::switch_oracle_to_council_by_oracle_member()
        .max(WeightInfoBounty::<T>::switch_oracle_to_member_by_oracle_member())]
        pub fn switch_oracle(
            origin,
            new_oracle: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
        ) {
            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_oracle = bounty.creation_params.oracle.clone();

            //Checks if the function caller (origin) is current oracle
            let actor_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                current_oracle.clone(),
            )?;

            ensure!(actor_manager != BountyActorManager::Council,
                Error::<T>::SwitchOracleOriginIsRoot);

            //Check if new oracle is a member
            BountyActorManager::<T>::get_bounty_actor_manager(new_oracle.clone())?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(
                matches!(current_bounty_stage,
                    BountyStage::Funding{..} |
                    BountyStage::WorkSubmission |
                    BountyStage::Judgment
                ),
                Self::unexpected_bounty_stage_error(current_bounty_stage)
            );

            //
            // == MUTATION SAFE ==
            //

            //Mutates the bounty params replacing the current oracle
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.creation_params.oracle  = new_oracle.clone()
            });

            Self::deposit_event(RawEvent::BountyOracleSwitchingByCurrentOracle(
                bounty_id,
                current_oracle,
                new_oracle));
        }

        /// Withdraw bounty funding by a member or a council.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::withdraw_funding_by_member()
              .max(WeightInfoBounty::<T>::withdraw_funding_by_council())]
        pub fn withdraw_funding(
            origin,
            funder: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
        ) {
            let bounty_funder_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                funder.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(
                current_bounty_stage == BountyStage::FailedBountyWithdrawal,
                Self::unexpected_bounty_stage_error(current_bounty_stage)
            );

            let funding = Self::ensure_bounty_contribution_exists(&bounty_id, &funder)?;

            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor_manager(
                bounty.creation_params.creator.clone(),
            )?;

            //
            // == MUTATION SAFE ==
            //

            let cherry_fraction = Self::get_cherry_fraction_for_member(&bounty, funding.amount);

            let withdrawal_amount = funding.get_total_amount().saturating_add(cherry_fraction);

            bounty_funder_manager.transfer_funds_from_bounty_account(bounty_id, withdrawal_amount);

            <BountyContributions<T>>::remove(&bounty_id, &funder);

            Self::deposit_event(RawEvent::BountyFundingWithdrawal(bounty_id, funder));

            if Self::can_remove_bounty(&bounty_id, &bounty) {
                Self::remove_bounty(
                    &bounty_id,
                    &bounty,
                    &bounty_creator_manager
                );
            }
        }

        /// Announce work entry for a successful bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::announce_work_entry(T::ClosedContractSizeLimit::get()
            .saturated_into())]
        pub fn announce_work_entry(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            staking_account_id: T::AccountId,
        ) {
            T::Membership::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::WorkSubmission)?;

            let stake = Self::validate_entrant_stake(
                member_id,
                &bounty,
                staking_account_id.clone()
            )?;

            Self::ensure_valid_contract_type(&bounty, &member_id)?;

            //
            // == MUTATION SAFE ==
            //

            let next_entry_count_value = Self::entry_count() + 1;
            let entry_id = T::EntryId::from(next_entry_count_value);

            // Lock stake balance for bounty if the stake is required.
            if let Some(stake) = stake {
                T::StakingHandler::lock(&stake.account_id, stake.amount);
            }

            let entry = Entry::<T> {
                member_id,
                staking_account_id: staking_account_id.clone(),
                submitted_at: Self::current_block(),
                work_submitted: false,
            };

            <Entries<T>>::insert(entry_id, entry);
            EntryCount::mutate(|count| {
                *count = next_entry_count_value
            });

            // Increment work entry counter and update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.increment_active_work_entry_counter();
            });

            Self::deposit_event(RawEvent::WorkEntryAnnounced(
                bounty_id,
                entry_id,
                member_id,
                staking_account_id,
            ));
        }

        /// Submit work for a bounty.
        /// # <weight>
        ///
        /// ## weight
        /// `O (N)`
        /// - `N` is the work_data length,
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight =  WeightInfoBounty::<T>::submit_work(work_data.len().saturated_into())]
        pub fn submit_work(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            entry_id: T::EntryId,
            work_data: Vec<u8>
        ) {
            T::Membership::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::WorkSubmission)?;

            let entry = Self::ensure_work_entry_exists(&entry_id)?;

            Self::ensure_work_entry_ownership(&entry, &member_id)?;

            //
            // == MUTATION SAFE ==
            //

            // Update entry
            <Entries<T>>::mutate(entry_id, |entry| {
                entry.work_submitted = true;
            });

            Self::deposit_event(RawEvent::WorkSubmitted(bounty_id, entry_id, member_id, work_data));
        }

        /// end bounty working period.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::end_working_period()]
        pub fn end_working_period(
            origin,
            bounty_id: T::BountyId,
        ) {
            let bounty = Self::ensure_bounty_exists(&bounty_id)?;
            let current_oracle = bounty.creation_params.oracle.clone();

            //Checks if the function caller (origin) is current oracle
            BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                current_oracle.clone(),
            )?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::WorkSubmission)?;

            //
            // == MUTATION SAFE ==
            //

            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::WorkSubmitted;
            });
            Self::deposit_event(RawEvent::WorkSubmissionPeriodEnded(bounty_id, current_oracle));
        }

        /// Submits an oracle judgment for a bounty, slashing the entries rejected
        /// by an arbitrary percentage and rewarding the winners by an arbitrary amount
        /// (not surpassing the total fund amount)
        /// # <weight>
        ///
        /// ## weight
        /// `O (N)`
        /// - `N` is the work_data length,
        /// - db:
        ///    - `O(N)`
        /// # </weight>
        #[weight = Module::<T>::submit_oracle_judgment_weight(&judgment)]
        pub fn submit_oracle_judgment(
            origin,
            bounty_id: T::BountyId,
            judgment: OracleJudgment<T::EntryId, BalanceOf<T>>
        ) {
            let bounty = Self::ensure_bounty_exists(&bounty_id)?;
            BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                bounty.creation_params.oracle.clone(),
            )?;

            let bounty_creator_manager = Self::ensure_creator_actor_manager(&bounty)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            Self::ensure_bounty_stage(current_bounty_stage, BountyStage::Judgment)?;

            Self::validate_judgment(&bounty, &judgment)?;

            // Lookup for any winners in the judgment.
            let successful_bounty = Self::judgment_has_winners(&judgment);

            //
            // == MUTATION SAFE ==
            //

            // Return a cherry to a creator.
            if successful_bounty {
                Self::return_bounty_cherry_to_creator(bounty_id, &bounty, &bounty_creator_manager);
            }

            // Update bounty record.
            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.milestone = BountyMilestone::JudgmentSubmitted{
                    successful_bounty
                };
            });

            // Judgments triage.
            for (entry_id, work_entry_judgment) in judgment.iter() {
                // Update work entries for winners.
                match *work_entry_judgment{
                    OracleWorkEntryJudgment::Winner{ reward } => {

                        let entry = Self::entries(entry_id);
                        // Unstake the full work entry state.
                        let worker_account_id = T::Membership::controller_account_id(entry.member_id)?;

                        T::StakingHandler::unlock(&entry.staking_account_id);
                        // Claim the winner reward.

                        Self::transfer_funds_from_bounty_account(
                            &worker_account_id,
                            bounty_id,
                            reward
                        );
                        // Delete the work entry record from the storage.
                        Self::remove_work_entry(&bounty_id, &entry_id);

                        // Fire an event.
                        Self::deposit_event(RawEvent::WorkEntrantFundsWithdrawn(bounty_id, *entry_id, entry.member_id));

                    },
                    OracleWorkEntryJudgment::Rejected{
                        slashing_share,
                        ..
                    } => {
                        let entry = Self::entries(entry_id);
                        let slashing_amount = slashing_share * bounty.creation_params.entrant_stake;

                        if slashing_amount > Zero::zero() {
                            T::StakingHandler::slash(&entry.staking_account_id, Some(slashing_amount));
                        }

                        T::StakingHandler::unlock(&entry.staking_account_id);

                        Self::remove_work_entry(&bounty_id, &entry_id);
                    }
                }
            }
            // Fire a judgment event.
            Self::deposit_event(RawEvent::OracleJudgmentSubmitted(bounty_id, bounty.creation_params.oracle, judgment));
        }

        ///Unlocks the stake related to a work entry
        ///After the oracle makes the judgment or the council terminates the bounty by calling terminate_bounty(...),
        ///each worker whose entry has not been judged, can unlock the totality of their stake.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::unlock_work_entrant_stake()]
        pub fn unlock_work_entrant_stake(
            origin,
            member_id: MemberId<T>,
            bounty_id: T::BountyId,
            entry_id: T::EntryId,
        ) {
            T::Membership::ensure_member_controller_account_origin(origin, member_id)?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(
                matches!(current_bounty_stage,
                    BountyStage::FailedBountyWithdrawal |
                    BountyStage::SuccessfulBountyWithdrawal ),
                Self::unexpected_bounty_stage_error(current_bounty_stage)
            );

            let entry = Self::ensure_work_entry_exists(&entry_id)?;

            Self::ensure_work_entry_ownership(&entry, &member_id)?;

            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor_manager(
                bounty.creation_params.creator.clone(),
            )?;

            //
            // == MUTATION SAFE ==
            //

            T::StakingHandler::unlock(&entry.staking_account_id);

            Self::deposit_event(
                RawEvent::WorkEntrantStakeUnlocked(
                    bounty_id,
                    entry_id,
                    entry.staking_account_id)
            );

            Self::remove_work_entry(&bounty_id, &entry_id);

            if Self::can_remove_bounty(&bounty_id, &bounty) {
                Self::remove_bounty(
                    &bounty_id,
                    &bounty,
                    &bounty_creator_manager
                );
            }
        }

        ///Withraws the state bloat bond to funder
        ///If bounty is successfully, funders must call this extrinsic to withdraw the state bloat bond,
        ///this makes the cleaning storage process more efficient.
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::withdraw_funder_state_bloat_bond_amount_by_council()
        .max(WeightInfoBounty::<T>::withdraw_funder_state_bloat_bond_amount_by_member())]
        pub fn withdraw_funder_state_bloat_bond_amount(
            origin,
            funder: BountyActor<MemberId<T>>,
            bounty_id: T::BountyId,
        ) {
            let bounty_funder_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                funder.clone(),
            )?;

            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(current_bounty_stage == BountyStage::SuccessfulBountyWithdrawal,
                Self::unexpected_bounty_stage_error(current_bounty_stage)
            );

            let funding = Self::ensure_bounty_contribution_exists(&bounty_id, &funder)?;

            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor_manager(
                bounty.creation_params.creator.clone(),
            )?;

            //
            // == MUTATION SAFE ==
            //

            bounty_funder_manager.transfer_funds_from_bounty_account(
                bounty_id,
                funding.funder_state_bloat_bond_amount);

            //Remove contribution from
            <BountyContributions<T>>::remove(&bounty_id, &funder);

            Self::deposit_event(
                RawEvent::FunderStateBloatBondWithdrawn(
                    bounty_id,
                    funder,
                    T::Balance::from(FUNDER_STATE_BLOAT_BOND_AMOUNT)));

            if Self::can_remove_bounty(&bounty_id, &bounty) {
                Self::remove_bounty(
                    &bounty_id,
                    &bounty,
                    &bounty_creator_manager
                );
            }
        }

        ///Withraws the oracle reward to oracle
        ///If bounty is successfully, Failed or Cancelled oracle must call this
        ///extrinsic to withdraw the oracle reward,
        /// # <weight>
        ///
        /// ## weight
        /// `O (1)`
        /// - db:
        ///    - `O(1)` doesn't depend on the state or parameters
        /// # </weight>
        #[weight = WeightInfoBounty::<T>::withdraw_oracle_reward_by_oracle_council()
        .max(WeightInfoBounty::<T>::withdraw_oracle_reward_by_oracle_member())]
        pub fn withdraw_oracle_reward(
            origin,
            bounty_id: T::BountyId,
        ) {
            let bounty = Self::ensure_bounty_exists(&bounty_id)?;

            let bounty_oracle_manager = BountyActorManager::<T>::ensure_bounty_actor_manager(
                origin,
                bounty.creation_params.oracle.clone(),
            )?;



            let oracle_reward = bounty.creation_params.oracle_reward;
            let current_bounty_stage = Self::get_bounty_stage(&bounty);

            ensure!(
                matches!(current_bounty_stage,
                    BountyStage::Cancelled|
                    BountyStage::FailedBountyWithdrawal |
                    BountyStage::SuccessfulBountyWithdrawal ),
                Self::unexpected_bounty_stage_error(current_bounty_stage)
            );
            ensure!(!bounty.oracle_withdrew_reward, Error::<T>::OracleRewardAlreadyWithdrawn);
            ensure!(!oracle_reward.is_zero(), Error::<T>::OracleRewardIsZero);

            let bounty_creator_manager = BountyActorManager::<T>::get_bounty_actor_manager(
                bounty.creation_params.creator.clone(),
            )?;

            //
            // == MUTATION SAFE ==
            //

            bounty_oracle_manager.transfer_funds_from_bounty_account(bounty_id, oracle_reward);

            <Bounties<T>>::mutate(bounty_id, |bounty| {
                bounty.oracle_withdrew_reward = true;
            });

            Self::deposit_event(RawEvent::BountyOracleRewardWithdrawal(
                bounty_id,
                bounty.creation_params.oracle.clone(),
                oracle_reward
            ));

            if Self::has_no_contributions_and_no_work_entries(&bounty_id) {
                Self::remove_bounty(
                    &bounty_id,
                    &bounty,
                    &bounty_creator_manager
                );
            }
        }
    }
}

impl<T: Trait> Module<T> {
    // Wrapper-function over System::block_number()
    pub(crate) fn current_block() -> T::BlockNumber {
        <frame_system::Module<T>>::block_number()
    }

    // Validates parameters for a bounty creation.
    fn ensure_create_bounty_parameters_valid(
        params: &BountyCreationParameters<T>,
    ) -> DispatchResult {
        match params.funding_type {
            FundingType::Perpetual { target } => {
                ensure!(
                    target != Zero::zero(),
                    Error::<T>::FundingAmountCannotBeZero
                );
            }
            FundingType::Limited {
                target,
                funding_period,
            } => {
                ensure!(
                    target != Zero::zero(),
                    Error::<T>::FundingAmountCannotBeZero
                );

                ensure!(
                    funding_period != Zero::zero(),
                    Error::<T>::FundingPeriodCannotBeZero
                );
            }
        }

        ensure!(
            params.entrant_stake >= T::MinWorkEntrantStake::get(),
            Error::<T>::EntrantStakeIsLessThanMininum
        );

        if let AssuranceContractType::Closed(ref member_ids) = params.contract_type {
            ensure!(
                !member_ids.is_empty(),
                Error::<T>::ClosedContractMemberListIsEmpty
            );

            ensure!(
                member_ids.len() <= T::ClosedContractSizeLimit::get().saturated_into(),
                Error::<T>::ClosedContractMemberListIsTooLarge
            );
        }

        Ok(())
    }

    // Verifies that member balance is sufficient for a bounty.
    fn check_balance_for_account(amount: BalanceOf<T>, account_id: &T::AccountId) -> bool {
        balances::Module::<T>::usable_balance(account_id) >= amount
    }
    fn ensure_creator_actor_manager(
        bounty: &Bounty<T>,
    ) -> Result<BountyActorManager<T>, DispatchError> {
        let creator = bounty.creation_params.creator.clone();
        BountyActorManager::<T>::get_bounty_actor_manager(creator)
    }
    // Transfer funds from the member account to the bounty account.
    fn transfer_funds_to_bounty_account(
        account_id: &T::AccountId,
        bounty_id: T::BountyId,
        amount: BalanceOf<T>,
    ) {
        let bounty_account_id = Self::bounty_account_id(bounty_id);

        let _ = <balances::Module<T> as Currency<T::AccountId>>::transfer(
            account_id,
            &bounty_account_id,
            amount,
            ExistenceRequirement::AllowDeath,
        );
    }

    // Transfer funds from the bounty account to the member account.
    fn transfer_funds_from_bounty_account(
        account_id: &T::AccountId,
        bounty_id: T::BountyId,
        amount: BalanceOf<T>,
    ) {
        let bounty_account_id = Self::bounty_account_id(bounty_id);

        let _ = <balances::Module<T> as Currency<T::AccountId>>::transfer(
            &bounty_account_id,
            account_id,
            amount,
            ExistenceRequirement::AllowDeath,
        );
    }

    // Verifies bounty existence and retrieves a bounty from the storage.
    fn ensure_bounty_exists(bounty_id: &T::BountyId) -> Result<Bounty<T>, DispatchError> {
        ensure!(
            <Bounties<T>>::contains_key(bounty_id),
            Error::<T>::BountyDoesntExist
        );

        let bounty = <Bounties<T>>::get(bounty_id);

        Ok(bounty)
    }
    fn ensure_bounty_contribution_exists(
        bounty_id: &T::BountyId,
        funder: &BountyActor<MemberId<T>>,
    ) -> Result<Contribution<T>, DispatchError> {
        ensure!(
            <BountyContributions<T>>::contains_key(&bounty_id, &funder),
            Error::<T>::NoBountyContributionFound,
        );

        let funding = <BountyContributions<T>>::get(&bounty_id, &funder);

        Ok(funding)
    }
    // Calculate cherry fraction to reward member for an unsuccessful bounty.
    // Cherry fraction = cherry * (member funding / total funding).
    fn get_cherry_fraction_for_member(
        bounty: &Bounty<T>,
        funding_amount: BalanceOf<T>,
    ) -> BalanceOf<T> {
        let funding_share =
            Perbill::from_rational_approximation(funding_amount, bounty.total_funding);

        // cherry share
        funding_share * bounty.creation_params.cherry
    }

    /// Remove bounty and all related info from the storage.
    fn remove_bounty(
        bounty_id: &T::BountyId,
        bounty: &Bounty<T>,
        bounty_creator_manager: &BountyActorManager<T>,
    ) {
        bounty_creator_manager.transfer_funds_from_bounty_account(
            *bounty_id,
            T::Balance::from(CREATOR_STATE_BLOAT_BOND_AMOUNT),
        );

        Self::deposit_event(RawEvent::CreatorStateBloatBondWithdrawn(
            *bounty_id,
            bounty.creation_params.creator.clone(),
            T::Balance::from(CREATOR_STATE_BLOAT_BOND_AMOUNT),
        ));

        <Bounties<T>>::remove(bounty_id);
        <BountyContributions<T>>::remove_prefix(bounty_id);
        // Slash remaining funds.
        let bounty_account_id = Self::bounty_account_id(*bounty_id);
        let all = balances::Module::<T>::usable_balance(&bounty_account_id);
        if all != Zero::zero() {
            let _ = balances::Module::<T>::slash(&bounty_account_id, all);
        }

        Self::deposit_event(RawEvent::BountyRemoved(*bounty_id));
    }

    // Verifies that the bounty has no pending fund withdrawals left.
    fn has_no_contributions_and_no_work_entries(bounty_id: &T::BountyId) -> bool {
        let has_no_contributions = !Self::contributions_exist(bounty_id);
        let has_no_work_entries = Self::bounties(bounty_id).active_work_entry_count == 0;
        // All work entrants withdrew their stakes and all funders withdrew cherry and
        // provided funds.
        has_no_contributions && has_no_work_entries
    }

    // Verifies that bounty has some contribution to withdraw.
    // Should be O(1) because of the single inner call of the next() function of the iterator.
    pub(crate) fn contributions_exist(bounty_id: &T::BountyId) -> bool {
        <BountyContributions<T>>::iter_prefix_values(bounty_id)
            .peekable()
            .peek()
            .is_some()
    }
    // Verifies that bounty has some contribution to withdraw.
    // Should be O(1) because of the single inner call of the next() function of the iterator.
    pub(crate) fn can_remove_bounty(bounty_id: &T::BountyId, bounty: &Bounty<T>) -> bool {
        (bounty.creation_params.oracle_reward.is_zero() || bounty.oracle_withdrew_reward)
            && Self::has_no_contributions_and_no_work_entries(bounty_id)
    }

    // The account ID of a bounty account. Tests require AccountID type to be at least u128.
    pub(crate) fn bounty_account_id(bounty_id: T::BountyId) -> T::AccountId {
        T::ModuleId::get().into_sub_account(bounty_id)
    }

    // Calculates bounty milestone on member funding.
    fn get_bounty_milestone_on_funding(
        target_funding_reached: bool,
        previous_milestone: BountyMilestone<T::BlockNumber>,
    ) -> BountyMilestone<T::BlockNumber> {
        let now = Self::current_block();

        if target_funding_reached {
            // Bounty target funding reached.
            BountyMilestone::BountyMaxFundingReached {
                target_funding_reached_at: now,
            }
        // No previous contributions.
        } else if let BountyMilestone::Created {
            created_at,
            has_contributions: false,
        } = previous_milestone
        {
            // The bounty has some contributions now.
            BountyMilestone::Created {
                created_at,
                has_contributions: true,
            }
        } else {
            // No changes.
            previous_milestone
        }
    }

    fn get_adjusted_contribution(
        bounty_id: &T::BountyId,
        bounty: &Bounty<T>,
        funder: &BountyActor<MemberId<T>>,
        amount: T::Balance,
    ) -> (bool, bool, T::Balance, Contribution<T>) {
        //Adds the amount to the total funds and check if the target funding  is reached
        let is_target_funding_reached =
            bounty.is_target_funding_reached(bounty.total_funding.saturating_add(amount));

        //The contribution should be saturated to the target funding,
        //in case of target funding is reached.
        let adjusted_amount = if is_target_funding_reached {
            bounty.target_funding().saturating_sub(bounty.total_funding)
        } else {
            amount
        };

        //Check if is the first time a funder is contributiong
        let (is_first_contribution, funds_so_far) =
            if <BountyContributions<T>>::contains_key(&bounty_id, &funder) {
                (
                    false,
                    Self::contribution_by_bounty_by_actor(bounty_id, &funder),
                )
            } else {
                (true, Contribution::<T>::default())
            };

        //Add adjusted_amount to the current funds
        let adjusted_contribution = funds_so_far.add_funds(adjusted_amount);

        (
            is_target_funding_reached,
            is_first_contribution,
            adjusted_amount,
            adjusted_contribution,
        )
    }

    // Validates stake on announcing the work entry.
    fn validate_entrant_stake(
        member_id: MemberId<T>,
        bounty: &Bounty<T>,
        staking_account_id: T::AccountId,
    ) -> Result<Option<RequiredStakeInfo<T>>, DispatchError> {
        let staking_balance = bounty.creation_params.entrant_stake;

        ensure!(
            T::Membership::is_member_staking_account(&member_id, &staking_account_id),
            Error::<T>::InvalidStakingAccountForMember
        );

        ensure!(
            T::StakingHandler::is_account_free_of_conflicting_stakes(&staking_account_id),
            Error::<T>::ConflictingStakes
        );

        ensure!(
            T::StakingHandler::is_enough_balance_for_stake(&staking_account_id, staking_balance),
            Error::<T>::InsufficientBalanceForStake
        );

        Ok(Some(RequiredStakeInfo {
            amount: staking_balance,
            account_id: staking_account_id,
        }))
    }

    // Verifies work entry existence and retrieves an entry from the storage.
    fn ensure_work_entry_exists(entry_id: &T::EntryId) -> Result<Entry<T>, DispatchError> {
        ensure!(
            <Entries<T>>::contains_key(entry_id),
            Error::<T>::WorkEntryDoesntExist
        );

        let entry = Self::entries(entry_id);

        Ok(entry)
    }

    // Ensures entry record ownership for a member.
    fn ensure_work_entry_ownership(
        entry: &Entry<T>,
        owner_member_id: &MemberId<T>,
    ) -> DispatchResult {
        ensure!(
            entry.member_id == *owner_member_id,
            Error::<T>::WorkEntryDoesntBelongToWorker
        );

        Ok(())
    }

    // Validates the contract type for a bounty
    fn ensure_valid_contract_type(bounty: &Bounty<T>, member_id: &MemberId<T>) -> DispatchResult {
        if let AssuranceContractType::Closed(ref valid_members) =
            bounty.creation_params.contract_type
        {
            ensure!(
                valid_members.contains(member_id),
                Error::<T>::CannotSubmitWorkToClosedContractBounty
            );
        }

        Ok(())
    }

    // Computes the stage of a bounty based on its creation parameters and the current state.
    pub(crate) fn get_bounty_stage(bounty: &Bounty<T>) -> BountyStage {
        let sc = BountyStageCalculator::<T> {
            now: Self::current_block(),
            bounty,
        };

        sc.get_bounty_stage()
    }

    // Validates oracle judgment.
    fn validate_judgment(bounty: &Bounty<T>, judgment: &OracleJudgmentOf<T>) -> DispatchResult {
        // Total judgment reward accumulator.
        let mut reward_sum_from_judgment: BalanceOf<T> = Zero::zero();

        // Validate all work entry Judgments.
        for (entry_id, work_entry_judgment) in judgment.iter() {
            let entry = Self::ensure_work_entry_exists(entry_id)?;
            //checks if member_id exists
            T::Membership::controller_account_id(entry.member_id)?;
            if let OracleWorkEntryJudgment::Winner { reward } = work_entry_judgment {
                // Check for zero reward.
                ensure!(*reward != Zero::zero(), Error::<T>::ZeroWinnerReward);
                // Check winner work submission.
                ensure!(
                    entry.work_submitted,
                    Error::<T>::WinnerShouldHasWorkSubmission
                );
                reward_sum_from_judgment += *reward;
            }
        }

        // Check for invalid total sum for successful bounty.
        if reward_sum_from_judgment != Zero::zero() {
            ensure!(
                reward_sum_from_judgment == bounty.total_funding, // 100% bounty distribution
                Error::<T>::TotalRewardShouldBeEqualToTotalFunding
            );
        }

        Ok(())
    }

    // Removes the work entry and decrements active entry count in a bounty.
    fn remove_work_entry(bounty_id: &T::BountyId, entry_id: &T::EntryId) {
        <Entries<T>>::remove(entry_id);

        // Decrement work entry counter and update bounty record.
        <Bounties<T>>::mutate(bounty_id, |bounty| {
            bounty.decrement_active_work_entry_counter();
        });
    }

    // Bounty stage validator.
    fn ensure_bounty_stage(
        actual_stage: BountyStage,
        expected_stage: BountyStage,
    ) -> DispatchResult {
        ensure!(
            actual_stage == expected_stage,
            Self::unexpected_bounty_stage_error(actual_stage)
        );

        Ok(())
    }

    // Bounty stage validator for cancel_bounty() extrinsic.
    fn ensure_bounty_stage_for_canceling(actual_stage: BountyStage) -> DispatchResult {
        ensure!(
            matches!(
                actual_stage,
                BountyStage::Funding {
                    has_contributions: false
                } | BountyStage::FundingExpired
            ),
            Self::unexpected_bounty_stage_error(actual_stage)
        );
        Ok(())
    }

    // Provides fined-grained errors for a bounty stages
    fn unexpected_bounty_stage_error(unexpected_stage: BountyStage) -> DispatchError {
        match unexpected_stage {
            BountyStage::Funding { .. } => Error::<T>::InvalidStageUnexpectedFunding.into(),
            BountyStage::FundingExpired => Error::<T>::InvalidStageUnexpectedFundingExpired.into(),
            BountyStage::Cancelled => Error::<T>::InvalidStageUnexpectedCancelled.into(),
            BountyStage::WorkSubmission => Error::<T>::InvalidStageUnexpectedWorkSubmission.into(),
            BountyStage::Judgment => Error::<T>::InvalidStageUnexpectedJudgment.into(),
            BountyStage::SuccessfulBountyWithdrawal => {
                Error::<T>::InvalidStageUnexpectedSuccessfulBountyWithdrawal.into()
            }
            BountyStage::FailedBountyWithdrawal => {
                Error::<T>::InvalidStageUnexpectedFailedBountyWithdrawal.into()
            }
        }
    }

    // Oracle judgment helper. Returns true if a Judgment contains at least one winner.
    pub(crate) fn judgment_has_winners(judgment: &OracleJudgmentOf<T>) -> bool {
        judgment.iter().any(|(_, j)| j.is_winner())
    }

    // Transfers cherry back to the bounty creator and fires an event.
    fn return_bounty_cherry_to_creator(
        bounty_id: T::BountyId,
        bounty: &Bounty<T>,
        bounty_creator_manager: &BountyActorManager<T>,
    ) {
        bounty_creator_manager
            .transfer_funds_from_bounty_account(bounty_id, bounty.creation_params.cherry);

        Self::deposit_event(RawEvent::BountyCreatorCherryWithdrawal(
            bounty_id,
            bounty.creation_params.creator.clone(),
        ));
    }

    // Calculates weight for create_bounty extrinsic.
    fn create_bounty_weight(params: &BountyCreationParameters<T>, metadata: &[u8]) -> Weight {
        let metadata_length = metadata.len().saturated_into();
        let member_list_length =
            if let AssuranceContractType::Closed(ref members) = params.contract_type {
                members.len().saturated_into()
            } else {
                1 // consider open contract member list as one.
            };

        WeightInfoBounty::<T>::create_bounty_by_member(metadata_length, member_list_length).max(
            WeightInfoBounty::<T>::create_bounty_by_council(metadata_length, member_list_length),
        )
    }

    // Calculates weight for submit_oracle_Judgment extrinsic.
    fn submit_oracle_judgment_weight(judgment_map: &OracleJudgmentOf<T>) -> Weight {
        let collection_length: u32 = judgment_map.len().saturated_into();
        let justification_length: u32 = judgment_map
            .iter()
            .map(|(_, judgment)| match judgment {
                OracleWorkEntryJudgment::Winner { .. } => 0,
                OracleWorkEntryJudgment::Rejected {
                    action_justification,
                    ..
                } => action_justification.len().saturated_into(),
            })
            .sum();
        WeightInfoBounty::<T>::submit_oracle_judgment_by_council_all_winners(collection_length)
            .max(
                WeightInfoBounty::<T>::submit_oracle_judgment_by_council_all_rejected(
                    collection_length,
                    justification_length,
                ),
            )
            .max(
                WeightInfoBounty::<T>::submit_oracle_judgment_by_member_all_winners(
                    collection_length,
                ),
            )
            .max(
                WeightInfoBounty::<T>::submit_oracle_judgment_by_member_all_rejected(
                    collection_length,
                    justification_length,
                ),
            )
    }
}
