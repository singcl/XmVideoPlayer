<template>
  <div class="container">
    <h1 class="home-title">
      <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      <span class="txt">{{ APP_TITLE }}</span>
    </h1>
    <!-- <Description /> -->
    <XmPlayer />
    <Banner />
    <!-- <Greet /> -->
  </div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
import XmPlayer from "./components/Player/XmPlayer.vue";
import Banner from "./components/Banner.vue";
// import Description from "./components/Description.vue";
const APP_TITLE = import.meta.env.VITE_APP_TITLE;
</script>

<style scoped>
.home-title {
  display: flex;
  align-items: center;
  margin: 12px auto;
}

.home-title .logo {
  width: 32px;
  height: 32px;
  padding: 2px;
}

.home-title .txt {
  margin-left: 5px;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
