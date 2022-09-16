/* // TIPS: 也可以安装 "@types/dplayer": "^1.25.2", 类型定义
// 视频信息
interface DPVideo {
  url: string; // 视频链接
  pic?: string; // 视频封面
  // video.quality	-	见#清晰度切换
  // video.defaultQuality	-	见#清晰度切换
  // video.thumbnails	-	视频缩略图，可以使用 DPlayer-thumbnails (opens new window)生成
  type?:
    | "auto"
    | "hls"
    | "flv"
    | "dash"
    | "webtorrent"
    | "normal"
    | "customHls"; // video.type	'auto'	可选值: 'auto', 'hls', 'flv', 'dash', 'webtorrent', 'normal' 或其他自定义类型, 见#MSE 支持
  customType?: Record<string, any>; // video.customType	-	自定义类型, 见#MSE 支持
}

// 显示弹幕
interface DPDanmaku {
  // danmaku.id	required	弹幕池 id，必须唯一
  // danmaku.api	required	见#弹幕接口
  // danmaku.token	-	弹幕后端验证 token
  // danmaku.maximum	-	弹幕最大数量
  // danmaku.addition	-	额外外挂弹幕，见#bilibili 弹幕
  // danmaku.user	'DIYgod'	弹幕用户名
  // danmaku.bottom	-	弹幕距离播放器底部的距离，防止遮挡字幕，取值形如: '10px' '10%'
  // danmaku.unlimited	false	海量弹幕模式，即使重叠也展示全部弹幕，请注意播放器会记忆用户设置，用户手动设置后即失效
  // danmaku.speedRate	1	弹幕速度倍率，越大速度越快
}

// 外挂字幕
interface DPSubtitle {
  url: string; // subtitle.url	required	字幕链接
  type?: "webvtt" | "ass"; // subtitle.type	'webvtt'	字幕类型，可选值: 'webvtt', 'ass'，目前只支持 webvtt
  fontSize?: string; // // subtitle.fontSize	'20px'	字幕字号
  bottom?: string; // subtitle.bottom	'40px'	字幕距离播放器底部的距离，取值形如: '10px' '10%'
  color?: string; // subtitle.color	'#fff'	字幕颜色
}

// DPlayer 播放器配置项目
interface DPOptions {
  container: HTMLElement; //	播放器容器元素
  live?: boolean; //	false	开启直播模式, 见#直播
  autoplay?: boolean; //	false	视频自动播放
  theme?: string; //	'#b7daff'	主题色
  loop?: boolean; //	false	视频循环播放
  lang?: "en" | "zh-cn" | "zh-tw"; //	navigator.language.toLowerCase()	可选值: 'en', 'zh-cn', 'zh-tw'
  screenshot?: boolean; //	false	开启截图，如果开启，视频和视频封面需要允许跨域
  hotkey?: boolean; //	true	开启热键，支持快进、快退、音量控制、播放暂停
  airplay?: boolean; //	false	在 Safari 中开启 AirPlay
  chromecast?: boolean; //	false	启用 Chromecast
  preload?: "none" | "metadata" | "auto"; //	视频预加载，可选值: 'none', 'metadata', 'auto'
  volume?: number; //	0.7	默认音量，请注意播放器会记忆用户设置，用户手动设置音量后默认音量即失效
  playbackSpeed?: number[]; //	[0.5, 0.75, 1, 1.25, 1.5, 2]	可选的播放速率，可以设置成自定义的数组
  logo?: string; //	-	在左上角展示一个 logo，你可以通过 CSS 调整它的大小和位置
  apiBackend?: any; //	-	自定义获取和发送弹幕行为，见#直播
  preventClickToggle?: boolean; //	false	阻止点击播放器时候自动切换播放/暂停
  video: DPVideo;
  danmaku?: DPDanmaku;
  subtitle?: DPSubtitle;
  contextmenu?: any[]; // []	自定义右键菜单
  highlight?: any[]; //	[]	自定义进度条提示点
  mutex?: boolean; //true	互斥，阻止多个播放器同时播放，当前播放器播放时暂停其他播放器
}

declare module "dplayer" {
  export default class DPlayer {
    constructor(options?: DPOptions);
  }
}
 */

declare type Lang = "en" | "zh-cn" | "zh-tw";
declare type Preload = "none" | "metadata" | "auto";
declare type VideoType =
  | "auto"
  | "hls"
  | "flv"
  | "dash"
  | "webtorrent"
  | "normal";
declare type SubTitleType = "webvtt" | "ass";
declare type DirectionType = "top" | "right" | "bottom";
declare type FullScreenType = "web" | "browser";

declare enum DPlayerEvents {
  abort = "abort",
  canplay = "canplay",
  canplaythrough = "canplaythrough",
  durationchange = "durationchange",
  emptied = "emptied",
  ended = "ended",
  error = "error",
  loadeddata = "loadeddata",
  loadedmetadata = "loadedmetadata",
  loadstart = "loadstart",
  mozaudioavailable = "mozaudioavailable",
  pause = "pause",
  play = "play",
  playing = "playing",
  progress = "progress",
  ratechange = "ratechange",
  seeked = "seeked",
  seeking = "seeking",
  stalled = "stalled",
  suspend = "suspend",
  timeupdate = "timeupdate",
  volumechange = "volumechange",
  waiting = "waiting",
  screenshot = "screenshot",
  thumbnails_show = "thumbnails_show",
  thumbnails_hide = "thumbnails_hide",
  danmaku_show = "danmaku_show",
  danmaku_hide = "danmaku_hide",
  danmaku_clear = "danmaku_clear",
  danmaku_loaded = "danmaku_loaded",
  danmaku_send = "danmaku_send",
  danmaku_opacity = "danmaku_opacity",
  contextmenu_show = "contextmenu_show",
  contextmenu_hide = "contextmenu_hide",
  notice_show = "notice_show",
  notice_hide = "notice_hide",
  quality_start = "quality_start",
  quality_end = "quality_end",
  destroy = "destroy",
  resize = "resize",
  fullscreen = "fullscreen",
  fullscreen_cancel = "fullscreen_cancel",
  subtitle_show = "subtitle_show",
  subtitle_hide = "subtitle_hide",
  subtitle_change = "subtitle_change",
}

declare interface DPlayerOptions {
  container: HTMLElement | null;
  live?: boolean | undefined;
  autoplay?: boolean | undefined;
  theme?: string | undefined;
  loop?: boolean | undefined;
  lang?: Lang | string | undefined;
  screenshot?: boolean | undefined;
  hotkey?: boolean | undefined;
  preload?: Preload | undefined;
  logo?: string | undefined;
  volume?: number | undefined;
  mutex?: boolean | undefined;
  video?: DPlayerVideo | undefined;
  subtitle?: DPlayerSubTitle | undefined;
  danmaku?: DPlayerDanmaku | undefined;
  contextmenu?: DPlayerContextMenuItem[] | undefined;
  highlight?: DPlayerHighLightItem[] | undefined;
  apiBackend?: DPlayerAPIBackend | undefined;
}

declare interface DPlayerDanmakuItem {
  text: string;
  color: string;
  type: DirectionType;
}

declare interface DPlayerContextMenuItem {
  text: string;
  link?: string | undefined;
  click?: (() => void) | undefined;
}

declare interface DPlayerHighLightItem {
  text: string;
  time: number;
}

declare interface DPlayerVideoQuality {
  name: string;
  url: string;
  type?: string | undefined;
}

declare interface DPlayerVideo {
  url: string;
  pic?: string | undefined;
  thumbnails?: string | undefined;
  type?: VideoType | string | undefined;
  customType?: any;
  quality?: DPlayerVideoQuality[] | undefined;
  defaultQuality?: number | undefined;
}

declare interface DPlayerSubTitle {
  url: string;
  type?: SubTitleType | undefined;
  fontSize?: string | undefined;
  bottom?: string | undefined;
  color?: string | undefined;
}

declare interface DPlayerDanmaku {
  id: string;
  api: string;
  token?: string | undefined;
  maximum?: string | undefined;
  addition?: string[] | undefined;
  user?: string | undefined;
  bottom?: string | undefined;
  unlimited?: boolean | undefined;
}

declare interface DPlayerAPIBackend {
  read(endpoint: any, callback: () => void): void;

  send(
    endpoint: any,
    danmakuData: DPlayerDanmakuItem,
    callback: () => void
  ): void;
}

declare interface Danmaku {
  send(danmaku: DPlayerDanmakuItem, callback: () => void): void;

  draw(danmaku: DPlayerDanmakuItem): void;

  opacity(percentage: number): void;

  clear(): void;

  hide(): void;

  show(): void;
}

declare interface FullScreen {
  request(type: FullScreenType): void;

  cancel(type: FullScreenType): void;
}

declare class DPlayer {
  events: any;
  video: HTMLVideoElement;
  danmaku: Danmaku;
  fullScreen: FullScreen;

  constructor(options: DPlayerOptions);

  play(): void;

  pause(): void;

  seek(time: number): void;

  toggle(): void;

  on(event: DPlayerEvents, handler: () => void): void;

  switchVideo(video: DPlayerVideo, danmaku: DPlayerDanmaku): void;

  notice(text: string, time: number, opacity: number): void;

  switchQuality(index: number): void;

  destroy(): void;

  speed(rate: number): void;

  volume(percentage: number, nostorage: boolean, nonotice: boolean): void;
}
