<template>
  <div class="container">
    <h1 class="home-title">
      <img src="/logo.svg" class="logo vite" alt="XmVideoPlayer logo" />
      <span class="txt typing">{{ APP_TITLE }}</span>
    </h1>
    <!-- <HDescription /> -->
    <XmPlayer />
    <!-- <HTest /> -->
    <!-- <HGreet /> -->
    <router-link :to="{ name: 'home' }">跳转</router-link>
  </div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import HGreet from "./components/HGreet.vue";
// import XmPlayer from '../components/Player/XmPlayer.vue';
// import HBanner from '../components/HBanner.vue';
// import XmFeedback from '../components/XmFeedback.vue';
// import HDescription from "../components/HDescription.vue";
const APP_TITLE = import.meta.env.VITE_APP_TITLE;
</script>

<style scoped>
.container {
  /* padding-top: 10vh; */
  display: flex;
  width: var(--area-width);
  max-width: var(--area-width-max);
  height: 100vh;
  flex-direction: column;
  justify-content: flex-start;
  margin: 0 auto;
  background-color: rgb(0 0 0 / 1%);
  text-align: center;
}

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
  height: 36px;
  margin-left: 5px;
  line-height: 36px;
  text-align: left;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
