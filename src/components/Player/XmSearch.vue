<template>
  <div class="wrapper">
    <div class="search">
      <a-auto-complete
        ref="searchRef"
        :model-value="modelValue"
        :filter-option="
          (v, option) => {
            if (!option.label) return true;
            return checkPinYin(option.label, v);
          }
        "
        :data="options"
        :placeholder="placeholder"
        :allow-clear="true"
        @change="(v) => $emit('update:modelValue', v)"
        @clear="handleClear"
        @press-enter="$emit('submit', modelValue)"
        @search="handleSearch"
        @select="handleSelect"
      />
    </div>
    <!-- allow-clear -->
    <!-- BUG: build后的应用点击清楚没有反应，必须手动绑定onClear事件 -->
    <!-- <a-button type="primary" status="warning" @click="$emit('submit', modelValue)">GO</a-button> -->

    <a-dropdown-button
      type="primary"
      status="warning"
      @click="$emit('submit', modelValue)"
      @select="handleDropdownSelect"
    >
      PLAY
      <template #content>
        <a-doption :value="1">打开本地资源</a-doption>
        <!-- <a-doption>Save and Publish</a-doption> -->
      </template>
    </a-dropdown-button>
  </div>
  <div class="tips">Tips: 支持mp4,m3u8,flv,mpeg-dash等多种流媒体格式🔥。</div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { downloadDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { save, open } from '@tauri-apps/api/dialog';
import { AutoComplete } from '@arco-design/web-vue';
// BUG:dropdown-button 没有自动导入button的样式
import '@arco-design/web-vue/es/button/style/css.js';
import API from '@/api';
import { checkPinYin } from './utils';

defineProps({
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
  (e: 'submit', v?: string): void;
}>();

const options = ref<{ label: string; value: string }[]>([]);
const searchRef = ref<InstanceType<typeof AutoComplete>>();

onMounted(async () => {
  const { data = [] } = await API.idb.getPlayerHistoryList();
  console.log('------data', data);
  options.value = data.map((item) => ({ label: item.name, value: item.url }));
});

// 可以发起请求远程获取
function handleSearch(v: string) {
  // 可以不要
  // options.value = m3u8List.filter((item) => checkPinYin(item.label, v));
}
//
function handleClear() {
  emits('update:modelValue', undefined);
  // options.value = m3u8List;
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
    emits('submit', path);
  }
}
</script>

<style scoped>
.wrapper {
  display: flex;
  margin-bottom: 5px;
  position: relative;
}

.search {
  box-sizing: border-box;
  margin-right: 5px;
  width: calc(100% - 5px - 55px);
}

.tips {
  text-align: left;
  margin-bottom: 5px;
  color: #666;
  font-size: 10px;
}

/* 已经加入style.css */
/* :global(.arco-scrollbar-track.arco-scrollbar-track-direction-vertical) {
  display: none;
} */
</style>
