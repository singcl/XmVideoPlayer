import pinyinMatch from 'pinyin-match';

export function getQueryObj(url: string) {
  const searchParams = new URLSearchParams(new URL(url).search);
  const query: Record<string, unknown> = {};
  for (const [key, value] of searchParams.entries()) {
    query[key] = value;
  }
  return query;
}

export function download(data: ArrayBuffer, fileName: string, contentType?: string) {
  const elink = document.createElement('a');
  elink.download = fileName;
  elink.style.display = 'none';
  const blob = new Blob([data], {
    type: contentType || 'application/octet-stream',
  });
  elink.href = URL.createObjectURL(blob);
  document.body.appendChild(elink);
  elink.click();
  document.body.removeChild(elink);
}

export function decodeURIComponentIgnoreError(v: string) {
  try {
    return decodeURIComponent(v);
  } catch (error) {
    return v;
  }
}

export function decodeURL(v: string, prot = 'stream') {
  return stripPrefix(decodeURIComponentIgnoreError(v), prot);
}

function stripPrefix(input: string, prot = 'stream') {
  const reg = new RegExp(`(^https://${prot}.localhost/)|(^${prot}://localhost/)`, 'g');
  return input.replace(reg, '');
}

//
export function checkPinYin(name: string, keyword: string) {
  const matchRes = pinyinMatch.match(name, keyword);
  return typeof matchRes === 'object' ? matchRes.length > 0 : matchRes;
}
