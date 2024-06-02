<template>
  <div class="container">
    <h1 class="home-title">
      <img src="/logo.svg" class="logo vite" alt="XmVideoPlayer logo" />
      <span class="txt typing">{{ APP_TITLE }}</span>
    </h1>
    <XmListSearch v-model="searchKeyword" @submit="handleSubmit" />
    <a-list
      class="his-list-action-layout"
      :bordered="false"
      size="small"
      :data="dataSource || []"
      :scrollbar="false"
      :max-height="'calc(100vh - 160px)'"
      @reach-bottom="handleReachBottom"
    >
      <!-- :virtual-list-props="{ height: 'calc(100vh - 175px)' }" -->
      <template #scroll-loading>
        <div v-if="bottom">{{ dataSource.length }}/{{ page.total }} 没有更多数据啦(*^_^*)</div>
        <a-spin v-else-if="loading" />
      </template>
      <template #item="{ item }">
        <a-list-item class="his-list-item" action-layout="vertical">
          <template #actions>
            <span><icon-heart />0</span>
            <span><icon-star />{{ item.index }}</span>
            <span @click="handlePlay($event, item)"><icon-play-circle />Play</span>
            <span @click="handleOptEdit($event, item)"><icon-edit />Edit</span>
            <span @click="handleOptDelete($event, item)"> <icon-delete />Delete</span>
          </template>
          <a-list-item-meta :title="decodeURL(item.name)" :description="decodeURL(item.url)">
            <template #avatar>
              <a-avatar shape="square">
                {{ decodeURL(item.name)?.slice(0, 1) }}
              </a-avatar>
            </template>
          </a-list-item-meta>
        </a-list-item>
      </template>
    </a-list>
    <xm-history-edit-dialog v-model:visible="hisEditVisible" :data="hisEditData" @success="handleOptEditSuccess" />
  </div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
const APP_TITLE = import.meta.env.VITE_APP_TITLE;

import API from '@/api';
import { reactive, ref, h } from 'vue';
import { Modal, Message } from '@arco-design/web-vue';
import { decodeURL } from '@/utils/tools';
import { useRouter } from 'vue-router';
// import { useObservable, from } from '@vueuse/rxjs';
// import { liveQuery } from 'dexie';
// //
// const dataSource = useObservable(
//   from(
//     liveQuery(async () => {
//       const r = await API.idb.getPlayerHistoryPageList();
//       return r.data;
//     })
//   )
// );

type HList = Await<ReturnType<typeof API.idb.getPlayerHistoryPageList>>['data']['list'];
type HListItem = UnArray<HList>;

const router = useRouter();

const loading = ref(false);
const bottom = ref(false);
const page = reactive({ pageNo: 0, pageSize: 20, total: 0 });
const dataSource = ref<HList>([]);
const hisEditVisible = ref(false);
const hisEditData = ref<HListItem>();
const searchKeyword = ref('');
//
const fetchData = async (currPage = page.pageNo) => {
  try {
    loading.value = true;
    const {
      data: { list = [], pageNo, pageSize, total },
    } = await API.idb.getPlayerHistoryPageList({ page: { pageNo: currPage, pageSize: page.pageSize } });
    dataSource.value = dataSource.value.concat(list);
    page.total = total;
    page.pageNo = pageNo;
    page.pageSize = pageSize;
    bottom.value = pageNo >= Math.ceil(total / pageSize);
  } finally {
    loading.value = false;
  }
};

//
const handleReachBottom = () => {
  fetchData(page.pageNo + 1);
};

//
const handleReSearch = async () => {
  try {
    loading.value = true;
    const {
      data: { list = [], pageNo, pageSize, total },
    } = await API.idb.getPlayerHistoryPageList({ page: { pageNo: 1, pageSize: page.pageSize } });
    dataSource.value = list;
    page.total = total;
    page.pageNo = pageNo;
    page.pageSize = pageSize;
    bottom.value = pageNo >= Math.ceil(total / pageSize);
  } finally {
    loading.value = false;
  }
};

//
const handlePlay = (e: Event, opt: HListItem) => {
  e.stopPropagation();
  router.push({ name: 'x-player', params: { id: opt.id } });
};

//
// 删除播放记录
async function handleOptDelete(e: Event, opt: HListItem) {
  e.stopPropagation();
  Modal.confirm({
    title: '删除确认',
    titleAlign: 'start',
    content: () =>
      h('div', { style: 'word-break: break-all' }, [
        h('span', null, '确认删除'),
        h('span', { style: 'color: red; margin: 0 3px' }, decodeURL(opt.name)),
        h('span', null, '吗？'),
      ]),
    async onBeforeOk(done) {
      await API.idb.deletePlayerHistory(opt.id);
      await handleReSearch();
      Message.success('删除成功');
      done(true);
    },
  });
}

// 编辑播放记录
async function handleOptEdit(e: Event, opt: HListItem) {
  e.stopPropagation();
  hisEditVisible.value = true;
  hisEditData.value = opt;
}

//
async function handleOptEditSuccess() {
  await handleReSearch();
  Message.success('编辑成功');
  hisEditVisible.value = false;
}

// Submit
async function handleSubmit(opt?: { name: string; url: string }) {
  if (!opt) return Message.info({ content: '请输入正确的链接' });
  if (!/^(((ht|f)tps?):\/\/)?([^!@#$%^&*?.\s-]([^!@#$%^&*?.\s]{0,63}[^!@#$%^&*?.\s])?\.)+[a-z]{2,6}\/?/.test(opt.url))
    return Message.info({ content: '请输入正确的链接' });
  const { data } = await API.idb.savePlayerHistory({ name: opt.name, url: opt.url });
  router.push({ name: 'x-player', params: { id: data } });
  handleReSearch();
}
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
