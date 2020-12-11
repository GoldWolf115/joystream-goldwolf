import { Api } from '../../Api'
import {
  BuyMembershipHappyCaseFixture,
  BuyMembershipWithInsufficienFundsFixture,
} from '../../fixtures/membershipModule'
import { PaidTermId } from '@joystream/types/members'
import BN from 'bn.js'
import Debugger from 'debug'

export default async function membershipCreation(api: Api, env: NodeJS.ProcessEnv) {
  const debug = Debugger('flow:memberships')
  debug('started')

  const N: number = +env.MEMBERSHIP_CREATION_N!
  const nAccounts = api.createKeyPairs(N).map((key) => key.address)
  const aAccount = api.createKeyPairs(1)[0].address
  const paidTerms: PaidTermId = api.createPaidTermId(new BN(+env.MEMBERSHIP_PAID_TERMS!))

  const happyCaseFixture = new BuyMembershipHappyCaseFixture(api, nAccounts, paidTerms)
  // Buy membeship is accepted with sufficient funds
  await happyCaseFixture.runner(false)

  const insufficientFundsFixture: BuyMembershipWithInsufficienFundsFixture = new BuyMembershipWithInsufficienFundsFixture(
    api,
    aAccount,
    paidTerms
  )
  // Account A can not buy the membership with insufficient funds
  await insufficientFundsFixture.runner(false)

  const buyMembershipAfterAccountTopUp = new BuyMembershipHappyCaseFixture(api, [aAccount], paidTerms)

  // Account A was able to buy the membership with sufficient funds
  await buyMembershipAfterAccountTopUp.runner(false)
  debug('finished')
}
