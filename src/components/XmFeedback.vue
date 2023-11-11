<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const greetMsg = ref('');
const name = ref('');

async function handleFeedback(e: Event) {
  e.stopPropagation();
  e.preventDefault();
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke('greet', { name: name.value });
}
</script>

<template>
  <div class="feedback">
    <a-tooltip :mini="true">
      <template #content>
        <span style="color: #eee6e6; font-size: 12px">意见与建议</span>
      </template>
      <a-button class="feedback-btn" shape="circle" size="large" status="success" @click="handleFeedback">
        <XmSvgIcon name="icon-feedback" style="padding-left: 3px; font-size: 20px" />
      </a-button>
    </a-tooltip>
  </div>

  <p>{{ greetMsg }}</p>
</template>

<style scoped>
.feedback-btn {
  box-shadow: 0 2px 12px #0000001a;
}

.feedback {
  position: absolute;
  top: 50%;
  right: 0;
  padding: 5px 20px;
  transform: translate(50%, -50%) scale(1);
  transition: all 0.2s;
}

.feedback:hover,
.feedback:focus {
  transform: translate(0%, -50%) scale(1.1);
}
</style>
