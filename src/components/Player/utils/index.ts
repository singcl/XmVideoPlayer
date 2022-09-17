import Hls from "hls.js";
import flv from "flv.js";

export function formatVideo(url: string, type?: string) {
  const isHls = type === "hls" || /^https?:\/\/.+(\.)?m3u8(\.php)?(\?(.*))?$/.test(url);
  if (isHls) {
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
  const isFlv =
    type === "flv" || /^https?:\/\/.+\.(flv|xs)(\?(.*))?$/.test(url);
  if (isFlv) {
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
