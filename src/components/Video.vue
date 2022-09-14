<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import DPlayer from "dplayer";

// 本地视频播放
import demoVideo from "./../assets/videos/demo.mp4";

const videoRef = ref();

onMounted(() => {
  const dp = new DPlayer({
    container: videoRef.value,
    video: {
      // url: "https://api.dogecloud.com/player/get.mp4?vcode=5ac682e6f8231991&userId=17&ext=.mp4",
      url: demoVideo,
    },
  });
});
</script>

<template>
  <div class="card">
    <div style="width: 600px;margin: 0 auto;">
      <div ref="videoRef"></div>
    </div>
  </div>

</template>
