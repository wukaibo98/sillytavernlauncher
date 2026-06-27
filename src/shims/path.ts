// Web shim for @tauri-apps/api/path
// Replaces Tauri path API with API calls or static paths

export async function resourceDir(): Promise<string> {
  // In web mode, resources are served from the root
  return '';
}

export async function appDataDir(): Promise<string> {
  return '/api/app-data-dir';
}

export async function appLocalDataDir(): Promise<string> {
  return '/api/app-local-data-dir';
}

export async function appConfigDir(): Promise<string> {
  return '/api/app-config-dir';
}

export async function appCacheDir(): Promise<string> {
  return '/api/app-cache-dir';
}

export async function appLogDir(): Promise<string> {
  return '/api/app-log-dir';
}

export async function desktopDir(): Promise<string> {
  return '/api/desktop-dir';
}

export async function documentDir(): Promise<string> {
  return '/api/document-dir';
}

export async function downloadDir(): Promise<string> {
  return '/api/download-dir';
}

export async function homeDir(): Promise<string> {
  return '/api/home-dir';
}

export async function tempDir(): Promise<string> {
  return '/api/temp-dir';
}

export async function sep(): Promise<string> {
  return '/';
}
