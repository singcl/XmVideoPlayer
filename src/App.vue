<template>
  <div class="container" :style="{ height: cHeight }">
    <XmThemePanel />
    <h1 class="home-title">
      <img src="/logo.svg" class="logo vite" alt="XmVideoPlayer logo" />
      <span class="txt typing">{{ APP_TITLE }}</span>
    </h1>
    <!-- <HDescription /> -->
    <XmPlayer />
    <HBanner />
    <HTest />
    <!-- <HGreet /> -->
    <XmFeedback />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import debounce from '@singcl/throttle-debounce/debounce';
import { storeToRefs } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
import { appWindow /* WebviewWindow */ } from '@tauri-apps/api/window';
import { useHeightStore } from '@/stores';
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import HGreet from "./components/HGreet.vue";
import XmPlayer from './components/Player/XmPlayer.vue';
import HBanner from './components/HBanner.vue';
import XmFeedback from './components/XmFeedback.vue';
// import HDescription from "./components/HDescription.vue";
const APP_TITLE = import.meta.env.VITE_APP_TITLE;
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
  setTimeout(() => invoke('close_splashscreen'), 1000); // 让加载动画多显示一会儿
  // invoke('init_process');
  appWindow.listen('pong', (e) => {
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

<style scoped>
.container {
  /* padding-top: 10vh; */
  display: flex;
  width: var(--area-width);
  max-width: var(--area-width-max);
  height: 100vh;
  flex-direction: column;
  justify-content: flex-start;
  margin: 0 auto;
  background-color: rgb(0 0 0 / 1%);
  text-align: center;
}

.home-title {
  display: flex;
  align-items: center;
  margin: 12px auto;
}

.home-title .logo {
  width: 32px;
  height: 32px;
  padding: 2px;
}

.home-title .txt {
  height: 36px;
  margin-left: 5px;
  line-height: 36px;
  text-align: left;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.typing {
  overflow: hidden;
  border-right: 2px solid transparent;
  animation: typing 3s steps(42, end) infinite, blink-caret 0.55s step-end infinite;
  word-break: break-all;
}

/* 打印效果 */
@keyframes typing {
  from {
    width: 0;
  }

  to {
    width: 100%;
  }
}

/* 光标 */
@keyframes blink-caret {
  0%,
  100% {
    border-color: transparent;
  }

  50% {
    border-color: currentColor;
  }
}
</style>

<style>
.dplayer-video {
  max-height: v-bind(cHeightPlayer);
}
</style>
