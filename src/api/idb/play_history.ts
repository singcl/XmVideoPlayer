import { ServerResponse } from '@/internal/http/http';
import * as playHistoryService from '@/internal/service/play_history';

// 获取播放列表
export async function getPlayerHistoryList() {
  const list = await playHistoryService.queryHistoryList();
  return ServerResponse.default(list);
}

// 删除
export async function deletePlayerHistory(id: number) {
  const res = await playHistoryService.deleteHistory(id);
  return ServerResponse.default(res);
}
