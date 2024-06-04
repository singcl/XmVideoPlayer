<template>
  <div class="player">
    <XmSearch v-model="mediaUrl" :placeholder="'请输入资源地址...'" @submit="handleSubmit" />
    <BannerWrapper>
      <BannerHeader :media-url="mediaUrl" />
    </BannerWrapper>
    <XmVideo ref="playerRef" />
    <BannerWrapper>
      <BannerFooter />
    </BannerWrapper>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Message } from '@arco-design/web-vue';
import '@arco-design/web-vue/lib/message/style';
import XmVideo from './XmVideo.vue';
import API from '@/api';
defineExpose({
  load,
});

const mediaUrl = defineModel<string>();

// 可以重载的函数类型定义
const emits = defineEmits<{
  (e: 'change', v?: string | number): void;
}>();
//
const playerRef = ref<InstanceType<typeof XmVideo>>();

// Submit
async function handleSubmit(id: number) {
  emits('change', id);
}

//
async function load(options?: Omit<DPlayerOptions, 'container'>) {
  const player = playerRef.value?.load(options);
  player?.play();
}
</script>

<style scoped>
.player {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: var(--area-width-max);
  margin: 0 auto;
}
</style>
