// Web shim for @tauri-apps/plugin-updater
// Replaces Tauri updater plugin with a simple version check

export interface Update {
  version: string;
  date: string;
  body: string;
  url: string;
}

export async function check(): Promise<Update | null> {
  try {
    const resp = await fetch('/api/check-update');
    if (resp.ok) {
      return await resp.json();
    }
    return null;
  } catch {
    return null;
  }
}
