import { createRouter, createWebHistory } from 'vue-router';
import Layout from '@/layout/Layout.vue';
const HomeView = () => import('@/views/Home.vue');

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: Layout,
      children: [
        {
          path: '/',
          name: 'home',
          component: HomeView,
        },
        {
          path: '/media/x-player/:id',
          name: 'x-player',
          component: () => import('@/views/media/XPlayer.vue'),
          props: true,
        },
      ],
    },
  ],
});

export default router;
