import Dexie from 'dexie';
import type { Table } from 'dexie';
import { historyListDefault } from './config';

export interface PlayHistory {
  id?: number;
  url: string;
  name: string;
}

export interface InitialTable {
  id?: number;
  name: string;
  initial: boolean;
}

export enum XM_DB {
  PLAYER_DB = 'player',
}

export enum XM_TABLE {
  INITIAL_TABLE = 'initial',
  PLAY_HISTORY_TABLE = 'play_history',
}

class MySubClassedDexie extends Dexie {
  // 'friends' is added by dexie when declaring the stores()
  // We just tell the typing system this is the case
  [XM_TABLE.PLAY_HISTORY_TABLE]!: Table<PlayHistory>;
  [XM_TABLE.INITIAL_TABLE]!: Table<InitialTable>;
  constructor() {
    super(XM_DB['PLAYER_DB']);
    this.version(1).stores({
      [XM_TABLE.PLAY_HISTORY_TABLE]: '++id, &name, url', // Primary key and indexed props
      [XM_TABLE.INITIAL_TABLE]: '++id, &name, initialized', // Primary key and indexed props
    });
  }
  async initializeHistory() {
    const initTable: Table<InitialTable> = db.table(XM_TABLE.INITIAL_TABLE);
    // 插入成功则表明是第一次插入
    await initTable.add({ name: XM_TABLE.PLAY_HISTORY_TABLE, initial: true });
    const historyTable: Table<PlayHistory> = db.table(XM_TABLE.PLAY_HISTORY_TABLE);
    await historyTable.bulkAdd(historyListDefault);
  }
}

export const db = new MySubClassedDexie();
