export function getQueryObj(url: string) {
  const searchParams = new URLSearchParams(new URL(url).search);
  const query: Record<string, unknown> = {};
  for (const [key, value] of searchParams.entries()) {
    query[key] = value;
  }
  return query;
}
