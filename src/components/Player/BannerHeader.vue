<template>
  <div class="b-header">
    <div class="avatar">
      <div class="avatar-dec"></div>
    </div>
    <div class="op">
      <icon-heart class="op-ico" :size="24" :style="{ color: '#a5bcff' }" />
      <icon-thumb-up class="op-ico" :size="24" :style="{ color: '#a5bcff' }" />
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.b-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 100%;
  margin: 0 12px 0 4px;
}
.avatar {
  position: relative;
  width: 48px;
  height: 48px;
  background-image: url('./../../assets/avatar.webp');
  background-size: cover;
  border-radius: 2px;
}
.avatar-dec {
  position: relative;
  width: 100%;
  height: 100%;
  background-image: url('./../../assets/avatar-dec.webp');
  background-size: cover;
}

.op {
  display: flex;
  align-items: center;
}

.op-ico + .op-ico {
  margin-left: 8px;
}
</style>
