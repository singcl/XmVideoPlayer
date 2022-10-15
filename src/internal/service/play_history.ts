import { db } from './../database/db';
import { XM_TABLE } from './../repository/model';

export function queryHistoryList() {
  return db.transaction('r', [db[XM_TABLE.PLAY_HISTORY_TABLE]], async () => {
    const list = await db[XM_TABLE.PLAY_HISTORY_TABLE].toArray();
    return list;
  });
}
