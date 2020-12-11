import { Api, WorkingGroups } from '../../Api'
import {
  AddWorkerOpeningFixture,
  ApplyForOpeningFixture,
  BeginApplicationReviewFixture,
  FillOpeningFixture,
  IncreaseStakeFixture,
  UpdateRewardAccountFixture,
} from '../../fixtures/workingGroupModule'
import BN from 'bn.js'
import { OpeningId } from '@joystream/types/hiring'
import { BuyMembershipHappyCaseFixture } from '../../fixtures/membershipModule'
import { assert } from 'chai'

// Manage worker as worker
export default async function manageWorkerAsWorker(api: Api, env: NodeJS.ProcessEnv, group: WorkingGroups) {
  const applicationStake: BN = new BN(env.WORKING_GROUP_APPLICATION_STAKE!)
  const roleStake: BN = new BN(env.WORKING_GROUP_ROLE_STAKE!)
  const firstRewardInterval: BN = new BN(env.LONG_REWARD_INTERVAL!)
  const rewardInterval: BN = new BN(env.LONG_REWARD_INTERVAL!)
  const payoutAmount: BN = new BN(env.PAYOUT_AMOUNT!)
  const unstakingPeriod: BN = new BN(env.STORAGE_WORKING_GROUP_UNSTAKING_PERIOD!)
  const openingActivationDelay: BN = new BN(0)
  const paidTerms = api.createPaidTermId(new BN(+env.MEMBERSHIP_PAID_TERMS!))

  const lead = await api.getGroupLead(group)
  assert(lead)

  const newMembers = api.createKeyPairs(1).map((key) => key.address)

  const memberSetFixture = new BuyMembershipHappyCaseFixture(api, newMembers, paidTerms)
  // Recreating set of members
  await memberSetFixture.runner(false)
  const applicant = newMembers[0]

  const addWorkerOpeningFixture = new AddWorkerOpeningFixture(
    api,
    applicationStake,
    roleStake,
    openingActivationDelay,
    unstakingPeriod,
    group
  )
  // Add worker opening
  await addWorkerOpeningFixture.runner(false)

  // First apply for worker opening
  const applyForWorkerOpeningFixture = new ApplyForOpeningFixture(
    api,
    [applicant],
    applicationStake,
    roleStake,
    addWorkerOpeningFixture.getCreatedOpeningId() as OpeningId,
    group
  )
  await applyForWorkerOpeningFixture.runner(false)
  const applicationIdToHire = applyForWorkerOpeningFixture.getApplicationIds()[0]

  // Begin application review
  const beginApplicationReviewFixture = new BeginApplicationReviewFixture(
    api,
    addWorkerOpeningFixture.getCreatedOpeningId() as OpeningId,
    group
  )
  await beginApplicationReviewFixture.runner(false)

  // Fill worker opening
  const fillOpeningFixture = new FillOpeningFixture(
    api,
    [applicationIdToHire],
    addWorkerOpeningFixture.getCreatedOpeningId() as OpeningId,
    firstRewardInterval,
    rewardInterval,
    payoutAmount,
    group
  )
  await fillOpeningFixture.runner(false)
  const workerId = fillOpeningFixture.getWorkerIds()[0]
  const increaseStakeFixture: IncreaseStakeFixture = new IncreaseStakeFixture(api, workerId, group)
  // Increase worker stake
  await increaseStakeFixture.runner(false)

  const updateRewardAccountFixture: UpdateRewardAccountFixture = new UpdateRewardAccountFixture(api, workerId, group)
  // Update reward account
  await updateRewardAccountFixture.runner(false)

  const updateRoleAccountFixture: UpdateRewardAccountFixture = new UpdateRewardAccountFixture(api, workerId, group)
  // Update role account
  await updateRoleAccountFixture.runner(false)
}
