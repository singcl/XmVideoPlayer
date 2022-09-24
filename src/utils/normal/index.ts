import { fetch, ResponseType } from '@tauri-apps/api/http';
import { getQueryObj } from '@/utils/tools';

interface NormalConfig {
  url: string;
}

// 常规媒体资源加载
export default class Normal {
  private blobUrl = '';
  public url: string;
  //
  constructor(opt: NormalConfig) {
    const { url } = opt;
    this.url = url;
  }
  //
  private async blob() {
    const rustResponse = await fetch<ArrayBuffer>(this.url, {
      method: 'GET',
      responseType: ResponseType.Binary,
      timeout: 100,
      headers: {
        Referer: this.url,
        Range: 'bytes=0-5000000',
      },
      query: getQueryObj(this.url),
    });
    //
    const { data, status, headers, url } = rustResponse;
    console.log('----headers', headers);
    const response = new Response(new Uint8Array(data), {
      headers: new Headers(Object.assign({}, headers)),
      status,
      statusText: 'ok',
    });
    Object.defineProperty(response, 'url', { value: url });
    console.log('----response', response);
    return response.blob();
  }

  // 载入资源
  public load(video: HTMLMediaElement) {
    URL.revokeObjectURL(this.blobUrl);
    this.blob()
      .then((res) => {
        const blobUrl = URL.createObjectURL(res);
        video.src = blobUrl;
      })
      .catch((e) => console.log(e));
  }
  // 初始化
  static create(opt: NormalConfig) {
    return new Normal(opt);
  }
}
