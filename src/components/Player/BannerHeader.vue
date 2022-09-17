<template>
  <div class="b-header">
    <div class="avatar">
      <div class="avatar-dec"></div>
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style scoped> 

.b-header {
  display: flex;
  align-items: center;
}
.avatar {
  position: relative;
  margin: 4px;
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
</style>