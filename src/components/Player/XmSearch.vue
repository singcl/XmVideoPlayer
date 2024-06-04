<template>
  <div class="wrapper">
    <div class="search">
      <a-auto-complete
        :model-value="modelValue"
        :data="options"
        :placeholder="placeholder"
        :allow-clear="true"
        :filter-option="false"
        @change="(v: string) => $emit('update:modelValue', v)"
        @clear="handleClear"
        @press-enter="() => handleSubmit()"
        @search="handleSearch"
        @select="handleSelect"
      >
        <template #option="optInfos">
          <div v-for="opt in optInfos" :key="opt.value" class="play-opt">
            <span class="play-opt__operation">
              <icon-delete style="color: red" @click="handleOptDelete($event, opt.raw)" />
              <icon-edit style="margin-left: 3px; color: blue" @click="handleOptEdit($event, opt.raw)" />
            </span>
            <span>{{ opt.label }}</span>
          </div>
        </template>
      </a-auto-complete>
    </div>
    <!-- allow-clear -->
    <!-- BUG: build后的应用点击清楚没有反应，必须手动绑定onClear事件 -->
    <!-- <a-button type="primary" status="warning" @click="$emit('submit', modelValue)">GO</a-button> -->

    <a-dropdown-button type="primary" status="warning" @click="() => handleSubmit()" @select="handleDropdownSelect">
      PLAY
      <template #content>
        <a-doption :value="1">打开本地资源</a-doption>
        <!-- <a-doption>Save and Publish</a-doption> -->
      </template>
    </a-dropdown-button>
  </div>
  <div class="tips">Tips: 支持mp4,m3u8,flv,mpeg-dash等多种流媒体格式🔥。</div>
  <xm-history-edit-dialog v-model:visible="hisEditVisible" :data="hisEditData" @success="handleOptEditSuccess" />
</template>

<script setup lang="ts">
import { ref, onMounted, h } from 'vue';
import { downloadDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { save, open } from '@tauri-apps/api/dialog';
import { Modal, Message } from '@arco-design/web-vue';
// dropdown-button 没有自动导入button的样式  UPDATE:@arco-plugins/vite-vue 插件会自动加载组件样式 https://arco.design/vue/docs/start
// import '@arco-design/web-vue/es/button/style/css.js';
// import '@arco-design/web-vue/es/modal/style/css.js';
import API from '@/api';
import { useDebounceFn } from '@vueuse/core';
import { decodeURL } from '@/utils/tools';
// import { checkPinYin } from '@/utils/tools';
type HListItem = Await<ReturnType<typeof API.idb.getPlayerHistoryInfo>>['data'] & { label: string; value: string };

const props = defineProps({
  modelValue: {
    type: String,
    default: undefined,
  },
  placeholder: {
    type: String,
    required: false,
    default: undefined,
  },
});
// 可以重载的函数类型定义
const emits = defineEmits<{
  (e: 'update:modelValue', v?: string): void;
  (e: 'submit', v: number): void;
}>();

const options = ref<HListItem[]>([]);
const hisEditVisible = ref(false);
const hisEditData = ref<HListItem>();

onMounted(() => {
  getPlayList();
});

// 获取播放列表
async function getPlayList(keyword?: string) {
  const { data } = await API.idb.getPlayerHistoryPageList({ page: { pageNo: 1, pageSize: 50 }, keyword });
  options.value = (data?.list || []).map((item) => ({
    ...item,
    label: decodeURL(item.name),
    value: decodeURIComponent(item.url),
  }));
}

// 删除播放记录
async function handleOptDelete(e: Event, opt: HListItem) {
  e.stopPropagation();
  Modal.confirm({
    title: '删除确认',
    titleAlign: 'start',
    content: () =>
      h('div', { style: 'word-break: break-all' }, [
        h('span', null, '确认删除'),
        h('span', { style: 'color: red; margin: 0 3px' }, decodeURIComponent(opt.name)),
        h('span', null, '吗？'),
      ]),
    async onBeforeOk(done) {
      await API.idb.deletePlayerHistory(opt?.id || -1);
      await getPlayList();
      Message.success('删除成功');
      done(true);
    },
  });
}

// 编辑播放记录
async function handleOptEdit(e: Event, opt: HListItem) {
  e.stopPropagation();
  hisEditVisible.value = true;
  hisEditData.value = opt;
}

//
async function handleOptEditSuccess() {
  await getPlayList();
  Message.success('编辑成功');
  hisEditVisible.value = false;
}

//
async function onSearch(v?: string) {
  await getPlayList(v);
}

//
const handleSearch = useDebounceFn(onSearch, 500);

// Clear
function handleClear() {
  emits('update:modelValue', undefined);
  getPlayList();
}

// Submit
async function handleSubmit(url?: string) {
  const val = url ?? props.modelValue;
  if (!val) return Message.info({ content: '请输入正确的链接' });
  if (!/^(((ht|f)tps?)|stream):\/\//.test(val)) return Message.info({ content: '请输入正确的链接' });
  const { data } = await API.idb.savePlayerHistory({ name: val, url: val });
  emits('submit', data);
  await getPlayList(url);
}

function handleSelect() {}
//
function handleDropdownSelect(v?: number | string | Record<string, any>) {
  switch (v) {
    case 1:
      loadLocalSource();
      break;
    default:
      break;
  }
}

// 播报本地资源
async function loadLocalSource() {
  const downloadDirPath = await downloadDir();
  const filePath = await open({
    // TODO: 这个filters什么意思？？
    filters: [
      {
        name: 'Video',
        extensions: ['mp4' /* ,'ts' */],
      },
      // {
      //   name: 'Image',
      //   extensions: ['png', 'jpg', 'jpeg'],
      // },
    ],
    // directory: true,
    defaultPath: downloadDirPath,
  });
  if (!filePath) return;
  if (typeof filePath === 'string') {
    const path = convertFileSrc(filePath, 'stream');
    emits('update:modelValue', path);
    // emits('submit', path);
    handleSubmit(path);
  }
}
</script>

<style scoped>
.wrapper {
  position: relative;
  display: flex;
  margin-bottom: 5px;
}

.search {
  width: calc(100% - 5px - 55px);
  box-sizing: border-box;
  margin-right: 5px;
}

.tips {
  margin-bottom: 5px;
  color: #666;
  font-size: 12px;
  text-align: left;
}

.play-opt {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.play-opt__operation {
  display: none;
  margin-right: 5px;
}

.play-opt:hover .play-opt__operation {
  display: block;
}

/* 已经加入style.css */

/* :global(.arco-scrollbar-track.arco-scrollbar-track-direction-vertical) {
  display: none;
} */
</style>
