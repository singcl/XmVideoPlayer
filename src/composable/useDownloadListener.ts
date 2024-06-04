import { appWindow } from '@tauri-apps/api/window';
import { computed, onMounted, ref } from 'vue';

export function useDownloadListener() {
  const downloadPayload = ref<PayloadDownloadFed>({
    downloadType: 'm3u8',
    message: '',
    total: 0,
    current: 1,
  });

  const percent = computed(() => {
    return Number(Number(downloadPayload.value.current / downloadPayload.value.total).toFixed(2));
  });

  const ballShow = computed(() => percent.value > 0 && percent.value < 1);

  onMounted(async () => {
    const unListener = await appWindow.listen<PayloadDownload>('download', (e) => {
      console.log('Download ball:', e.payload);
      downloadPayload.value = {
        ...e.payload,
        total: Number(e.payload.total),
        current: Number(e.payload.current),
        message: e.payload.message.replace(/━/g, ''),
      };
    });
    return () => {
      unListener();
    };
  });

  return {
    downloadPayload,
    percent,
    ballShow,
  };
}
