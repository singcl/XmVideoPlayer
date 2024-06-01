<template>
  <div :style="{ height: cHeight }">
    <XmThemePanel />
    <RouterView />
    <HBanner />
    <XmFeedback />
    <HBackHome />
  </div>
</template>

<script setup lang="ts">
import debounce from '@singcl/throttle-debounce/debounce';
import { appWindow /* WebviewWindow */ } from '@tauri-apps/api/window';
import { useHeightStore } from '@/stores';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const heightStore = useHeightStore();
const cHeight = storeToRefs(heightStore).height;
const cHeightPlayer = computed(() => {
  return `calc(${cHeight.value} - 265px)`;
});

// onMounted(() => {
//   invoke('close_splashscreen');
// });
document.addEventListener('DOMContentLoaded', () => {
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
  // setTimeout(() => invoke('close_splashscreen'), 1000); // 让加载动画多显示一会儿
  // invoke('init_process');
  appWindow.listen('pong', (e: { payload: object }) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log('-----pong:', e.payload);
  });
  appWindow.onResized(
    debounce(500, () => {
      heightStore.change();
    })
  );
});
</script>

<style>
.dplayer-video {
  max-height: v-bind(cHeightPlayer);
}
</style>
