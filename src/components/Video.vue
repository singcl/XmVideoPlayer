<script setup lang="ts">
import { ref } from "vue";

import XmSearch from "./Player/XmSearch.vue";
import XmVideo from "./Player/XmVideo.vue";

const defaultUrl =
  "https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8";
//
const playerRef = ref<InstanceType<typeof XmVideo>>();
const m3u8Url = ref("");

// 重载新的播放链接
// TODO:一定要销毁重建吗？更新video部分行不行？
// TODO:支持播放m3u8, flv, mp4等
function restart(v?: string) {
  if (!v) return alert("请输入正确的链接");
  const player = playerRef.value?.load({
    video: {
      url: v,
    },
  });
  player?.play();
}
</script>

<template>
  <XmSearch v-model="m3u8Url" @submit="restart" :placeholder="defaultUrl" />
  <XmVideo ref="playerRef" :default-url="defaultUrl" />
</template>

<style scoped>
.card {
  margin-bottom: 12px;
}
</style>
