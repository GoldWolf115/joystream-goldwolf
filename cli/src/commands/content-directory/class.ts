import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import chalk from 'chalk'
import { displayCollapsedRow, displayHeader, displayTable } from '../../helpers/display'

export default class ClassCommand extends ContentDirectoryCommandBase {
  static description = 'Show Class details by id or name.'
  static args = [
    {
      name: 'className',
      required: true,
      description: 'Name or ID of the Class',
    },
  ]

  async run() {
    const { className } = this.parse(ClassCommand).args
    const [id, aClass] = await this.classEntryByNameOrId(className)
    const permissions = aClass.class_permissions
    const maintainers = permissions.maintainers.toArray()

    displayCollapsedRow({
      'Name': aClass.name.toString(),
      'ID': id.toString(),
      'Any member': permissions.any_member.toString(),
      'Entity creation blocked': permissions.entity_creation_blocked.toString(),
      'All property values locked': permissions.all_entity_property_values_locked.toString(),
      'Number of entities': aClass.current_number_of_entities.toNumber(),
      'Max. number of entities': aClass.maximum_entities_count.toNumber(),
      'Default entity creation voucher max.': aClass.default_entity_creation_voucher_upper_bound.toNumber(),
    })

    displayHeader(`Maintainers`)
    this.log(
      maintainers.length ? maintainers.map((groupId) => chalk.white(`Group ${groupId.toString()}`)).join(', ') : 'NONE'
    )

    displayHeader(`Properties`)
    if (aClass.properties.length) {
      displayTable(
        aClass.properties.map((p, i) => ({
          'Index': i,
          'Name': p.name.toString(),
          'Type': JSON.stringify(p.property_type.toJSON()),
          'Required': p.required.toString(),
          'Unique': p.unique.toString(),
          'Controller lock': p.locking_policy.is_locked_from_controller.toString(),
          'Maintainer lock': p.locking_policy.is_locked_from_maintainer.toString(),
        })),
        3
      )
    } else {
      this.log('NONE')
    }
  }
}
