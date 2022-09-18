<template>
  <div>
    <div class="banner">
      <slot></slot>
    </div>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.banner {
  width: 100%;
  height: 56px;
  background-image: url('./../../assets/b-bg.webp');
  background-size: cover;
}
</style>
