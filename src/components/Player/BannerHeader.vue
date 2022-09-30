<template>
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
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { /* convertFileSrc */ invoke } from '@tauri-apps/api/tauri';
import { downloadDir } from '@tauri-apps/api/path';
import { save } from '@tauri-apps/api/dialog';
import { Message } from '@arco-design/web-vue';

const props = defineProps({
  mediaUrl: {
    type: String,
    default: '',
  },
});
const loading = ref(false);
// 下载
async function handleDownloadClick() {
  if (!props.mediaUrl) return Message.info({ content: '请输入正确的链接' });
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
    Message.success({ content: '下载成功！' });
    console.log('------', res);
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.b-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  margin: 0 12px 0 4px;
}
.avatar {
  position: relative;
  width: 48px;
  height: 48px;
  background-image: url('./../../assets/avatar.webp');
  background-size: cover;
  border-radius: 2px;
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
