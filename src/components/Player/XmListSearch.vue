<template>
  <div class="wrapper">
    <div class="search">
      <a-input-search
        :model-value="modelValue"
        :placeholder="placeholder"
        :allow-clear="true"
        :loading="loading"
        :spellcheck="false"
        @change="handleChange"
        @input="handleInput"
        @clear="handleClear"
        @press-enter="(e: KeyboardEvent) => $emit('press-enter', e)"
        @search="handleSearch"
      >
      </a-input-search>
    </div>
    <!-- allow-clear -->
    <!-- BUG: build后的应用点击清除没有反应，必须手动绑定onClear事件 -->
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
</template>

<script setup lang="ts">
import { downloadDir } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { save, open } from '@tauri-apps/api/dialog';
import { Message } from '@arco-design/web-vue';
// dropdown-button 没有自动导入button的样式  UPDATE:@arco-plugins/vite-vue 插件会自动加载组件样式 https://arco.design/vue/docs/start
// import '@arco-design/web-vue/es/button/style/css.js';
// import '@arco-design/web-vue/es/modal/style/css.js';
import API from '@/api';
// import { checkPinYin } from './utils';

const props = defineProps({
  modelValue: {
    type: String,
    default: undefined,
  },
  placeholder: {
    type: String,
    required: false,
    default: '请输入资源地址...',
  },
  loading: {
    type: Boolean,
    required: false,
    default: false,
  },
});
// 可以重载的函数类型定义
const emits = defineEmits<{
  (e: 'update:modelValue', v?: string): void;
  (e: 'submit', v?: { name: string; url: string }): void;
  (e: 'change', v?: string): void;
  (e: 'input', v?: string): void;
  (e: 'search', v?: string): void;
  (e: 'clear'): void;
  (e: 'press-enter', v: KeyboardEvent): void;
}>();

// 可以发起请求远程获取
function handleSearch(v?: string) {
  emits('search', v);
}
// Clear
function handleClear() {
  emits('update:modelValue', undefined);
  emits('clear');
}

//
function handleInput(v?: string) {
  emits('update:modelValue', v);
  emits('input', v);
}

//
function handleChange(v?: string) {
  emits('change', v);
}

// Submit
async function handleSubmit(url?: string) {
  const val = url ?? props.modelValue;
  // TODO: URI校验
  if (!val) return Message.info({ content: '请输入正确的链接' });
  emits('submit', { url: val, name: val });
}

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
</style>
