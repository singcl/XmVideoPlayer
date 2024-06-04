import { /* convertFileSrc */ invoke } from '@tauri-apps/api/tauri';
import { downloadDir } from '@tauri-apps/api/path';
import { save /*open*/ } from '@tauri-apps/api/dialog';
import { Notification } from '@arco-design/web-vue';
import { ref } from 'vue';
// import { checkM3U8Url } from '@/utils/validator';

export function useDownloadM3u8() {
  const loading = ref(false);
  async function download(mediaUrl: string) {
    try {
      const downloadDirPath = await downloadDir();
      const filePath = await save({
        filters: [
          {
            name: '视频',
            extensions: ['mp4', 'ts'],
          },
          // {
          //   name: '图片',
          //   extensions: ['png', 'jpg', 'jpeg'],
          // },
        ],
        // directory: true,
        defaultPath: downloadDirPath,
      });
      if (!filePath) return;
      console.log('------', filePath);
      loading.value = true;
      const res = await invoke('m3u8_download', {
        m3u8Url: mediaUrl,
        savePath: filePath,
      });
      console.log('------', res);
      Notification.success({
        title: '结果',
        content: '下载成功！🎉',
        duration: 3000,
      });
    } finally {
      loading.value = false;
    }
  }

  return {
    loading,
    download,
  };
}
