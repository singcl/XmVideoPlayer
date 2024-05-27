![ci](https://img.shields.io/github/actions/workflow/status/singcl/XmVideoPlayer/main.yml?branch=master&?label=build&logo=github)
[![publish](https://github.com/singcl/XmVideoPlayer/actions/workflows/main.yml/badge.svg)](https://github.com/singcl/XmVideoPlayer/actions/workflows/main.yml)
![GitHub package.json version](https://img.shields.io/github/package-json/v/singcl/XmVideoPlayer)
[![Commitizen friendly](https://img.shields.io/badge/commitizen-friendly-brightgreen.svg)](http://commitizen.github.io/cz-cli/)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-%23FE5196?logo=conventionalcommits&logoColor=white)](https://conventionalcommits.org)

## XmVideoPlayer 在线播放客户端

[![windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/singcl/XmVideoPlayer/releases)
[![macos](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)](https://github.com/singcl/XmVideoPlayer/releases)
[![linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://github.com/singcl/XmVideoPlayer/releases)

一款高颜值,支持播放 mp4, m3u8,flv,mpeg-dash 等多种流媒体格式的桌面客户端 🔥。
支持播放本地视频资源，下载在线资源，播放列表&播放历史记录。

### Prerequisites

- `node >= 16`
- 资源转换依赖`ffmpeg`，应用每次启动会自动检测是否安装`ffmpeg`, 若没有安装则会自动下载并安装，已安装则跳过 WOW🎉!

### Start

```sh
#  start via NPM
npm run tauri dev
```

### Preview 🤩

|                                                                                                     GIF                                                                                                     |                                                                                                             GIF                                                                                                              |
| :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| [![启动自动检测下载ffmpeg](https://img.picgo.net/2024/05/28/ffmpeg2ac49a43a24f286e.gif)](https://www.picgo.net/image/%E5%90%AF%E5%8A%A8%E8%87%AA%E5%8A%A8%E6%A3%80%E6%B5%8B%E4%B8%8B%E8%BD%BDffmpeg.SPUkSq) | <a href="https://www.picgo.net/image/%E7%BC%96%E8%BE%91%E5%92%8C%E5%88%A0%E9%99%A4.SPU8D2"><img src="https://img.picgo.net/2024/05/28/d940986e0326a3e42934e8ac11fd32e0ce89b826259ba990.gif" alt="编辑和删除" border="0"></a> |

### Preview Gif 🤩

|                                                                                                      GIF                                                                                                       |                                                                                             GIF                                                                                             |
| :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| [![启动自动检测下载ffmpeg](https://img.picgo.net/2024/05/28/ffmpeg2ac49a43a24f286e.md.gif)](https://www.picgo.net/image/%E5%90%AF%E5%8A%A8%E8%87%AA%E5%8A%A8%E6%A3%80%E6%B5%8B%E4%B8%8B%E8%BD%BDffmpeg.SPUkSq) |                         [![m3u8 download](https://img.picgo.net/2024/05/28/m3u8_download15a99fd99b9008e9.md.gif)](https://www.picgo.net/image/m3u8-download.SPU1NW)                         |
|                                     [![播放m3u8](https://img.picgo.net/2024/05/28/m3u8601a4f219e6142de.md.gif)](https://www.picgo.net/image/%E6%92%AD%E6%94%BEm3u8.SPUUVi)                                     |                 [![反馈](https://img.picgo.net/2024/05/28/c0a7babc91b0b7fe52d1c8a54e58a1b604816655f83bc3fa.md.gif)](https://www.picgo.net/image/%E5%8F%8D%E9%A6%88.SPUow4)                  |
|                           [![主题](https://img.picgo.net/2024/05/28/fcc87cf2cb4c6d82b9eaf707f3d4a6efaf5c76b0b78d4bcb.md.gif)](https://www.picgo.net/image/%E4%B8%BB%E9%A2%98.SPUWhu)                           | [![编辑和删除](https://img.picgo.net/2024/05/28/d940986e0326a3e42934e8ac11fd32e0ce89b826259ba990.md.gif)](https://www.picgo.net/image/%E7%BC%96%E8%BE%91%E5%92%8C%E5%88%A0%E9%99%A4.SPU8D2) |

### 学习资料 🤩

#### MACOS

|                          macos 截图                          |                          macos 截图                          |
| :----------------------------------------------------------: | :----------------------------------------------------------: |
| ![macos](https://s2.loli.net/2022/10/31/Kr2BkOsG4RbUtjL.jpg) | ![macos](https://s2.loli.net/2022/10/31/sgWo4AHpKqFLlRr.jpg) |
| ![macos](https://s2.loli.net/2022/10/31/YbprKqoi7OI1Zhf.jpg) | ![macos](https://s2.loli.net/2022/10/31/qnY7IGduLh415JQ.jpg) |

#### Windows

|                          Windows 截图                           |                          Windows 截图                           |
| :-------------------------------------------------------------: | :-------------------------------------------------------------: |
|   ![界面](https://s2.loli.net/2022/09/18/4Yid5Ql81wnV2bU.png)   |   ![界面](https://s2.loli.net/2022/09/18/cbzwIdaXvoxWMi9.png)   |
| ![下载进度](https://s2.loli.net/2022/10/08/74otrlVCgKR2hfE.png) | ![本地视频](https://s2.loli.net/2022/10/10/jVI1m54AwbckHR2.png) |
|   ![界面](https://s2.loli.net/2023/11/13/7RVG1XvjNcSDBgz.png)   |   ![界面](https://s2.loli.net/2024/05/25/2HonVT5Kuw3WBDN.png)   |

## Roadmap

- [x] GithubActions: [🥂push 指定的 tag github actions 自动构建并创建 release draft🥂!](https://tauri.app/zh/v1/guides/building/cross-platform)
- [x] 播放列表&播放历史记录 WOW🎉🎉🎉！
- [ ] 跨域资源
  - [x] M3u8 资源跨域加载！ WOW🎉！
  - [ ] Flv 资源跨域加载！
  - [ ] Mpd 资源跨域加载！
- [x] 优化全屏播放体验
- [x] 应用托盘
- [x] 流媒体下载
  - [x] M3u8 资源下载！ WOW🎉！
  - [x] M3u8 资源下载进度展示 WOW🎉！
  - [x] M3u8 资源断点续下载 WOW🎉！
  - [x] 常规资源下载 WOW🎉！
- [x] 本地媒体资源播放 - 暂时只支持 mp4🎉
- [x] UI 兼容竖版视频
- [x] 提供主题界面 🎉！
- [x] Updater 🎉！
- [ ] 提供简单的视频格式转换工具
- [ ] Window Customization
- [ ] 截图功能

## XmVideoPlayer Releases

- [在线下载最新发布](https://singcl-xmvideoplayer-fresh.deno.dev/)
- [在线下载最新发布](https://tauri-update-server-chi.vercel.app/)

## Contributors

[![](https://contrib.rocks/image?repo=singcl/XmVideoPlayer)](https://github.com/singcl/XmVideoPlayer/graphs/contributors)
