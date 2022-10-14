import type { Table } from 'dexie';
import { db, XM_TABLE } from './db';
import type { InitialTable, PlayHistory } from './db';

//
export async function initial() {
  const initTB: Table<InitialTable> = db.table(XM_TABLE.INITIAL_TABLE);
  // 插入成功则表明是第一次插入
  await initTB.add({ name: XM_TABLE.PLAY_HISTORY_TABLE, initial: true });
  const historyTB: Table<PlayHistory> = db.table(XM_TABLE.PLAY_HISTORY_TABLE);
  await historyTB.bulkAdd([
    { name: '毒液[MPEG-DASH]', url: 'https://storage.googleapis.com/wvmedia/clear/h264/tears/tears.mpd' },
    {
      name: '西瓜视频[FLV]',
      url: 'https://sf1-hscdn-tos.pstatp.com/obj/media-fe/xgplayer_doc_video/flv/xgplayer-demo-360p.flv',
    },
  ]);
}
