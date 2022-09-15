// 视频信息
interface DPVideo {
  url: string; // 视频链接
  pic?: string; // 视频封面
  // video.quality	-	见#清晰度切换
  // video.defaultQuality	-	见#清晰度切换
  // video.thumbnails	-	视频缩略图，可以使用 DPlayer-thumbnails (opens new window)生成
  // video.type	'auto'	可选值: 'auto', 'hls', 'flv', 'dash', 'webtorrent', 'normal' 或其他自定义类型, 见#MSE 支持
  // video.customType	-	自定义类型, 见#MSE 支持
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
