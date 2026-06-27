import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(), tailwindcss()],

  optimizeDeps: {
    entries: ['index.html'],
    exclude: ['data/*', 'src-tauri/*'],
  },
  resolve: {
    alias: {
      '@tauri-apps/api/core': path.resolve(__dirname, 'src/shims/core.ts'),
      '@tauri-apps/api/event': path.resolve(__dirname, 'src/shims/event.ts'),
      '@tauri-apps/api/window': path.resolve(__dirname, 'src/shims/window.ts'),
      '@tauri-apps/api/path': path.resolve(__dirname, 'src/shims/path.ts'),
      '@tauri-apps/plugin-dialog': path.resolve(__dirname, 'src/shims/dialog.ts'),
      '@tauri-apps/plugin-http': path.resolve(__dirname, 'src/shims/http.ts'),
      '@tauri-apps/plugin-opener': path.resolve(__dirname, 'src/shims/opener.ts'),
      '@tauri-apps/plugin-process': path.resolve(__dirname, 'src/shims/process.ts'),
      '@tauri-apps/plugin-clipboard-manager': path.resolve(__dirname, 'src/shims/clipboard.ts'),
      '@tauri-apps/plugin-updater': path.resolve(__dirname, 'src/shims/updater.ts'),
    },
  },
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      external: ['node:fs/promises', 'node:zlib'],
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**', '**/data/**'],
    },
  },
}))
