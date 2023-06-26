import { flags } from '@oclif/command'
import { updateStorageBucketsForBag } from '../../services/runtime/extrinsics'
import LeaderCommandBase from '../../command-base/LeaderCommandBase'
import logger from '../../services/logger'
import ExitCodes from '../../command-base/ExitCodes'
import _ from 'lodash'
import { CLIError } from '@oclif/errors'

// Custom 'integer array' oclif flag.
const integerArrFlags = {
  integerArr: flags.build({
    parse: (value: string) => {
      const arr: number[] = value.split(',').map((v) => {
        if (!/^-?\d+$/.test(v)) {
          throw new CLIError(`Expected comma-separated integers, but received: ${value}`, {
            exit: ExitCodes.InvalidIntegerArray,
          })
        }
        return parseInt(v)
      })
      return arr
    },
  }),
}

/**
 * CLI command:
 * Updates bags-to-buckets relationships.
 *
 * @remarks
 * Storage working group leader command. Requires storage WG leader priviliges.
 * Shell command: "leader:update-bag"
 */
export default class LeaderUpdateBag extends LeaderCommandBase {
  static description = 'Add/remove a storage bucket from a bag (adds by default).'

  static flags = {
    add: integerArrFlags.integerArr({
      char: 'a',
      description: 'ID/s of a bucket/s to add to bag',
      default: [],
    }),
    remove: integerArrFlags.integerArr({
      char: 'r',
      description: 'ID/s of a bucket/s to remove from bag',
      default: [],
    }),
    bagId: LeaderCommandBase.extraFlags.bagId({
      char: 'i',
      required: true,
    }),
    ...LeaderCommandBase.flags,
  }

  async run(): Promise<void> {
    const { flags } = this.parse(LeaderUpdateBag)

    logger.info('Updating the bag...')
    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    if (_.isEmpty(flags.add) && _.isEmpty(flags.remove)) {
      logger.error('No bucket ID provided.')
      this.exit(ExitCodes.InvalidParameters)
    }

    const account = this.getAccount()
    const api = await this.getApi()

    const success = await updateStorageBucketsForBag(api, flags.bagId, account, flags.add, flags.remove)

    this.exitAfterRuntimeCall(success)
  }
}
