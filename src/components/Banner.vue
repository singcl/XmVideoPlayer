<template>
  <div class="banner">
    <span class="address">https://github.com/singcl</span>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./../assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.banner {
  position: fixed;
  bottom: 0;
  display: flex;
  justify-content: flex-end;
  align-items: center;
  margin: 0 auto;
  width: var(--area-width);
  max-width: var(--area-width-max);
}
.banner a {
  display: inline-block;
}

.banner .address {
  color: #666;
}
.logo {
  width: 18px;
  height: 18px;
  padding: 5px;
  display: block;
}
</style>
