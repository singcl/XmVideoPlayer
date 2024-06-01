import { createRouter, createWebHistory } from 'vue-router';
const HomeView = () => import('@/views/Home.vue');

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/media/x-player',
      name: 'x-player',
      component: () => import('@/views/media/XPlayer.vue'),
    },
  ],
});

export default router;
