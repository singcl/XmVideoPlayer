<script setup lang="ts">
import { ref, onMounted } from "vue";
import DPlayer from "dplayer";
import Hls from "hls.js";

// 本地视频播放
import demoVideo from "./../assets/videos/demo.mp4";

const videoRef = ref<HTMLElement>();

onMounted(() => {
  if (!videoRef.value) return;
  const dp = new DPlayer({
    container: videoRef.value,
    video: {
      // url: "https://api.dogecloud.com/player/get.mp4?vcode=5ac682e6f8231991&userId=17&ext=.mp4",
      url: "https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8",
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
});
</script>

<template>
  <div class="card">
    <div style="width: 600px; margin: 0 auto">
      <div ref="videoRef"></div>
    </div>
  </div>
</template>
