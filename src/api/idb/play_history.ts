import { ServerResponse, ServerTableResponse } from '@/internal/http/http';
import * as playHistoryService from '@/internal/service/play_history';
import type { SavePlayHistory, UpdatePlayHistory } from '@/internal/service/play_history';

// 获取播放列表
export async function getPlayerHistoryList() {
  const list = await playHistoryService.queryHistoryList();
  return ServerResponse.default(list);
}

// 获取播放列表 - 分页
export async function getPlayerHistoryPageList(data?: { page?: PageCamels; keyword?: string }) {
  const response = await playHistoryService.queryHistoryPageList(data);
  return ServerTableResponse.default(response, { costTime: 0, status: 1, msg: 'success' });
}

// 删除
export async function deletePlayerHistory(id: number) {
  const res = await playHistoryService.deleteHistory(id);
  return ServerResponse.default(res);
}

// 新增
export async function savePlayerHistory(data: SavePlayHistory) {
  const res = await playHistoryService.saveHistory(data);
  return ServerResponse.default(res);
}

// 修改
export async function updatePlayerHistory(data: UpdatePlayHistory) {
  const res = await playHistoryService.updateHistory(data);
  return ServerResponse.default(res);
}

// 修改
export async function getPlayerHistoryInfo(data: { id: number }) {
  const res = await playHistoryService.getHistoryInfo(data);
  return ServerResponse.default(res);
}
