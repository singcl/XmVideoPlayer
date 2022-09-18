<template>
  <div class="banner">
    <span class="address"><a href="https://github.com/singcl/XmVideoPlayer" target="_blank">版本:v0.0.7</a></span>
    <span>Powered by</span>
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
  font-size: 10px;
}
.banner a {
  display: inline-block;
}

.banner .address {
  color: #666;
  padding-right: 5px;
}
.logo {
  width: 12px;
  height: 12px;
  padding: 5px;
  display: block;
}
</style>
