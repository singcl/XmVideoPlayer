<template>
  <div class="container">
    <h1 class="home-title">
      <img src="/logo.svg" class="logo vite" alt="XmVideoPlayer logo" />
      <span class="txt typing">{{ APP_TITLE }}</span>
    </h1>
    <XmSearch />
    <a-list
      class="list-demo-action-layout"
      :bordered="false"
      :data="dataSource"
      :pagination-props="paginationProps"
      :scrollbar="true"
      :max-height="400"
    >
      <template #item="{ item }">
        <a-list-item class="list-demo-item" action-layout="vertical">
          <template #actions>
            <span><icon-heart />83</span>
            <span><icon-star />{{ item.index }}</span>
            <span><icon-message />Reply</span>
          </template>
          <template #extra>
            <div className="image-area">
              <img alt="arco-design" :src="item.imageSrc" />
            </div>
          </template>
          <a-list-item-meta :title="item.title" :description="item.description">
            <template #avatar>
              <a-avatar shape="square">
                <img alt="avatar" :src="item.avatar" />
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

import { reactive } from 'vue';

const names = ['Socrates', 'Balzac', 'Plato'];
const avatarSrc = [
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/a8c8cdb109cb051163646151a4a5083b.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/e278888093bef8910e829486fb45dd69.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/9eeb1800d9b78349b24682c3518ac4a3.png~tplv-uwbnlip3yd-webp.webp',
];
const imageSrc = [
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/29c1f9d7d17c503c5d7bf4e538cb7c4f.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/04d7bc31dd67dcdf380bc3f6aa07599f.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/1f61854a849a076318ed527c8fca1bbf.png~tplv-uwbnlip3yd-webp.webp',
];
const dataSource = new Array(15).fill(null).map((_, index) => {
  return {
    index: index,
    avatar: avatarSrc[index % avatarSrc.length],
    title: names[index % names.length],
    description:
      'Beijing ByteDance Technology Co., Ltd. is an enterprise located in China. ByteDance has products such as TikTok, Toutiao, volcano video and Douyin (the Chinese version of TikTok).',
    imageSrc: imageSrc[index % imageSrc.length],
  };
});
const paginationProps = reactive({
  defaultPageSize: 3,
  total: dataSource.length,
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
.list-demo-action-layout .image-area {
  overflow: hidden;
  width: 183px;
  height: 119px;
  border-radius: 2px;
}

.list-demo-action-layout .list-demo-item {
  padding: 20px 0;
  border-bottom: 1px solid var(--color-fill-3);
}

.list-demo-action-layout .image-area img {
  width: 100%;
}

.list-demo-action-layout .arco-list-item-action .arco-icon {
  margin: 0 4px;
}
</style>
