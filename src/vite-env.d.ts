/// <reference types="vite/client" />
declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, unknown>;
  export default component;
}

declare module '@singcl/dplayer' {
  import DPlayer from '@types/dplayer';
  export default DPlayer;
}

declare module '@/libs/*.js';

declare module '@singcl/throttle-debounce/debounce' {
  declare function debounce(delay: number, callback: () => void);
  declare function debounce(delay: number, atBegin: boolean, callback: () => void);

  export default debounce;
}

interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string;
  readonly VITE_APP_NAME: string;
  readonly VITE_APP_XM_VIDEO_BASE_URL: string;
}

// interface ImportMeta {
//   readonly env: ImportMetaEnv
// }

// 全局arco icon 类型提示修复
declare global {
  import '@arco-design/web-vue/es/icon';
}
