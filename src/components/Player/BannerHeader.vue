<template>
  <div class="wrapper">
    <div class="b-header">
      <div class="avatar">
        <div class="avatar-dec"></div>
      </div>
      <div class="op">
        <icon-refresh v-if="loading" class="op-ico" :size="24" :style="{ color: '#a5bcff' }" spin />
        <icon-cloud-download
          v-else
          class="op-ico"
          :size="24"
          :style="{ color: '#a5bcff' }"
          @click="handleDownloadClick"
        />
        <icon-heart class="op-ico" :size="24" :style="{ color: '#a5bcff' }" />
        <icon-thumb-up class="op-ico" :size="24" :style="{ color: '#a5bcff' }" />
      </div>
    </div>
    <a-tooltip>
      <div v-if="downloadPayload.total > 0 && downloadPayload.current < downloadPayload.total" class="progress">
        <div
          :style="{ width: `${(downloadPayload.current / downloadPayload.total) * 100}%` }"
          :class="{ success: downloadPayload.current >= downloadPayload.total }"
          class="progress__chunk"
        ></div>
      </div>
      <template #content>
        <a-progress
          type="circle"
          :stroke-width="12"
          :percent="Number(Number(downloadPayload.current / downloadPayload.total).toFixed(2))"
        />
      </template>
    </a-tooltip>
  </div>
</template>

<script setup lang="ts">
import { appWindow /* WebviewWindow */ } from '@tauri-apps/api/window';
import { ref, reactive } from 'vue';
import { /* convertFileSrc */ invoke } from '@tauri-apps/api/tauri';
import { downloadDir } from '@tauri-apps/api/path';
import { save, open } from '@tauri-apps/api/dialog';
import { Message } from '@arco-design/web-vue';
import { checkM3U8Url } from '@/utils/validator';

interface PayloadDownload {
  downloadType: string;
  message: string;
  total: string;
  current: string;
}

interface PayloadDownloadFed {
  downloadType: string;
  message: string;
  total: number;
  current: number;
}

const props = defineProps({
  mediaUrl: {
    type: String,
    default: '',
  },
});
const loading = ref(false);
const downloadPayload = ref<PayloadDownloadFed>({ downloadType: 'm3u8', message: '', total: 0, current: 0 });
appWindow.listen<PayloadDownload>('download', (e) => {
  console.log('-----download:', e.payload);
  downloadPayload.value = {
    ...e.payload,
    total: Number(e.payload.total),
    current: Number(e.payload.current),
  };
});
// 下载
async function handleDownloadClick() {
  if (!props.mediaUrl) return Message.info({ content: '请输入正确的链接' });
  if (checkM3U8Url(props.mediaUrl)) {
    await downloadM3u8();
  } else {
    await downloadNormal();
  }
}

async function downloadM3u8() {
  try {
    const downloadDirPath = await downloadDir();
    const filePath = await save({
      filters: [
        {
          name: '视频',
          extensions: ['mp4'],
        },
        // {
        //   name: '图片',
        //   extensions: ['png', 'jpg', 'jpeg'],
        // },
      ],
      // directory: true,
      defaultPath: downloadDirPath,
    });
    if (!filePath) return;
    console.log('------', filePath);
    loading.value = true;
    const res = await invoke('m3u8_download', {
      m3u8Url: props.mediaUrl,
      savePath: filePath,
    });
    console.log('------', res);
    Message.success({ content: '下载成功！' });
  } finally {
    loading.value = false;
  }
}

async function downloadNormal() {
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
    const res = await invoke('video_download', { url: props.mediaUrl, path: filePath });
    console.log('------', res);
    Message.success({ content: '下载成功！' });
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.wrapper {
  position: relative;
  display: flex;
  height: 100%;
  max-height: 56px;
  flex-direction: column;
  justify-content: space-between;
}

.progress {
  display: flex;
  width: 100%;
  align-items: center;
  background-color: #f7f7f7;
}

.progress__chunk {
  width: 5px;
  height: 5px;
  box-sizing: border-box;
  background-color: #255dc5;

  /* border: 1px solid #f7f7f7; */
}

/* .progress__chunk + .progress__chunk {
  border-left: none;
} */

.progress__chunk.success {
  background-color: green;
}

.b-header {
  display: flex;
  height: 48px;
  align-items: center;
  justify-content: space-between;
  margin: 0 12px 0 4px;
}

.avatar {
  position: relative;
  width: 42px;
  height: 42px;
  border-radius: 2px;
  background-image: url('./../../assets/avatar.webp');
  background-size: cover;
}

.avatar-dec {
  position: relative;
  width: 100%;
  height: 100%;
  background-image: url('./../../assets/avatar-dec.webp');
  background-size: cover;
}

.op {
  display: flex;
  align-items: center;
}

.op-ico + .op-ico {
  margin-left: 8px;
}
</style>
