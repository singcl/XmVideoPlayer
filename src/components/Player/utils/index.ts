import Hls from "hls.js";

export function formatVideo(url: string) {
  if (/^https?:\/\/.+\.m3u8(\?(.*))?$/) {
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
  return {
    url,
    type: 'normal'
  };
}
