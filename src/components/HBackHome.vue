<template>
  <div class="back-home">
    <a-space>
      <router-link :to="{ name: 'home' }">
        <a-button type="outline" shape="circle"><icon-home style="font-size: 20px" /></a-button>
      </router-link>
      <a-button v-if="route.name !== 'home'" type="outline" shape="circle" @click="handleBack"
        ><icon-left-circle style="font-size: 22px"
      /></a-button>
    </a-space>
  </div>
</template>

<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';
const route = useRoute();
const router = useRouter();
const handleBack = () => {
  router.back();
};
</script>

<style scoped>
.back-home {
  position: fixed;
  top: 15px;
  left: 50px;
}
</style>
