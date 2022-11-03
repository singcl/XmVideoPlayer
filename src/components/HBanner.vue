<template>
  <div class="banner">
    <span class="address">{{ address }}</span>
    <span class="address"
      ><a href="https://github.com/singcl/XmVideoPlayer" target="_blank">{{ appName }}@v{{ version }}</a></span
    >
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

<script setup lang="ts">
import { ref } from 'vue';
import { getTauriVersion, getVersion, getName } from '@tauri-apps/api/app';
// import pkg from '@package';

const address = window.location.href;
const appName = ref<string>();
const version = ref<string>();

(async function () {
  appName.value = await getName();
  version.value = await getVersion();
})();
</script>

<style scoped>
.banner {
  position: fixed;
  bottom: 0;
  display: flex;
  width: var(--area-width);
  max-width: var(--area-width-max);
  align-items: center;
  justify-content: flex-end;
  margin: 0 auto;
  font-size: 10px;
}

.banner a {
  display: inline-block;
}

.banner .address {
  padding-right: 5px;
  color: #666;
}

.logo {
  display: block;
  width: 12px;
  height: 12px;
  padding: 5px;
}
</style>
