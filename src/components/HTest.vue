<template>
  <div class="container">
    <a-button type="primary" @click="handleTestClick">测试按钮</a-button>
  </div>
</template>

<script setup lang="ts">
// import { ResponseType, fetch } from '@tauri-apps/api/http';
import { convertFileSrc } from '@tauri-apps/api/tauri';
// import HDescription from "./components/HDescription.vue";

// 测试按钮
async function handleTestClick() {
  const url = convertFileSrc('https://media.w3.org/2010/05/sintel/trailer.mp4', 'stream');
  console.log('---url', url);
  const response = await fetch(url, {
    method: 'GET',
  });
  console.log('--response', response);
}
</script>

<style scoped>
.container {
  margin-top: 12px;
  width: 100%;
}
</style>
