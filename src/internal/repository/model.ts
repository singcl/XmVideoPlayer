export enum XM_DB {
  PLAYER_DB = 'player',
}
export enum XM_TABLE {
  PLAY_HISTORY_TABLE = 'play_history',
}
export interface PlayHistory {
  id: number;
  url: string;
  name: string;
  created_at: string;
  updated_at?: string;
}
