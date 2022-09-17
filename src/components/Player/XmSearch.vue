<template>
  <div class="card">
    <a-input
      class="search-input"
      v-model="modelValue"
      @input="(v, e) => $emit('update:modelValue', v)"
      :placeholder="placeholder"
      :allow-clear="true"
      @clear="$emit('update:modelValue', undefined)"
      @press-enter="$emit('submit', modelValue)"
    />
      <!-- allow-clear -->
      <!-- BUG: build后的应用点击清楚没有反应，必须手动绑定onClear事件 -->
    <a-button
      type="primary"
      status="warning"
      @click="$emit('submit', modelValue)"
      >GO</a-button
    >
  </div>
  <div class="tips">Tips: 支持mp4, m3u8,flv等多种视频格式或直播流🔥。</div>
</template>

<script setup lang="ts">
defineProps({
  modelValue: String,
  placeholder: {
    type: String,
    required: false,
  },
});
// 可以重载的函数类型定义
defineEmits<{
  (e: "update:modelValue", v?: string): void;
  (e: "submit", v?: string): void;
}>();
</script>

<style scoped>
.card {
  margin-bottom: 5px;
  position: relative;
}

.search-input {
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
</style>
