<script setup lang="ts">
import { useDownloadListener } from '@/composable/useDownloadListener';
import { ref } from 'vue';
// import API from '@/api';
// import type { FormInstance } from '@arco-design/web-vue/es/form';
// import { Notification } from '@arco-design/web-vue';
// import '@arco-design/web-vue/es/notification/style/css.js';

// const loading = ref(false);
const visible = ref(false);

const { ballShow, percent } = useDownloadListener();
async function handleDownloadDetail(e: Event) {
  e.stopPropagation();
  e.preventDefault();
  visible.value = !visible.value;
}
</script>

<template>
  <div v-if="ballShow" class="download">
    <a-tooltip :mini="true">
      <template #content>
        <div style="width: 65px; color: #eee6e6; font-size: 12px">下载进度</div>
      </template>
      <a-button class="download-btn" shape="circle" size="large" @click="handleDownloadDetail">
        <a-progress type="circle" size="small" :percent="percent" />
      </a-button>
    </a-tooltip>
  </div>
</template>

<style scoped>
.download-btn {
  box-shadow: 0 2px 12px #0000001a;
}

.download {
  position: fixed;
  top: 40%;
  right: 0;
  padding: 5px 20px;
  transform: translate(50%, -50%) scale(1);
  transition: all 0.2s;
}

.download:hover,
.download:focus {
  transform: translate(0%, -50%) scale(1.1);
}

/* stylelint-disable-next-line selector-pseudo-class-no-unknown */
.download-btn :deep(.arco-progress-circle-wrapper) {
  width: 36px !important;
  height: 36px !important;
}
</style>
