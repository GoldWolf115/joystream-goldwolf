use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_system::RawOrigin;

use crate::tests::fixtures::{
    setup_members, AddOpeningFixture, ApplyOnOpeningFixture, FillOpeningFixture, HireLeadFixture,
};
use crate::tests::mock::TestWorkingGroup;
use crate::types::StakeParameters;
use crate::{OpeningType, RewardPolicy, StakePolicy};

#[derive(Clone)]
struct HiringWorkflowApplication {
    stake_parameters: Option<StakeParameters<u64, u64>>,
    worker_handle: Vec<u8>,
    origin: RawOrigin<u64>,
    member_id: u64,
}

pub struct HiringWorkflow {
    opening_type: OpeningType,
    expected_result: DispatchResult,
    stake_policy: Option<StakePolicy<u64, u64>>,
    reward_policy: Option<RewardPolicy<u64>>,
    applications: Vec<HiringWorkflowApplication>,
    setup_environment: bool,
}

impl Default for HiringWorkflow {
    fn default() -> Self {
        Self {
            opening_type: OpeningType::Regular,
            expected_result: Ok(()),
            stake_policy: None,
            reward_policy: None,
            applications: Vec::new(),
            setup_environment: true,
        }
    }
}

impl HiringWorkflow {
    pub fn with_stake_policy(self, stake_policy: Option<StakePolicy<u64, u64>>) -> Self {
        Self {
            stake_policy,
            ..self
        }
    }

    pub fn with_reward_policy(self, reward_policy: Option<RewardPolicy<u64>>) -> Self {
        Self {
            reward_policy,
            ..self
        }
    }

    pub fn expect(self, result: DispatchResult) -> Self {
        Self {
            expected_result: result,
            ..self
        }
    }

    pub fn with_setup_environment(self, setup_environment: bool) -> Self {
        Self {
            setup_environment,
            ..self
        }
    }

    pub fn with_opening_type(self, opening_type: OpeningType) -> Self {
        Self {
            opening_type,
            ..self
        }
    }

    pub fn add_default_application(self) -> Self {
        let worker_handle = b"default".to_vec();

        self.add_application(worker_handle)
    }

    pub fn add_application(self, worker_handle: Vec<u8>) -> Self {
        self.add_application_full(worker_handle, RawOrigin::Signed(1), 1, Some(1))
    }

    pub fn add_application_full(
        self,
        worker_handle: Vec<u8>,
        origin: RawOrigin<u64>,
        member_id: u64,
        staking_account_id: Option<u64>,
    ) -> Self {
        let stake_parameters = staking_account_id.map(|staking_account_id| StakeParameters {
            stake: self
                .stake_policy
                .clone()
                .map(|policy| policy.stake_amount)
                .unwrap_or_default(),
            staking_account_id,
        });

        let mut applications = self.applications;
        applications.push(HiringWorkflowApplication {
            worker_handle,
            origin,
            member_id,
            stake_parameters,
        });

        Self {
            applications,
            ..self
        }
    }

    fn setup_environment(&self) {
        if matches!(self.opening_type, OpeningType::Regular) {
            HireLeadFixture::default().hire_lead();
        } else {
            setup_members(6);
        }
    }

    pub fn execute(&self) -> Option<u64> {
        if self.setup_environment {
            self.setup_environment()
        }

        let result = self.fill_worker_position();

        let check_result = result.clone().map(|_| ());

        assert_eq!(check_result, self.expected_result);

        result.ok()
    }

    fn fill_worker_position(&self) -> Result<u64, DispatchError> {
        let origin = match self.opening_type {
            OpeningType::Leader => RawOrigin::Root,
            OpeningType::Regular => {
                let leader_worker_id = TestWorkingGroup::current_lead().unwrap();
                let leader = TestWorkingGroup::worker_by_id(leader_worker_id);
                let lead_account_id = leader.role_account_id;

                RawOrigin::Signed(lead_account_id)
            }
        };

        // create the opening
        let add_worker_opening_fixture = AddOpeningFixture::default()
            .with_stake_policy(self.stake_policy.clone())
            .with_reward_policy(self.reward_policy.clone())
            .with_opening_type(self.opening_type)
            .with_origin(origin.clone());

        let opening_id = add_worker_opening_fixture.call()?;

        // Fill applications.
        let mut application_ids = Vec::new();
        for application in self.applications.clone() {
            let apply_on_worker_opening_fixture =
                ApplyOnOpeningFixture::default_for_opening_id(opening_id)
                    .with_stake_parameters(application.stake_parameters)
                    .with_text(application.worker_handle)
                    .with_origin(application.origin, application.member_id);

            let application_id = apply_on_worker_opening_fixture.call()?;
            application_ids.push(application_id);
        }

        // fill opening
        let fill_opening_fixture = FillOpeningFixture::default_for_ids(opening_id, application_ids)
            .with_origin(origin.clone());

        let worker_id = fill_opening_fixture.call()?;

        Ok(worker_id)
    }
}
