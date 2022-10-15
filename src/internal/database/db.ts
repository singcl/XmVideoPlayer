import Dexie from 'dexie';
import type { Table } from 'dexie';
import type { PlayHistory } from '@/internal/repository/model';
import { XM_DB, XM_TABLE } from '@/internal/repository/model';
import { historyListDefault } from './config';

class MySubClassedDexie extends Dexie {
  // 'friends' is added by dexie when declaring the stores()
  // We just tell the typing system this is the case
  [XM_TABLE.PLAY_HISTORY_TABLE]!: Table<PlayHistory>;
  constructor() {
    super(XM_DB['PLAYER_DB']);
    this.version(1)
      .stores({
        [XM_TABLE.PLAY_HISTORY_TABLE]: '++id, &name, url', // Primary key and indexed props
      })
      .upgrade((trans) => {
        return trans.table<PlayHistory>(XM_TABLE.PLAY_HISTORY_TABLE).bulkAdd(historyListDefault);
      });
  }
}

export const db = new MySubClassedDexie();
