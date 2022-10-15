#### dplayer.play_history

播放历史记录表

| 序号 |     名称     |        描述         |     类型     | 键  | 为空 |                     额外                      |      默认值       |
| :--: | :----------: | :-----------------: | :----------: | :-: | :--: | :-------------------------------------------: | :---------------: |
|  1   |      id      |        主键         | int unsigned | PRI |  NO  |                auto_increment                 |                   |
|  2   |     name     |      资源名称       | varchar(32)  | UNI |  NO  |                                               |                   |
|  3   |     url      |      资源链接       | varchar(100) |     |  NO  |                                               |                   |
|  6   |   is_used    | 是否启用 1:是 -1:否 |  tinyint(1)  |     |  NO  |                                               |         1         |
|  7   |  is_deleted  | 是否删除 1:是 -1:否 |  tinyint(1)  |     |  NO  |                                               |        -1         |
|  8   |  created_at  |      创建时间       |  timestamp   |     |  NO  |               DEFAULT_GENERATED               | CURRENT_TIMESTAMP |
|  9   | created_user |       创建人        | varchar(60)  |     |  NO  |                                               |                   |
|  10  |  updated_at  |      更新时间       |  timestamp   |     |  NO  | DEFAULT_GENERATED on update CURRENT_TIMESTAMP | CURRENT_TIMESTAMP |
|  11  | updated_user |       更新人        | varchar(60)  |     |  NO  |                                               |                   |
