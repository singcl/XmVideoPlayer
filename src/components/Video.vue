<script setup lang="ts">
import { ref, onMounted } from "vue";
import DPlayer, { DPlayerEvents } from "dplayer";
import Hls from "hls.js";

// 本地视频播放
import demoVideo from "./../assets/videos/demo.mp4";
import Search from "./Player/Search.vue";

const m3u8DefaultUrl =
  "https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8";
let dplayer: DPlayer | undefined;
//
const videoRef = ref<HTMLElement>();
const m3u8Url = ref("");
const isPlay = ref(false);

onMounted(() => {
  init();
});

//
function init() {
  if (!videoRef.value) return;
  if (dplayer) dplayer.destroy();
  //
  const dp = new DPlayer({
    container: videoRef.value,
    video: {
      // url: "https://api.dogecloud.com/player/get.mp4?vcode=5ac682e6f8231991&userId=17&ext=.mp4",
      url: m3u8Url.value || m3u8DefaultUrl,
      type: "customHls",
      customType: {
        customHls: function (video: HTMLVideoElement /* player */) {
          const hls = new Hls();
          hls.loadSource(video.src);
          hls.attachMedia(video);
        },
      },
    },
  });
  dp.on("play" as DPlayerEvents.play, () => {
    isPlay.value = true;
  });
  dp.on("pause" as DPlayerEvents.pause, () => {
    isPlay.value = false;
  });
  dplayer = dp;
  return dp;
}

function toggle() {
  dplayer?.toggle();
}

// 重载新的播放链接
// TODO:一定要销毁重建吗？更新video部分行不行？
// TODO:支持播放m3u8, flv, mp4等
function restart(v?: string) {
  if (!/^https?:\/\/.+\.m3u8(\?(.*))?$/.test(m3u8Url.value)) {
    return alert("请输入正确的m3u8链接");
  }
  init();
  dplayer?.play();
}
</script>

<template>
  <Search
    v-model="m3u8Url"
    @submit="restart"
    :placeholder="m3u8DefaultUrl"
  />
  <div class="card">
    <div style="width: 600px; margin: 0 auto">
      <div ref="videoRef"></div>
    </div>
  </div>
</template>

<style scoped>
.card {
  margin-bottom: 12px;
}

.card input {
  width: 482px;
}
</style>
