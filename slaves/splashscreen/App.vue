<template>
  <div class="start-wrapper">
    <h1 class="start-name">XmVideoPlayer</h1>
    <div class="start loading">
      <span></span>
      <span></span>
      <span></span>
      <span></span>
      <span></span>
    </div>
  </div>
  <div class="tips">{{ tips }}</div>
</template>

<script setup lang="ts">
import { appWindow /* WebviewWindow */ } from '@tauri-apps/api/window';
import { ref } from 'vue';

const tips = ref('');

document.addEventListener('DOMContentLoaded', () => {
  appWindow.listen<{ message?: string }>('init_resources', (e) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    // console.log(e.payload);
    const message = e.payload.message || '';
    tips.value = message;
  });
});
</script>

<style scoped>
.tips {
  padding: 5px 10px;
  color: #712020;
  font-size: 14px;
  text-align: center;
}
</style>
