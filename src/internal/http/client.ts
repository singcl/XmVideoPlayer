// /* eslint-disable @typescript-eslint/no-unsafe-return */
// /* eslint-disable @typescript-eslint/no-unsafe-member-access */
// /* eslint-disable @typescript-eslint/no-unsafe-assignment */

import { Message } from '@arco-design/web-vue';
import { getClient, Body, ResponseType, RequestOptions } from '@tauri-apps/api/http';

interface XmServerResponse<T> {
  code: number;
  msg: string;
  data?: T;
}

interface XmClientOptions {
  baseURL?: string;
}

export class XmClient {
  public static rsClient: Await<ReturnType<typeof getClient>>;

  private constructor(private options?: XmClientOptions) {
    if (!XmClient.rsClient) {
      const error = new Error('Error - Please use XmClient.create() create XmClient instance');
      error.cause = 'build_class_instance';
      throw error;
    }
  }

  // TODO
  // public request() {}

  public async post<T = unknown>(url: string, body?: Record<any, any>, options?: RequestOptions) {
    let mergeUrl = url;
    const baseURL = this.options?.baseURL;
    if (baseURL) mergeUrl = `${baseURL}/${mergeUrl}`;
    const response = await XmClient.rsClient.post<XmServerResponse<T>>(
      mergeUrl,
      body ? Body.json(body) : undefined,
      Object.assign({ responseType: ResponseType.JSON }, options)
    );
    const resData = response.data;
    if (![200, 204, 206].includes(response.status) || resData.code !== 0) {
      const message = resData.msg;
      Message.error(message || '好像失败了哦~');
      return Promise.reject(response.data);
    }
    return resData.data;
  }

  public static async create(options?: XmClientOptions) {
    if (!XmClient.rsClient) {
      XmClient.rsClient = await getClient();
    }
    return new XmClient(options);
  }
}

//
export const xmCl = XmClient.create({ baseURL: import.meta.env.VITE_APP_XM_VIDEO_BASE_URL });
