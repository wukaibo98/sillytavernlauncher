// Web shim for @tauri-apps/plugin-http
// Replaces Tauri HTTP plugin with native fetch

export type RequestInit = globalThis.RequestInit;

export interface FetchResponse<T> {
  data: T;
  status: number;
  headers: Headers;
  url: string;
  ok: boolean;
}

export async function fetch<T = unknown>(
  url: string,
  options?: globalThis.RequestInit
): Promise<FetchResponse<T>> {
  const resp = await window.fetch(url, options);
  const data = await resp.json() as T;
  return {
    data,
    status: resp.status,
    headers: resp.headers,
    url: resp.url,
    ok: resp.ok,
  };
}
