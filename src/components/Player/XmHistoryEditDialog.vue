<template>
  <a-modal
    v-model:visible="modalVisible"
    title-align="start"
    :closable="false"
    :mask-closable="false"
    :esc-to-close="false"
    @ok="handleOk"
    @cancel="handleCancel"
  >
    <template #title>资源更新</template>
    <a-form :model="form">
      <a-form-item field="name" label="资源名称">
        <a-input v-model="form.name" placeholder="please enter your username..." />
      </a-form-item>
      <a-form-item field="url" label="资源URI">
        <a-input v-model="form.url" placeholder="please enter your post..." disabled />
        <icon-copy class="icon-copy icon-clipboard" :data-clipboard-text="form.url" />
      </a-form-item>
    </a-form>
  </a-modal>
</template>

<script setup lang="ts">
import { reactive, ref, computed, watchEffect, onMounted, onBeforeUnmount } from 'vue';
import ClipboardJS from 'clipboard';
import { Message } from '@arco-design/web-vue';
import API from '@/api';

interface HistoryData {
  label: string;
  value: string;
  id: number;
}

// 当使用基于类型的声明时，我们失去了为 props 声明默认值的能力。这可以通过 withDefaults 编译器宏解决：
// 或者，你可以使用目前为实验性的响应性语法糖
// https://cn.vuejs.org/guide/typescript/composition-api.html#typing-component-props
const props = withDefaults(defineProps<{ data?: HistoryData; visible?: boolean }>(), {
  visible: false,
  data: undefined,
});

// 可以重载的函数类型定义
const emits = defineEmits<{
  (e: 'update:visible', v?: boolean): void;
  (e: 'success', v?: number): void;
}>();

const form = reactive({
  name: '',
  url: '',
});
const clipboard = ref<InstanceType<typeof ClipboardJS>>();

// v-model
const modalVisible = computed({
  get() {
    return props.visible;
  },
  set(newValue) {
    emits('update:visible', newValue);
  },
});

onMounted(() => {
  const c = new ClipboardJS('.icon-clipboard');
  c.on('success', (e) => {
    Message.success({ content: '复制成功' });
    e.clearSelection();
  });
  clipboard.value = c;
});

onBeforeUnmount(() => {
  clipboard.value?.destroy();
});

watchEffect(() => {
  if (props.visible && props.data) {
    form.name = props.data.label;
    form.url = props.data.value;
  }
});

async function handleOk() {
  if (!props.data) {
    return;
  }
  const params = {
    ...form,
    id: props.data.id,
  };
  const { data } = await API.idb.updatePlayerHistory(params);
  emits('success', data);
}
function handleCancel() {}
</script>

<style scoped>
.icon-copy {
  margin-left: 5px;
  font-size: 16px;
}
</style>
