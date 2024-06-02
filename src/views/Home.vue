<template>
  <div class="container">
    <h1 class="home-title">
      <img src="/logo.svg" class="logo vite" alt="XmVideoPlayer logo" />
      <span class="txt typing">{{ APP_TITLE }}</span>
    </h1>
    <XmSearch />
    <a-list
      ref="listRef"
      class="his-list-action-layout"
      :bordered="false"
      :data="dataSource?.list || []"
      :scrollbar="true"
      :virtual-list-props="{ height: 'calc(100vh - 160px)' }"
    >
      <template #item="{ item }">
        <a-list-item class="his-list-item" action-layout="vertical">
          <template #actions>
            <span><icon-heart />83</span>
            <span><icon-star />{{ item.index }}</span>
            <span><icon-message />Reply</span>
          </template>
          <!-- <template #extra>
            <div className="image-area">
              <img alt="arco-design" :src="item.imageSrc" />
            </div>
          </template> -->
          <a-list-item-meta :title="item.name" :description="item.url">
            <template #avatar>
              <a-avatar shape="square">
                <!-- <img alt="avatar" :src="item.name" /> -->
                {{ item.name?.slice(0, 1) }}
              </a-avatar>
            </template>
          </a-list-item-meta>
        </a-list-item>
      </template>
    </a-list>
  </div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const APP_TITLE = import.meta.env.VITE_APP_TITLE;

import API from '@/api';
import { useObservable, from } from '@vueuse/rxjs';
import { liveQuery } from 'dexie';
import { onMounted, reactive, ref } from 'vue';
//
const dataSource = useObservable(
  from(
    liveQuery(async () => {
      const r = await API.idb.getPlayerHistoryPageList();
      return r.data;
    })
  )
);
const paginationProps = reactive({
  defaultPageSize: 3,
  total: dataSource.value?.list.length,
});
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

<style scoped>
.his-list-action-layout .image-area {
  overflow: hidden;
  width: 183px;
  height: 119px;
  border-radius: 2px;
}

.his-list-action-layout .his-list-item {
  padding: 20px 0;
  border-bottom: 1px solid var(--xm-color-border-3);
  text-align: left;
  word-break: break-all;
}

.his-list-action-layout .image-area img {
  width: 100%;
}

.his-list-action-layout .arco-list-item-action .arco-icon {
  margin: 0 4px;
}
</style>
