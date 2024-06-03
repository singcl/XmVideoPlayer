import { DexieDb } from './../database/db';
import { XM_TABLE } from './../repository/model';
import { PlayHistory } from '@/internal/repository/model';

export type SavePlayHistory = Omit<PlayHistory, 'id' | 'created_at' | 'updated_at'>;
export interface UpdatePlayHistory extends Partial<PlayHistory> {
  id: number;
}
//
const db = DexieDb.create();
const dbTable = db[XM_TABLE.PLAY_HISTORY_TABLE];

export function queryHistoryList() {
  return db.transaction('r', [dbTable], async () => {
    const list = await dbTable.toArray();
    return list;
  });
}

export function queryHistoryPageList(params?: { page?: PageCamels; keyword?: string }) {
  const { page: { pageNo, pageSize } = { pageNo: 1, pageSize: 20 }, keyword } = params ?? {};
  return db.transaction('r', [dbTable], async () => {
    const offset = (pageNo - 1) * pageSize;

    let collection = dbTable.orderBy('created_at');
    if (keyword) {
      const reg = new RegExp(encodeURIComponent(keyword), 'i');
      collection = collection.filter((obj) => reg.test(obj.name) || reg.test(obj.url));
    }
    //
    const count = await collection.count();
    const list = await collection.reverse().offset(offset).limit(pageSize).toArray();
    return { list, total: count, pageNo, pageSize };
  });
}

export function deleteHistory(id: number) {
  return db.transaction('rw', [dbTable], async () => {
    await dbTable.delete(id);
    return null;
  });
}

export function saveHistory(data: SavePlayHistory) {
  return db.transaction('rw', [dbTable], async () => {
    const record = await dbTable.where('url').equals(data.url).first();
    if (record) return record.id;
    const res = await db
      .table<Omit<PlayHistory, 'id'>, number>(XM_TABLE.PLAY_HISTORY_TABLE)
      .add({ ...data, created_at: new Date().toISOString() });
    return res;
  });
}

export function updateHistory(data: UpdatePlayHistory) {
  return db.transaction('rw', [dbTable], async () => {
    const record = await dbTable.where('id').equals(data.id).modify(data);
    return record;
  });
}

export function getHistoryInfo(data: { id: number }) {
  return db.transaction('r', [dbTable], async () => {
    const record = await dbTable.where('id').equals(Number(data.id)).first();
    return record;
  });
}
