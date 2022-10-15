interface Options {
  costTime?: number;
  errorcode?: number;
  status: 0 | 1;
  msg?: string;
}
export class ServerResponse<T> {
  costTime = 0;
  result: { errorcode?: number; status: 0 | 1; msg?: string } = {
    errorcode: 0,
    status: 0,
  };

  constructor(public data: T, options?: Options) {
    this.data = data;
    if (options) {
      const { costTime, errorcode, status, msg } = options;
      this.costTime = costTime ?? 0;
      this.result = {
        errorcode: errorcode ?? 0,
        status: status ?? 0,
        msg,
      };
    }
  }

  static default<T>(data: T) {
    return new ServerResponse(data);
  }
}
