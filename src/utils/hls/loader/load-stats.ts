import type { HlsPerformanceTiming, HlsProgressivePerformanceTiming, LoaderStats } from '../types/loader';

export class LoadStats implements LoaderStats {
  aborted = false;
  loaded = 0;
  retry = 0;
  total = 0;
  chunkCount = 0;
  bwEstimate = 0;
  loading: HlsProgressivePerformanceTiming = { start: 0, first: 0, end: 0 };
  parsing: HlsPerformanceTiming = { start: 0, end: 0 };
  buffering: HlsProgressivePerformanceTiming = { start: 0, first: 0, end: 0 };
}
