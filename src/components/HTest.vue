<template>
  <div class="container">
    <a-button :loading="loading" type="primary" @click="handleTestClick">测试按钮</a-button>
  </div>
</template>

<script setup lang="ts">
// import { ResponseType, fetch } from '@tauri-apps/api/http';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { downloadDir } from '@tauri-apps/api/path';
import { ref } from 'vue';
import { save } from '@tauri-apps/api/dialog';

// import HDescription from "./components/HDescription.vue";
const loading = ref(false);
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
    // console.log('------', filePath);
    loading.value = true;
    const res = await invoke('video_download', { url: 'http://vjs.zencdn.net/v/oceans.mp4', path: filePath });
    console.log('------', res);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.container {
  margin-top: 12px;
  width: 100%;
}
</style>
