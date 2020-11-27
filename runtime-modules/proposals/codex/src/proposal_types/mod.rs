#![warn(missing_docs)]

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

use crate::ElectionParameters;
use common::working_group::WorkingGroup;

/// Encodes proposal using its details information.
pub trait ProposalEncoder<T: crate::Trait> {
    /// Encodes proposal using its details information.
    fn encode_proposal(proposal_details: ProposalDetailsOf<T>) -> Vec<u8>;
}

/// _ProposalDetails_ alias for type simplification
pub type ProposalDetailsOf<T> = ProposalDetails<
    crate::BalanceOfMint<T>,
    crate::BalanceOfGovernanceCurrency<T>,
    <T as frame_system::Trait>::BlockNumber,
    <T as frame_system::Trait>::AccountId,
    working_group::OpeningId<T>,
    working_group::ApplicationId<T>,
    crate::BalanceOf<T>,
    working_group::WorkerId<T>,
    crate::MemberId<T>,
>;

/// Proposal details provide voters the information required for the perceived voting.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub enum ProposalDetails<
    MintedBalance,
    CurrencyBalance,
    BlockNumber,
    AccountId,
    OpeningId,
    ApplicationId,
    StakeBalance,
    WorkerId,
    MemberId,
> {
    /// The text of the `text` proposal
    Text(Vec<u8>),

    /// The wasm code for the `runtime upgrade` proposal
    RuntimeUpgrade(Vec<u8>),

    /// ********** Deprecated.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// Election parameters for the `set election parameters` proposal
    SetElectionParameters(ElectionParameters<CurrencyBalance, BlockNumber>),

    /// Balance and destination account for the `spending` proposal
    Spending(MintedBalance, AccountId),

    /// ********** Deprecated during the Babylon release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// New leader memberId and account_id for the `set lead` proposal
    DeprecatedSetLead(Option<(MemberId, AccountId)>),

    /// ********** Deprecated during the Babylon release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// Balance for the `set content working group mint capacity` proposal
    DeprecatedSetContentWorkingGroupMintCapacity(MintedBalance),

    /// ********** Deprecated during the Nicaea release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// AccountId for the `evict storage provider` proposal
    DeprecatedEvictStorageProvider(AccountId),

    /// Validator count for the `set validator count` proposal
    SetValidatorCount(u32),

    /// ********** Deprecated during the Nicaea release.
    /// It is kept only for backward compatibility in the Pioneer. **********
    /// Role parameters for the `set storage role parameters` proposal
    DeprecatedSetStorageRoleParameters(RoleParameters<CurrencyBalance, BlockNumber>),

    /// Add opening for the working group leader position.
    AddWorkingGroupLeaderOpening(AddOpeningParameters<BlockNumber, CurrencyBalance>),

    /// Begin review applications for the working group leader position.
    BeginReviewWorkingGroupLeaderApplications(OpeningId, WorkingGroup),

    /// Fill opening for the working group leader position.
    FillWorkingGroupLeaderOpening(
        FillOpeningParameters<BlockNumber, MintedBalance, OpeningId, ApplicationId>,
    ),

    /// Set working group mint capacity.
    SetWorkingGroupMintCapacity(MintedBalance, WorkingGroup),

    /// Decrease the working group leader stake.
    DecreaseWorkingGroupLeaderStake(WorkerId, StakeBalance, WorkingGroup),

    /// Slash the working group leader stake.
    SlashWorkingGroupLeaderStake(WorkerId, StakeBalance, WorkingGroup),

    /// Set working group leader reward balance.
    SetWorkingGroupLeaderReward(WorkerId, MintedBalance, WorkingGroup),

    /// Fire the working group leader with possible slashing.
    TerminateWorkingGroupLeaderRole(TerminateRoleParameters<WorkerId>),

    /// Amend constitution.
    AmendConstitution(Vec<u8>),
}

impl<
        MintedBalance,
        CurrencyBalance,
        BlockNumber,
        AccountId,
        OpeningId,
        ApplicationId,
        StakeBalance,
        WorkerId,
        MemberId,
    > Default
    for ProposalDetails<
        MintedBalance,
        CurrencyBalance,
        BlockNumber,
        AccountId,
        OpeningId,
        ApplicationId,
        StakeBalance,
        WorkerId,
        MemberId,
    >
{
    fn default() -> Self {
        ProposalDetails::Text(b"invalid proposal details".to_vec())
    }
}

/// Parameters for the 'terminate the leader position' proposal.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct TerminateRoleParameters<WorkerId> {
    /// Leader worker id to fire.
    pub worker_id: WorkerId,

    /// Terminate role rationale.
    pub rationale: Vec<u8>,

    /// Slash the leader stake on terminating.
    pub slash: bool,

    /// Defines working group with the open position.
    pub working_group: WorkingGroup,
}

/// Parameters for the 'fill opening for the leader position' proposal.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct FillOpeningParameters<BlockNumber, Balance, OpeningId, ApplicationId> {
    /// Finalizing opening id.
    pub opening_id: OpeningId,

    /// Id of the selected application.
    pub successful_application_id: ApplicationId,

    /// Position reward policy.
    pub reward_policy: Option<working_group::RewardPolicy<Balance, BlockNumber>>,

    /// Defines working group with the open position.
    pub working_group: WorkingGroup,
}

/// Parameters for the 'add opening for the leader position' proposal.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Debug)]
pub struct AddOpeningParameters<BlockNumber, Balance> {
    /// Activate opening at block.
    pub activate_at: hiring::ActivateOpeningAt<BlockNumber>,

    /// Opening conditions.
    pub commitment: working_group::OpeningPolicyCommitment<BlockNumber, Balance>,

    /// Opening description.
    pub human_readable_text: Vec<u8>,

    /// Defines working group with the open position.
    pub working_group: WorkingGroup,
}

/// ********** Deprecated during the Nicaea release.
/// It is kept only for backward compatibility in the Pioneer. **********
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, Debug)]
pub struct RoleParameters<Balance, BlockNumber> {
    /// Minimum balance required to stake to enter a role.
    pub min_stake: Balance,

    /// Minimum actors to maintain - if role is unstaking
    /// and remaining actors would be less that this value - prevent or punish for unstaking.
    pub min_actors: u32,

    /// The maximum number of spots available to fill for a role.
    pub max_actors: u32,

    /// Fixed amount of tokens paid to actors' primary account.
    pub reward: Balance,

    /// Payouts are made at this block interval.
    pub reward_period: BlockNumber,

    /// Minimum amount of time before being able to unstake.
    pub bonding_period: BlockNumber,

    /// How long tokens remain locked for after unstaking.
    pub unbonding_period: BlockNumber,

    /// Minimum period required to be in service. unbonding before this time is highly penalized
    pub min_service_period: BlockNumber,

    /// "Startup" time allowed for roles that need to sync their infrastructure
    /// with other providers before they are considered in service and punishable for
    /// not delivering required level of service.
    pub startup_grace_period: BlockNumber,

    /// Small fee burned to make a request to enter role.
    pub entry_request_fee: Balance,
}
