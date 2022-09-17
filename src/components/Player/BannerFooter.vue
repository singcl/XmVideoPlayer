<template>
  <div class="b-footer">
    <div
      class="gift"
      v-for="item in giftList"
      :key="item.label"
      :style="{ backgroundImage: `url(${item.value})` }"
    ></div>
  </div>
</template>

<script setup lang="ts">
import flour from "./../../assets/flour.webp";
import redbag from "./../../assets/redbag.webp";
import car from "./../../assets/car.webp";
import xj from "./../../assets/xj.webp";
const giftList = [
  {
    label: "鲜花",
    value: flour,
  },
  {
    label: "红包",
    value: redbag,
  },
  {
    label: "跑车",
    value: car,
  },
  {
    label: "仙境",
    value: xj,
  },
];
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
