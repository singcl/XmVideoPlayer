<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import DPlayer from "dplayer";

// TODO: 本地视频播放
import demoVideo from "./../assets/videos/七夕.mp4";

const videoRef = ref();

onMounted(() => {
  const dp = new DPlayer({
    container: videoRef.value,
    video: {
      url: "https://api.dogecloud.com/player/get.mp4?vcode=5ac682e6f8231991&userId=17&ext=.mp4",
    },
  });
});
</script>

<template>
  <div class="card">
    <div>Dplayer播放器</div>
    <div style="width: 900px;margin: 0 auto;">
      <div ref="videoRef"></div>
    </div>
  </div>

</template>
