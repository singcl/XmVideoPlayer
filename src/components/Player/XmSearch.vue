<template>
  <div class="card">
    <input
      class="search-input"
      v-model="modelValue"
      @input="$emit('update:modelValue', $event.target?.value)"
      :placeholder="placeholder"
    />
    <button type="button" @click="$emit('submit', modelValue)">GO</button>
    <!-- <button type="button" @click="toggle()">
      {{ isPlay ? "暂停" : "播放" }}
    </button> -->
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
  width: calc(100% - 5px - 66px);
}

.tips {
  text-align: left;
  margin-bottom: 5px;
  color: #666;
  font-size: 10px;
}
</style>
