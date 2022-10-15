import { ServerResponse } from '@/internal/http/http';
import * as playHistoryService from '@/internal/service/play_history';

export async function getPlayerHistoryList() {
  const list = await playHistoryService.queryHistoryList();
  return ServerResponse.default(list);
}
