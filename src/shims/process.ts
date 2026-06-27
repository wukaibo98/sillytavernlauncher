// Web shim for @tauri-apps/plugin-process
// Replaces Tauri process plugin with Web APIs

export async function exit(exitCode: number = 0): Promise<void> {
  // In web mode, we can't really exit the process
  // Just log and close the window
  console.log(`Process exit requested with code: ${exitCode}`);
  window.close();
}

export async function relaunch(): Promise<void> {
  // In web mode, just reload the page
  window.location.reload();
}
