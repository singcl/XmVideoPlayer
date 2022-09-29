<template>
  <div class="container">
    <a-button :loading="loading" type="primary" @click="handleTestClick">测试按钮</a-button>
  </div>
</template>

<script setup lang="ts">
// import { ResponseType, fetch } from '@tauri-apps/api/http';
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';
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
    loading.value = true;
    const res = await invoke('video_download', { path: 'http://vjs.zencdn.net/v/oceans.mp4' });
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
