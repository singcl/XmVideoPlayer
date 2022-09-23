export enum HlsSkip {
  No = '',
  Yes = 'YES',
  v2 = 'v2',
}

export class HlsUrlParameters {
  msn?: number;
  part?: number;
  skip?: HlsSkip;

  constructor(msn?: number, part?: number, skip?: HlsSkip) {
    this.msn = msn;
    this.part = part;
    this.skip = skip;
  }

  addDirectives(uri: string): string | never {
    const url: URL = new self.URL(uri);
    if (this.msn !== undefined) {
      url.searchParams.set('_HLS_msn', this.msn.toString());
    }
    if (this.part !== undefined) {
      url.searchParams.set('_HLS_part', this.part.toString());
    }
    if (this.skip) {
      url.searchParams.set('_HLS_skip', this.skip);
    }
    return url.toString();
  }
}
