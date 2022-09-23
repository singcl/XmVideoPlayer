import type { HlsUrlParameters } from './level';

// TODO: 简单摘录部分类型，类型定义不完整
interface Part {
  readonly fragOffset: number;
  readonly duration: number;
  readonly gap: boolean;
  readonly independent: boolean;
  readonly relurl: string;
  readonly fragment: Fragment;
  readonly index: number;
  start(): number;
  end(): number;
  loaded(): boolean;
}

interface Fragment {
  rawProgramDateTime: string | null;
  programDateTime: number | null;
  tagList: Array<string[]>;

  // EXTINF has to be present for a m3u8 to be considered valid
  duration: number;
  // sn notates the sequence number for a segment, and if set to a string can be 'initSegment'
  sn: number | 'initSegment';
  // levelkey is the EXT-X-KEY that applies to this segment for decryption
  // core difference from the private field _decryptdata is the lack of the initialized IV
  // _decryptdata will set the IV for this segment based on the segment number in the fragment
  levelkey?: string;
  // A string representing the fragment type
  readonly type: PlaylistLevelType;
  // A reference to the loader. Set while the fragment is loading, and removed afterwards. Used to abort fragment loading
  loader: Loader<FragmentLoaderContext> | null;
  // The level/track index to which the fragment belongs
  level: number;
  // The continuity counter of the fragment
  cc: number;
  // The starting Presentation Time Stamp (PTS) of the fragment. Set after transmux complete.
  startPTS?: number;
  // The ending Presentation Time Stamp (PTS) of the fragment. Set after transmux complete.
  endPTS?: number;
  // The latest Presentation Time Stamp (PTS) appended to the buffer.
  appendedPTS?: number;
  // The starting Decode Time Stamp (DTS) of the fragment. Set after transmux complete.
  startDTS: number;
  // The ending Decode Time Stamp (DTS) of the fragment. Set after transmux complete.
  endDTS: number;
  // The start time of the fragment, as listed in the manifest. Updated after transmux complete.
  start: number;
  // Set by `updateFragPTSDTS` in level-helper
  deltaPTS?: number;
  // The maximum starting Presentation Time Stamp (audio/video PTS) of the fragment. Set after transmux complete.
  maxStartPTS?: number;
  // The minimum ending Presentation Time Stamp (audio/video PTS) of the fragment. Set after transmux complete.
  minEndPTS?: number;
  urlId: number;
  data?: Uint8Array;
  // A flag indicating whether the segment was downloaded in order to test bitrate, and was not buffered
  bitrateTest: boolean;
  // #EXTINF  segment title
  title: string | null;
  // The Media Initialization Section for this segment
  initSegment: Fragment | null;
  decryptdata(): void;
}

interface LevelDetails {
  PTSKnown: boolean;
  alignedSliding: boolean;
  averagetargetduration?: number;
  endCC: number;
  endSN: number;
  fragments: Fragment[];
  fragmentHint?: Fragment;
  partList: Part[] | null;
  dateRanges: Record<string, unknown>;
  live: boolean;
  ageHeader: number;
  advancedDateTime?: number;
  updated: boolean;
  advanced: boolean;
  availabilityDelay?: number; // Manifest reload synchronization
  misses: number;
  needSidxRanges: boolean;
  startCC: number;
  startSN: number;
  startTimeOffset: number | null;
  targetduration: number;
  totalduration: number;
  type: string | null;
  url: string;
  m3u8: string;
  version: number | null;
  canBlockReload: boolean;
  canSkipUntil: number;
  canSkipDateRanges: boolean;
  skippedSegments: number;
  recentlyRemovedDateranges?: string[];
  partHoldBack: number;
  holdBack: number;
  partTarget: number;
  // preloadHint?: AttrList;
  // renditionReports?: AttrList[];
  tuneInGoal: number;
  deltaUpdateFailed?: boolean;
  driftStartTime: number;
  driftEndTime: number;
  driftStart: number;
  driftEnd: number;
  reloaded(previous: LevelDetails | undefined): void;
}

export interface LoaderContext {
  // target URL
  url: string;
  // loader response type (arraybuffer or default response type for playlist)
  responseType: string;
  // headers
  headers?: Record<string, string>;
  // start byte range offset
  rangeStart?: number;
  // end byte range offset
  rangeEnd?: number;
  // true if onProgress should report partial chunk of loaded content
  progressData?: boolean;
}

export interface FragmentLoaderContext extends LoaderContext {
  frag: Fragment;
  part: Part | null;
}

export interface LoaderConfiguration {
  // Max number of load retries
  maxRetry: number;
  // Timeout after which `onTimeOut` callback will be triggered
  // (if loading is still not finished after that delay)
  timeout: number;
  // Delay between an I/O error and following connection retry (ms).
  // This to avoid spamming the server
  retryDelay: number;
  // max connection retry delay (ms)
  maxRetryDelay: number;
  // When streaming progressively, this is the minimum chunk size required to emit a PROGRESS event
  highWaterMark: number;
}

export interface LoaderResponse {
  url: string;
  data: string | ArrayBuffer;
}

export interface LoaderStats {
  aborted: boolean;
  loaded: number;
  retry: number;
  total: number;
  chunkCount: number;
  bwEstimate: number;
  loading: HlsProgressivePerformanceTiming;
  parsing: HlsPerformanceTiming;
  buffering: HlsProgressivePerformanceTiming;
}

export interface HlsPerformanceTiming {
  start: number;
  end: number;
}

export interface HlsChunkPerformanceTiming extends HlsPerformanceTiming {
  executeStart: number;
  executeEnd: number;
}

export interface HlsProgressivePerformanceTiming extends HlsPerformanceTiming {
  first: number;
}

export type LoaderOnSuccess<T extends LoaderContext> = (
  response: LoaderResponse,
  stats: LoaderStats,
  context: T,
  networkDetails: any
) => void;

export type LoaderOnProgress<T extends LoaderContext> = (
  stats: LoaderStats,
  context: T,
  data: string | ArrayBuffer,
  networkDetails: any
) => void;

export type LoaderOnError<T extends LoaderContext> = (
  error: {
    // error status code
    code: number;
    // error description
    text: string | null;
  },
  context: T,
  networkDetails: any
) => void;

export type LoaderOnTimeout<T extends LoaderContext> = (stats: LoaderStats, context: T, networkDetails: any) => void;

export type LoaderOnAbort<T extends LoaderContext> = (stats: LoaderStats, context: T, networkDetails: any) => void;

export interface LoaderCallbacks<T extends LoaderContext> {
  onSuccess: LoaderOnSuccess<T>;
  onError: LoaderOnError<T>;
  onTimeout: LoaderOnTimeout<T>;
  onAbort?: LoaderOnAbort<T>;
  onProgress?: LoaderOnProgress<T>;
}

export interface Loader<T extends LoaderContext> {
  destroy(): void;
  abort(): void;
  load(context: LoaderContext, config: LoaderConfiguration, callbacks: LoaderCallbacks<T>): void;
  /**
   * `getCacheAge()` is called by hls.js to get the duration that a given object
   * has been sitting in a cache proxy when playing live.  If implemented,
   * this should return a value in seconds.
   *
   * For HTTP based loaders, this should return the contents of the "age" header.
   *
   * @returns time object being lodaded
   */
  getCacheAge?: () => number | null;
  context: T;
  stats: LoaderStats;
}

export enum PlaylistContextType {
  MANIFEST = 'manifest',
  LEVEL = 'level',
  AUDIO_TRACK = 'audioTrack',
  SUBTITLE_TRACK = 'subtitleTrack',
}

export enum PlaylistLevelType {
  MAIN = 'main',
  AUDIO = 'audio',
  SUBTITLE = 'subtitle',
}

export interface PlaylistLoaderContext extends LoaderContext {
  loader?: Loader<PlaylistLoaderContext>;

  type: PlaylistContextType;
  // the level index to load
  level: number | null;
  // level or track id from LevelLoadingData / TrackLoadingData
  id: number | null;
  // track group id
  groupId: string | null;
  // defines if the loader is handling a sidx request for the playlist
  isSidxRequest?: boolean;
  // internal representation of a parsed m3u8 level playlist
  levelDetails?: LevelDetails;
  // Blocking playlist request delivery directives (or null id none were added to playlist url
  deliveryDirectives: HlsUrlParameters | null;
}
