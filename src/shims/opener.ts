// Web shim for @tauri-apps/plugin-opener
// Replaces Tauri opener plugin with Web APIs

export async function openUrl(url: string): Promise<void> {
  window.open(url, '_blank');
}

export async function openPath(path: string): Promise<void> {
  window.open(path, '_blank');
}
