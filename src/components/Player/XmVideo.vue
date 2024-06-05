<template>
  <div class="card">
    <div ref="videoRef"></div>
  </div>
</template>

<script setup lang="ts">
import { appWindow /* WebviewWindow */ } from '@tauri-apps/api/window';
import { ref, onMounted, onBeforeUnmount } from 'vue';
import DPlayer from '@singcl/dplayer';
import { useHeightStore } from '@/stores';
import { formatVideo } from './utils';
// 本地视频播放
// import demoVideo from "./../../assets/videos/demo.mp4";

const props = defineProps({
  defaultUrl: {
    type: String,
    default: 'https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8',
  },
});

defineExpose({
  load,
});

let dplayer: DPlayer | undefined;
//
const videoRef = ref<HTMLElement>();
const isPlay = ref(false);
const heightStore = useHeightStore();

onMounted(() => {
  // load();
});

onBeforeUnmount(() => {
  dplayer?.destroy();
});

function load(options?: Omit<DPlayerOptions, 'container'>) {
  if (!videoRef.value) return;
  if (dplayer) dplayer.destroy();
  //
  const { video: { url = props.defaultUrl, type = '' } = {} } = options || {};
  const vo = formatVideo(url, type);
  //
  const dp = new DPlayer({
    // screenshot: true,
    playbackSpeed: [0.5, 0.75, 1, 1.25, 1.5, 2],
    // contextmenu: [
    //   {
    //     text: 'singcl',
    //     link: 'https://github.com/singcl',
    //   },
    //   {
    //     text: 'Version',
    //     click: (player: DPlayer) => {
    //       console.log(player);
    //     },
    //   },
    // ],
    ...(options || {}),
    container: videoRef.value,
    video: vo,
  });
  //
  dp.on('play' as DPlayerEvents, () => {
    isPlay.value = true;
  });
  dp.on('pause' as DPlayerEvents, () => {
    isPlay.value = false;
  });
  // dp?.on('canplay' as DPlayerEvents, async () => {
  //   await API.idb.savePlayerHistory({ name: v, url: v });
  //   console.log('-----新增成功:', v);
  // });
  dp.on('fullscreen' as DPlayerEvents, async () => {
    const full = await appWindow.isFullscreen();
    if (!full) await appWindow.setFullscreen(true);
    heightStore.change();
  });
  dp.on('fullscreen_cancel' as DPlayerEvents, async () => {
    const full = await appWindow.isFullscreen();
    if (full) await appWindow.setFullscreen(false);
    heightStore.change();
  });
  dp.play();
  dplayer = dp;
  return dp;
}

function toggle() {
  dplayer?.toggle();
}
</script>

<style scoped>
/* style */
</style>
