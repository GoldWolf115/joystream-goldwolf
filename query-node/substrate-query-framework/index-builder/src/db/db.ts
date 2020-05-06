import { Connection, EntityManager, FindOneOptions, DeepPartial } from 'typeorm';
import { SavedEntityEvent } from '.';
import { QueryEvent } from '..';
import * as helper from './helper';

/**
 * Database access object based on typeorm. Use typeorm database connection to run get/save/remove
 * methods on entities.
 * Generic get/save/remove methods are generic. Entitites registered to the typeorm connection
 * are valid entities for get/save/remove methods.
 *
 * @constructor(connection: typeorm.Connection)
 *
 */
export default class DB {
  private readonly _connection: Connection;

  constructor(connection: Connection) {
    this._connection = connection;
  }

  /**
   * Save given entity instance, if entity is exists then just update
   * @param entity
   */
  async save<T>(entity: DeepPartial<T>, event: QueryEvent) {
    entity = helper.fillRequiredWarthogFields(entity);

    const eventHistory = await SavedEntityEvent.updateOrCreate(event);

    await this._connection.manager.transaction(async (manager: EntityManager) => {
      await manager.save(entity);
      await manager.save(eventHistory);
    });
  }

  // Find entity that match find options
  async get<T>(entity: { new (...args: any[]): T }, options: FindOneOptions<T>) {
    return await this._connection.getRepository(entity).findOne(options);
  }
}
