<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import XmPlayer from "./components/Player/XmPlayer.vue";
import Banner from "./components/Banner.vue";
// import Description from "./components/Description.vue";
</script>

<template>
  <div class="container">
    <!-- <h1>Welcome to Tauri!</h1> -->
    <Banner />
    <!-- <Description /> -->
    <XmPlayer />
    <!-- <Greet /> -->
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
