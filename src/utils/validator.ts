export function checkM3U8Url(url?: string) {
  if (!url) return false;
  return /^https?:\/\/.+(\.)?m3u8(\.php)?(\?(.*))?$/.test(url);
}

export function checkBuffer(data: string | ArrayBuffer, checked: boolean): data is ArrayBuffer {
  return checked;
}
