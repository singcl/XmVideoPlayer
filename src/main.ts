import { createApp } from 'vue';
import './style.css';
import App from './App.vue';
import '@/libs/heart.js';
import '@/libs/sakura.js';
import { pinia } from '@/stores';

import XmSvgIcon from '~virtual/svg-component';

const app = createApp(App);
app.use(pinia);
app.component(XmSvgIcon.name, XmSvgIcon);

app.mount('#app');
