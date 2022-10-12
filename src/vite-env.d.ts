/// <reference types="vite/client" />
declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  const component: DefineComponent<Record<string, unknown>, Record<string, unknown>, any>;
  export default component;
}

declare module '@singcl/dplayer' {
  import DPlayer from '@types/dplayer';
  export default DPlayer;
}

interface ImportMetaEnv {
  readonly VITE_APP_TITLE: string;
  readonly VITE_APP_NAME: string;
}

// interface ImportMeta {
//   readonly env: ImportMetaEnv
// }
