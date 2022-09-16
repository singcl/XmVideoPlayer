import Hls from "hls.js";
import flv from "flv.js";

export function formatVideo(url: string) {
  if (/^https?:\/\/.+\.m3u8(\?(.*))?$/.test(url)) {
    return {
      url: url,
      type: "customHls",
      customType: {
        customHls: function (video: HTMLVideoElement /* player */) {
          const hls = new Hls();
          hls.loadSource(video.src);
          hls.attachMedia(video);
        },
      },
    };
  }
  if (/^https?:\/\/.+\.flv(\?(.*))?$/.test(url)) {
    return {
      url: url,
      type: "customFlv",
      customType: {
        customFlv: function (video: HTMLVideoElement /* player */) {
          const flvPlayer = flv.createPlayer({
            type: "flv",
            url: video.src,
          });
          flvPlayer.attachMediaElement(video);
          flvPlayer.load();
        },
      },
    };
  }
  return {
    url,
    type: "normal",
  };
}
