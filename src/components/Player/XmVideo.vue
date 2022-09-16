<template>
  <div class="card">
    <div style="width: 600px; margin: 0 auto">
      <div ref="videoRef"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import DPlayer from "dplayer";
import { formatVideo } from "./utils";
// 本地视频播放
import demoVideo from "./../../assets/videos/demo.mp4";

const props = defineProps({
  defaultUrl: {
    type: String,
    default: demoVideo,
  },
});

defineExpose({
  load,
});

let dplayer: DPlayer | undefined;
//
const videoRef = ref<HTMLElement>();
const isPlay = ref(false);

onMounted(() => {
  load();
});

function load(options?: Omit<DPlayerOptions, "container">) {
  if (!videoRef.value) return;
  if (dplayer) dplayer.destroy();
  //
  const { video: { url = props.defaultUrl } = {} } = options || {};
  const vo = formatVideo(url);
  //
  const dp = new DPlayer({
    ...(options || {}),
    container: videoRef.value,
    video: vo,
  });
  //
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
</script>

<style scoped>
.card {
  margin-bottom: 12px;
}
</style>
