// Web shim for @tauri-apps/api/core
// Replaces Tauri invoke() with HTTP POST to axum API

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const resp = await fetch(`/api/${cmd}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(args || {}),
  });
  if (!resp.ok) {
    const text = await resp.text().catch(() => '');
    throw new Error(`invoke ${cmd} failed: ${resp.status} ${text}`);
  }
  return resp.json();
}
