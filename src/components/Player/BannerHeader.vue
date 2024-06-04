<template>
  <div class="wrapper">
    <div class="b-header">
      <div class="avatar">
        <div class="avatar-dec"></div>
      </div>
      <div class="op">
        <icon-refresh
          v-if="m3u8Loading || normalLoading"
          class="op-ico"
          :size="24"
          :style="{ color: '#a5bcff' }"
          spin
        />
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
        <div class="progress__track">
          {{ downloadPayload.message }}
        </div>
        <div
          :style="{ width: `${(downloadPayload.current / downloadPayload.total) * 100}%` }"
          :class="{ success: downloadPayload.current >= downloadPayload.total }"
          class="progress__chunk"
        ></div>
      </div>
      <template #content>
        <a-progress type="circle" :stroke-width="12" :percent="percent" />
      </template>
    </a-tooltip>
  </div>
</template>

<script setup lang="ts">
import { Message } from '@arco-design/web-vue';
import { checkM3U8Url } from '@/utils/validator';
import { useDownloadListener } from '@/composable/useDownloadListener';
import { useDownloadM3u8 } from '@/composable/useDownloadM3u8';
import { useDownloadNormal } from '@/composable/useDownloadNormal';

const props = defineProps({
  mediaUrl: {
    type: String,
    default: '',
  },
});
const { percent, downloadPayload } = useDownloadListener();
const { download: downloadM3u8, loading: m3u8Loading } = useDownloadM3u8();
const { download: downloadNormal, loading: normalLoading } = useDownloadNormal();

//
// 下载
async function handleDownloadClick() {
  if (!props.mediaUrl) return Message.info({ content: '请输入正确的链接' });
  if (checkM3U8Url(props.mediaUrl)) {
    await downloadM3u8(props.mediaUrl);
  } else {
    await downloadNormal(props.mediaUrl);
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
  position: relative;
  width: 100%;
  align-items: center;
  background-color: #f7f7f7;
}

.progress__track {
  height: 10px;
  font-size: 10px;
  line-height: 10px;
}

.progress__chunk {
  position: absolute;
  z-index: 1;
  top: 0;
  left: 0;
  width: 0;
  height: 10px;
  box-sizing: border-box;
  background-color: rgb(37 93 197 / 72%);

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
