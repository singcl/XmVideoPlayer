interface Options {
  costTime?: number;
  errorcode?: number;
  status: 0 | 1;
  msg?: string;
}

interface ServerTableData<T extends Array<unknown>> extends CamelPage {
  total: number;
  list: T;
}

export class CamelPage {
  constructor(public pageNo = 0, public pageSize = 20) {}
  static default(pageNo?: number, pageSize?: number) {
    return new CamelPage(pageNo, pageSize);
  }
}

abstract class ServerResponseBase<T> {
  costTime = 0;
  result: { errorcode?: number; status: 0 | 1; msg?: string } = {
    errorcode: 0,
    status: 1,
  };
  constructor(public data: T, options?: Options) {
    this.data = data;
    if (options) {
      const { costTime, errorcode, status, msg } = options;
      this.costTime = costTime ?? 0;
      this.result = {
        errorcode: errorcode ?? 0,
        status: status ?? 1,
        msg,
      };
    }
  }
}

export class ServerResponse<T> extends ServerResponseBase<T> {
  constructor(public data: T, options?: Options) {
    super(data, options);
  }

  static default<U>(data: U, options?: Options) {
    return new ServerResponse(data, options);
  }
}

export class ServerTableResponse<
  D extends Array<unknown> = Array<unknown>,
  T extends ServerTableData<D> = ServerTableData<D>
> extends ServerResponseBase<T> {
  constructor(data: T, options?: Options) {
    super(data, options);
  }
  static default<D extends Array<unknown> = Array<unknown>, T extends ServerTableData<D> = ServerTableData<D>>(
    data: T,
    options?: Options
  ) {
    return new ServerTableResponse(data, options);
  }
}
