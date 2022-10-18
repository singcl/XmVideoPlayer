import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import '@/libs/heart.js';
import '@/libs/sakura.js';
import { pinia } from '@/stores';

const app = createApp(App);
app.use(pinia);

app.mount('#app');
