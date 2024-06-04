import { /* convertFileSrc */ invoke } from '@tauri-apps/api/tauri';
import { downloadDir } from '@tauri-apps/api/path';
import { save /*open*/ } from '@tauri-apps/api/dialog';
import { Notification } from '@arco-design/web-vue';
import { ref } from 'vue';
// import { checkM3U8Url } from '@/utils/validator';

export function useDownloadNormal() {
  const loading = ref(false);
  async function download(mediaUrl: string) {
    try {
      const downloadDirPath = await downloadDir();
      const filePath = await save({
        // TODO: 这个filters什么意思？？
        // filters: [
        //   {
        //     name: 'Video',
        //     extensions: ['mp4'],
        //   },
        //   {
        //     name: 'Image',
        //     extensions: ['png', 'jpg', 'jpeg'],
        //   },
        // ],
        defaultPath: downloadDirPath,
      });
      if (!filePath) return;
      console.log('------', filePath);
      loading.value = true;
      const res = await invoke('video_download', { url: mediaUrl, path: filePath });
      console.log('------', res);
      Notification.success({ content: '下载成功！' });
    } finally {
      loading.value = false;
    }
  }

  return {
    loading,
    download,
  };
}
