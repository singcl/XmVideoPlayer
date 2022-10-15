// import type { PlayHistory } from '@/internal/repository/model';
import * as playHistoryService from '@/internal/service/play_history';

export async function getPlayerHistoryList() {
  const list = await playHistoryService.queryHistoryList();
  console.log('------list', list);
  return list;
}
