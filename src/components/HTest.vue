<template>
  <div class="container">
    <a-space>
      <a-button :loading="loading" type="primary" @click="handleDownloadClick">下载测试按钮</a-button>
      <a-button :loading="loading" type="primary" @click="handleFullClick">全屏测试按钮</a-button>
      <a-button :loading="loading" type="primary" @click="handlePiniaClick">pinia测试按钮</a-button>
    </a-space>
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
async function handleDownloadClick() {
  // const url = convertFileSrc('https://media.w3.org/2010/05/sintel/trailer.mp4', 'stream');
  // console.log('---url', url);
  // const response = await fetch(url, {
  //   method: 'GET',
  // });
  // console.log('--response', response);
  try {
    const downloadDirPath = await downloadDir();
    // const filePath = await save({
    //   // TODO: 这个filters什么意思？？
    //   // filters: [
    //   //   {
    //   //     name: 'Video',
    //   //     extensions: ['mp4'],
    //   //   },
    //   //   {
    //   //     name: 'Image',
    //   //     extensions: ['png', 'jpg', 'jpeg'],
    //   //   },
    //   // ],
    //   defaultPath: downloadDirPath,
    // });
    // if (!filePath) return;
    const filePath = 'C:\\Users\\Administrator\\Downloads\\1.txt';
    console.log('------', filePath);
    // loading.value = true;
    const res = await invoke('m3u8_download', {
      // m3u8Url: 'https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8',
      m3u8Url:
        'https://hls.vdtuzv.com/videos3/384e44a785222864cd61b31116880286/384e44a785222864cd61b31116880286.m3u8?auth_key=1715870315-66461a6b61e3f-0-6ea4561218213760f576d4b610759cc0&v=3&time=0',
      savePath: filePath,
    });
    console.log('------', res);
  } finally {
    loading.value = false;
  }
}

async function handleFullClick() {
  const full = await appWindow.isFullscreen();
  await appWindow.setFullscreen(!full);
}
async function handlePiniaClick() {
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
