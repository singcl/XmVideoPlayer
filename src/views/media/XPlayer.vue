<template>
  <div class="container">
    <a-typography-title :heading="3" :ellipsis="true" style="margin-top: 0.5em">
      {{ hisInfo?.name && decodeURL(hisInfo?.name) }}
    </a-typography-title>
    <XmPlayer ref="playerRef" @change="handleXmPlayerChange" />
  </div>
</template>

<script setup lang="ts">
import API from '@/api';
import XmPlayer from '@/components/Player/XmPlayer.vue';
import { decodeURL } from '@/utils/tools';
import { onMounted, ref } from 'vue';

type HListItem = Await<ReturnType<typeof API.idb.getPlayerHistoryInfo>>['data'];
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const props = withDefaults(defineProps<{ id?: number | string }>(), {
  id: undefined,
});
onMounted(() => {
  getHisInfo(props.id);
});
// async function beforeRouteUpdate(to, from) {
//   // 对路由变化做出响应...
//   console.log('-----', props);
// }
const playerRef = ref<InstanceType<typeof XmPlayer>>();
const hisInfo = ref<HListItem>();
const getHisInfo = async (id?: number | string) => {
  if (typeof id === 'undefined') {
    return;
  }
  const { data } = await API.idb.getPlayerHistoryInfo({ id: Number(id) });
  if (!data) return;
  hisInfo.value = data;
  playerRef.value?.load({ video: { url: data.url } });
};

//
const handleXmPlayerChange = async (v?: string | number) => {
  if (!v) return;
  await getHisInfo(v);
};

//
</script>

<style scoped>
.container {
  /* padding-top: 10vh; */
  display: flex;
  width: var(--area-width);
  max-width: var(--area-width-max);
  height: 100vh;
  flex-direction: column;
  justify-content: flex-start;
  margin: 0 auto;
  background-color: rgb(0 0 0 / 1%);
  text-align: center;
}
</style>
