// Web shim for @tauri-apps/plugin-clipboard-manager
// Replaces Tauri clipboard plugin with Web Clipboard API

export async function writeText(text: string): Promise<void> {
  await navigator.clipboard.writeText(text);
}

export async function readText(): Promise<string> {
  return await navigator.clipboard.readText();
}
