<template>
  <div class="player">
    <XmSearch v-model="mediaUrl" :placeholder="defaultUrl" @submit="restart" />
    <BannerWrapper>
      <BannerHeader :media-url="mediaUrl" />
    </BannerWrapper>
    <XmVideo ref="playerRef" :default-url="defaultUrl" />
    <BannerWrapper>
      <BannerFooter />
    </BannerWrapper>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Message } from '@arco-design/web-vue';
import '@arco-design/web-vue/lib/message/style';
// import API from '@/api';

import XmSearch from './XmSearch.vue';
import XmVideo from './XmVideo.vue';

const defaultUrl = 'https://bitdash-a.akamaihd.net/content/sintel/hls/video/800kbit.m3u8';
//
const playerRef = ref<InstanceType<typeof XmVideo>>();
const mediaUrl = ref('');

// 重载新的播放链接
// TODO:一定要销毁重建吗？更新video部分行不行？
function restart(v?: string) {
  // if (!v) return alert("请输入正确的链接");
  if (!v) return Message.info({ content: '请输入正确的链接' });

  const player = playerRef.value?.load({
    video: {
      url: v,
    },
  });
  player?.play();
}
</script>

<style scoped>
.player {
  width: 100%;
  max-width: var(--area-width-max);
  margin: 0 auto;
}
</style>
