import { db } from './../database/db';
import { XM_TABLE } from './../repository/model';
import { PlayHistory } from '@/internal/repository/model';

export type SavePlayHistory = Omit<PlayHistory, 'id'>;
export interface UpdatePlayHistory extends Partial<PlayHistory> {
  id: number;
}

export function queryHistoryList() {
  return db.transaction('r', [db[XM_TABLE.PLAY_HISTORY_TABLE]], async () => {
    const list = await db[XM_TABLE.PLAY_HISTORY_TABLE].toArray();
    return list;
  });
}

export function deleteHistory(id: number) {
  return db.transaction('rw', [db[XM_TABLE.PLAY_HISTORY_TABLE]], async () => {
    await db[XM_TABLE.PLAY_HISTORY_TABLE].delete(id);
    return null;
  });
}

export function saveHistory(data: SavePlayHistory) {
  return db.transaction('rw', [db[XM_TABLE.PLAY_HISTORY_TABLE]], async () => {
    const record = await db[XM_TABLE.PLAY_HISTORY_TABLE].where('url').equals(data.url).first();
    if (record) return record.id;
    const res = await db.table<SavePlayHistory, number>(XM_TABLE.PLAY_HISTORY_TABLE).add(data);
    return res;
  });
}

export function updateHistory(data: UpdatePlayHistory) {
  return db.transaction('rw', [db[XM_TABLE.PLAY_HISTORY_TABLE]], async () => {
    const record = await db[XM_TABLE.PLAY_HISTORY_TABLE].where('id').equals(data.id).modify(data);
    return record;
  });
}
