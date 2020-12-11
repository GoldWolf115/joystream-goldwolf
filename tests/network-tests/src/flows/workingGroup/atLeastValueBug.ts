import { Api, WorkingGroups } from '../../Api'
import { AddWorkerOpeningFixture } from '../../fixtures/workingGroupModule'
import BN from 'bn.js'
import { assert } from 'chai'
import Debugger from 'debug'
const debug = Debugger('flow:atLeastValueBug')

// Zero at least value bug scenario
export default async function zeroAtLeastValueBug(api: Api, env: NodeJS.ProcessEnv) {
  debug('Started')
  const applicationStake: BN = new BN(env.WORKING_GROUP_APPLICATION_STAKE!)
  const roleStake: BN = new BN(env.WORKING_GROUP_ROLE_STAKE!)
  const unstakingPeriod: BN = new BN(env.STORAGE_WORKING_GROUP_UNSTAKING_PERIOD!)
  const openingActivationDelay: BN = new BN(0)

  // Pre-conditions
  // A hired lead
  const lead = await api.getGroupLead(WorkingGroups.StorageWorkingGroup)
  assert(lead)

  const addWorkerOpeningWithoutStakeFixture = new AddWorkerOpeningFixture(
    api,
    new BN(0),
    new BN(0),
    openingActivationDelay,
    unstakingPeriod,
    WorkingGroups.StorageWorkingGroup
  )
  // Add worker opening with 0 stake, expect failure
  await addWorkerOpeningWithoutStakeFixture.runner(true)

  const addWorkerOpeningWithoutUnstakingPeriodFixture = new AddWorkerOpeningFixture(
    api,
    applicationStake,
    roleStake,
    openingActivationDelay,
    new BN(0),
    WorkingGroups.StorageWorkingGroup
  )
  // Add worker opening with 0 unstaking period, expect failure
  await addWorkerOpeningWithoutUnstakingPeriodFixture.runner(true)

  // TODO: close openings
  debug('Passed')
}
