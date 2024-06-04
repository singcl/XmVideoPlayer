/// <reference types="lib.dom.d.ts" />

declare interface EventTarget {
  value: any;
}

type UnArray<T> = T extends Array<infer U> ? U : any;
interface PageCamels {
  pageNo: number;
  pageSize: number;
}
interface PageSnake {
  page_no: number;
  page_size: number;
}

interface TableRecordData<T> extends PageSnake {
  total_data: Array<T>;
  total_count: number;
}
interface TableAllData<S> {
  recordData: TableRecordData<S>;
  totalData: Record<string, any>;
}

// 一般接口返回数据结构
interface ServerResponse<T> {
  costTime: number;
  result: {
    errorcode: number;
    status: 0 | 1;
    msg?: string;
  };
  data: T;
}

// 表格返回数据类型
interface ServerTableResponse<S = Record<string, any>> extends ServerResponse<TableAllData<S>> {}

// 表格返回数据类型 （不含合计）
interface ServerTableV2Response<S = Record<string, any>> extends ServerResponse<TableRecordData<S>> {}

type Await<T> = T extends Promise<infer U> ? U : T;

// XM 一般接口返回数据结构
declare interface XmServerResponse<T> {
  code: number;
  msg: string;
  data?: T;
}

declare interface PayloadDownload {
  downloadType: string;
  message: string;
  total: string;
  current: string;
}

declare interface PayloadDownloadFed {
  downloadType: string;
  message: string;
  total: number;
  current: number;
}
