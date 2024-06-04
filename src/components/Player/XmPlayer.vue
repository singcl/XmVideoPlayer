<template>
  <div class="player">
    <XmListSearch v-model="mediaUrl" :placeholder="'请输入资源地址...'" @submit="handleSubmit" />
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

// 重载新的播放链接
// TODO:一定要销毁重建吗？更新video部分行不行？
// Submit
async function handleSubmit(opt?: { name: string; url: string }) {
  if (!opt) return Message.info({ content: '请输入正确的链接' });
  if (!/^(((ht|f)tps?)|stream):\/\//.test(opt.url)) return Message.info({ content: '请输入正确的链接' });
  const { data } = await API.idb.savePlayerHistory({ name: opt.name, url: opt.url });
  emits('change', data);
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
