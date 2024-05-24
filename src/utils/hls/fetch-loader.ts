import {
  LoaderCallbacks,
  LoaderContext,
  Loader,
  LoaderStats,
  LoaderConfiguration,
  LoaderOnProgress,
} from './types/loader';
import { LoadStats } from './loader/load-stats';
import ChunkCache from './demux/chunk-cache';
import { fetch, ResponseType } from '@tauri-apps/api/http';
import { checkM3U8Url, checkBuffer } from '@/utils/validator';
import { getQueryObj } from '@/utils/tools';
import { HlsConfig } from 'hls.js';

interface FError extends Error {
  details?: Record<string, unknown>;
  code?: number;
}

export function fetchSupported() {
  if (self.AbortController && self.ReadableStream && self.Request) {
    try {
      new self.ReadableStream({}); // eslint-disable-line no-new
      return true;
    } catch (e) {
      /* noop */
    }
  }
  return false;
}

class FetchLoader implements Loader<LoaderContext> {
  private fetchSetup: (context: LoaderContext, initParams: Record<string, unknown>) => Request;
  private requestTimeout?: number;
  private request!: Request;
  private response!: Response;
  private controller: AbortController;
  public context!: LoaderContext;
  private config: LoaderConfiguration | null = null;
  private callbacks: LoaderCallbacks<LoaderContext> | null = null;
  public stats: LoaderStats;
  private loader: Response | null = null;

  constructor(config: HlsConfig) {
    this.fetchSetup = config.fetchSetup || getRequest;
    this.controller = new self.AbortController();
    this.stats = new LoadStats();
  }

  destroy(): void {
    this.loader = this.callbacks = null;
    this.abortInternal();
  }

  abortInternal(): void {
    const response = this.response;
    if (!response || !response.ok) {
      this.stats.aborted = true;
      this.controller.abort();
    }
  }

  abort(): void {
    this.abortInternal();
    if (this.callbacks?.onAbort) {
      this.callbacks.onAbort(this.stats, this.context, this.response);
    }
  }

  load(context: LoaderContext, config: LoaderConfiguration, callbacks: LoaderCallbacks<LoaderContext>): void {
    const stats = this.stats;
    if (stats.loading.start) {
      throw new Error('Loader can only be used once.');
    }
    stats.loading.start = self.performance.now();

    const initParams = getRequestParameters(context, this.controller.signal);
    const onProgress: LoaderOnProgress<LoaderContext> | undefined = callbacks.onProgress;
    const isArrayBuffer = context.responseType === 'arraybuffer';
    // const LENGTH = isArrayBuffer ? 'byteLength' : 'length';

    this.context = context;
    this.config = config;
    this.callbacks = callbacks;
    this.request = this.fetchSetup(context, initParams);
    self.clearTimeout(this.requestTimeout);
    this.requestTimeout = self.setTimeout(() => {
      this.abortInternal();
      callbacks.onTimeout(stats, context, this.response);
    }, config.timeout);

    const { url, method, headers } = this.request;

    // WEB API Request 类中无法设置内置headers属性
    const rustHeader: Record<string, unknown> = {
      'User-Agent':
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36',
    };
    headers.forEach((v, k) => (rustHeader[k] = v));
    fetch<string | ArrayBuffer>(url, {
      method: method as HttpVerb,
      headers: rustHeader,
      query: getQueryObj(url),
      timeout: 100,
      responseType: checkM3U8Url(url) ? ResponseType.Text : ResponseType.Binary,
    })
      .then((rustResponse): Promise<string | ArrayBuffer> => {
        // console.log('----RUST_RESPONSE----', rustResponse);
        const { data, headers, status, url } = rustResponse;
        const response = new Response(checkBuffer(data, isArrayBuffer) ? new Uint8Array(data) : data, {
          headers: new Headers(Object.assign({}, headers)),
          status,
          statusText: 'ok',
        });
        Object.defineProperty(response, 'url', { value: url });
        this.response = this.loader = response;

        if (!response.ok) {
          const { status, statusText } = response;
          throw new FetchError(statusText || 'fetch, bad network response', status, response);
        }
        stats.loading.first = Math.max(self.performance.now(), stats.loading.start);
        stats.total = parseInt(response.headers.get('Content-Length') || '0');

        if (onProgress && Number.isFinite(config.highWaterMark)) {
          return this.loadProgressively(response, stats, context, config.highWaterMark, onProgress);
        }

        if (isArrayBuffer) {
          return response.arrayBuffer();
        }
        return response.text();
      })
      .then((responseData: string | ArrayBuffer) => {
        const { response } = this;
        self.clearTimeout(this.requestTimeout);
        stats.loading.end = Math.max(self.performance.now(), stats.loading.first);
        stats.loaded = stats.total = checkBuffer(responseData, isArrayBuffer)
          ? responseData['byteLength']
          : responseData['length'];

        const loaderResponse = {
          url: response.url,
          data: responseData,
        };

        if (onProgress && !Number.isFinite(config.highWaterMark)) {
          onProgress(stats, context, responseData, response);
        }

        callbacks.onSuccess(loaderResponse, stats, context, response);
      })
      .catch((error: FError) => {
        console.error('-----RUST_FETCH_ERROR------', error);
        self.clearTimeout(this.requestTimeout);
        if (stats.aborted) {
          return;
        }
        // CORS errors result in an undefined code. Set it to 0 here to align with XHR's behavior
        // when destroying, 'error' itself can be undefined
        const code: number = !error ? 0 : error.code || 0;
        const text: string | null = !error ? null : error.message;
        callbacks.onError({ code, text }, context, error ? error.details : null);
      });
  }

  getCacheAge(): number | null {
    let result: number | null = null;
    if (this.response) {
      const ageHeader = this.response.headers.get('age');
      result = ageHeader ? parseFloat(ageHeader) : null;
    }
    return result;
  }

  private loadProgressively(
    response: Response,
    stats: LoaderStats,
    context: LoaderContext,
    highWaterMark = 0,
    onProgress: LoaderOnProgress<LoaderContext>
  ): Promise<ArrayBuffer> {
    const chunkCache = new ChunkCache();
    const reader = (response.body as ReadableStream<Uint8Array>).getReader();

    const pump = (): Promise<ArrayBuffer> => {
      return reader
        .read()
        .then((data) => {
          if (data.done) {
            if (chunkCache.dataLength) {
              onProgress(stats, context, chunkCache.flush(), response);
            }

            return Promise.resolve(new ArrayBuffer(0));
          }
          const chunk: Uint8Array = data.value;
          const len = chunk.length;
          stats.loaded += len;
          if (len < highWaterMark || chunkCache.dataLength) {
            // The current chunk is too small to to be emitted or the cache already has data
            // Push it to the cache
            chunkCache.push(chunk);
            if (chunkCache.dataLength >= highWaterMark) {
              // flush in order to join the typed arrays
              onProgress(stats, context, chunkCache.flush(), response);
            }
          } else {
            // If there's nothing cached already, and the chache is large enough
            // just emit the progress event
            onProgress(stats, context, chunk, response);
          }
          return pump();
        })
        .catch(() => {
          /* aborted */
          return Promise.reject();
        });
    };

    return pump();
  }
}

type HttpVerb = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH' | 'HEAD' | 'OPTIONS' | 'CONNECT' | 'TRACE';
interface FetchOptions extends Record<string, unknown> {
  method: HttpVerb;
  headers: Headers;
  query?: Record<string, unknown>;
  body?: Body;
  timeout?: number;
  responseType?: ResponseType;
  //
  mode?: string;
  credentials?: string;
  signal?: AbortSignal;
}
function getRequestParameters(context: LoaderContext, signal?: AbortSignal) {
  const initParams: FetchOptions = {
    method: 'GET',
    mode: 'cors',
    credentials: 'same-origin',
    responseType: ResponseType.Text,
    signal,
    headers: new self.Headers(Object.assign({}, context.headers)),
  };

  if (context.rangeEnd) {
    // eslint-disable-next-line @typescript-eslint/restrict-plus-operands
    initParams.headers.set('Range', 'bytes=' + context.rangeStart + '-' + String(context.rangeEnd - 1));
  }

  return initParams;
}

function getRequest(context: LoaderContext, initParams: Record<string, unknown>): Request {
  return new self.Request(context.url, initParams);
}

class FetchError extends Error {
  public code: number;
  public details: object;
  constructor(message: string, code: number, details: object) {
    super(message);
    this.code = code;
    this.details = details;
  }
}

export default FetchLoader;
