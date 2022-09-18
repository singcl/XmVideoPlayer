<template>
  <div class="b-footer">
    <div
      v-for="item in giftList"
      :key="item.label"
      class="gift"
      :style="{ backgroundImage: `url(${item.value})` }"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { giftList } from './source.config';
</script>

<style scoped>
.b-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  height: 100%;
  margin: 0 12px;
}
.gift {
  position: relative;
  width: 36px;
  height: 36px;
  /* background-image: url("./../../assets/flour.webp"); */
  background-size: cover;
  border-radius: 2px;
}

.gift + .gift {
  margin-left: 16px;
}
</style>
