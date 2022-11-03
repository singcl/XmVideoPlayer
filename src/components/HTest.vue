<template>
  <div class="container">
    <a-button :loading="loading" type="primary" @click="handleTestClick">下载测试按钮</a-button>
    <a-button :loading="loading" type="primary" @click="handleTestClick2">全屏测试按钮</a-button>
    <a-button :loading="loading" type="primary" @click="handleTestClick3">pinia测试按钮</a-button>
  </div>
</template>

<script setup lang="ts">
// import { ResponseType, fetch } from '@tauri-apps/api/http';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { appWindow /* WebviewWindow */ } from '@tauri-apps/api/window';
import { downloadDir } from '@tauri-apps/api/path';
import { ref } from 'vue';
import { save } from '@tauri-apps/api/dialog';
import { useHeightStore } from '@/stores';

// import HDescription from "./components/HDescription.vue";
const loading = ref(false);
const heightStore = useHeightStore();
// 测试按钮
async function handleTestClick() {
  // const url = convertFileSrc('https://media.w3.org/2010/05/sintel/trailer.mp4', 'stream');
  // console.log('---url', url);
  // const response = await fetch(url, {
  //   method: 'GET',
  // });
  // console.log('--response', response);
  try {
    const downloadDirPath = await downloadDir();
    const filePath = await save({
      // TODO: 这个filters什么意思？？
      // filters: [
      //   {
      //     name: 'Video',
      //     extensions: ['mp4'],
      //   },
      //   {
      //     name: 'Image',
      //     extensions: ['png', 'jpg', 'jpeg'],
      //   },
      // ],
      defaultPath: downloadDirPath,
    });
    if (!filePath) return;
    console.log('------', filePath);
    loading.value = true;
    const res = await invoke('m3u8_download', {
      m3u8Url: 'https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8',
      savePath: filePath,
    });
    console.log('------', res);
  } finally {
    loading.value = false;
  }
}

async function handleTestClick2() {
  const full = await appWindow.isFullscreen();
  await appWindow.setFullscreen(!full);
}
async function handleTestClick3() {
  heightStore.$patch({ height: '50vh' });
}
</script>

<style scoped>
.container {
  display: flex;
  width: 100%;
  flex-direction: row;
  margin-top: 12px;
}
</style>
