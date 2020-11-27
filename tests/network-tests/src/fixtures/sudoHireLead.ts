import { Fixture } from '../Fixture'
import {
  SudoAddLeaderOpeningFixture,
  ApplyForOpeningFixture,
  SudoBeginLeaderApplicationReviewFixture,
  SudoFillLeaderOpeningFixture,
} from './workingGroupModule'
import { BuyMembershipHappyCaseFixture } from './membershipModule'
import { Api, WorkingGroups } from '../Api'
import { OpeningId } from '@joystream/types/hiring'
import { PaidTermId } from '@joystream/types/members'
import BN from 'bn.js'
import { assert } from 'chai'

export class SudoHireLeadFixture implements Fixture {
  private api: Api
  private leadAccount: string
  private paidTerms: PaidTermId
  private applicationStake: BN
  private roleStake: BN
  private openingActivationDelay: BN
  private rewardInterval: BN
  private firstRewardInterval: BN
  private payoutAmount: BN
  private workingGroup: WorkingGroups

  constructor(
    api: Api,
    leadAccount: string,
    paidTerms: PaidTermId,
    applicationStake: BN,
    roleStake: BN,
    openingActivationDelay: BN,
    rewardInterval: BN,
    firstRewardInterval: BN,
    payoutAmount: BN,
    workingGroup: WorkingGroups
  ) {
    this.api = api
    this.leadAccount = leadAccount
    this.paidTerms = paidTerms
    this.applicationStake = applicationStake
    this.roleStake = roleStake
    this.openingActivationDelay = openingActivationDelay
    this.rewardInterval = rewardInterval
    this.firstRewardInterval = firstRewardInterval
    this.payoutAmount = payoutAmount
    this.workingGroup = workingGroup
  }

  public async runner(expectFailure: boolean): Promise<void> {
    const leaderHappyCaseFixture: BuyMembershipHappyCaseFixture = new BuyMembershipHappyCaseFixture(
      this.api,
      [this.leadAccount],
      this.paidTerms
    )
    // Buying membership for leader account
    await leaderHappyCaseFixture.runner(false)

    const addLeaderOpeningFixture: SudoAddLeaderOpeningFixture = new SudoAddLeaderOpeningFixture(
      this.api,
      this.applicationStake,
      this.roleStake,
      this.openingActivationDelay,
      this.workingGroup
    )
    // Add lead opening
    await addLeaderOpeningFixture.runner(false)

    const applyForLeaderOpeningFixture = new ApplyForOpeningFixture(
      this.api,
      [this.leadAccount],
      this.applicationStake,
      this.roleStake,
      addLeaderOpeningFixture.getCreatedOpeningId() as OpeningId,
      this.workingGroup
    )
    await applyForLeaderOpeningFixture.runner(false)

    assert(applyForLeaderOpeningFixture.getApplicationIds().length === 1)

    const beginLeaderApplicationReviewFixture = new SudoBeginLeaderApplicationReviewFixture(
      this.api,
      addLeaderOpeningFixture.getCreatedOpeningId() as OpeningId,
      this.workingGroup
    )
    await beginLeaderApplicationReviewFixture.runner(false)

    const fillLeaderOpeningFixture = new SudoFillLeaderOpeningFixture(
      this.api,
      applyForLeaderOpeningFixture.getApplicationIds()[0],
      addLeaderOpeningFixture.getCreatedOpeningId() as OpeningId,
      this.firstRewardInterval,
      this.rewardInterval,
      this.payoutAmount,
      this.workingGroup
    )
    await fillLeaderOpeningFixture.runner(false)
  }
}
