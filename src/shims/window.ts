// Web shim for @tauri-apps/api/window
// Replaces Tauri window API with Web APIs

interface WindowManager {
  onCloseRequested: (cb: (event: { preventDefault: () => void }) => void) => void;
  close: () => void;
  setFocus: () => void;
  minimize: () => void;
  toggleMaximize: () => void;
  isMaximized: () => Promise<boolean>;
  setTitle: (title: string) => void;
  setDecorations: (decorations: boolean) => void;
  setAlwaysOnTop: (alwaysOnTop: boolean) => void;
  innerSize: () => Promise<{ width: number; height: number }>;
  outerSize: () => Promise<{ width: number; height: number }>;
  innerPosition: () => Promise<{ x: number; y: number }>;
  outerPosition: () => Promise<{ x: number; y: number }>;
  setSize: (size: { width: number; height: number }) => void;
  setPosition: (position: { x: number; y: number }) => void;
  center: () => void;
  requestUserAttention: (level: 'info' | 'critical') => void;
  onResized: (cb: (event: { payload: { width: number; height: number } }) => void) => Promise<() => void>;
  onMoved: (cb: (event: { payload: { x: number; y: number } }) => void) => Promise<() => void>;
  onScaleChanged: (cb: (event: { payload: { scaleFactor: number; size: { width: number; height: number } } }) => void) => Promise<() => void>;
  startDragging: () => void;
  listen: (event: string, cb: (event: unknown) => void) => Promise<() => void>;
}

export function getCurrentWindow(): WindowManager {
  const noop = () => {};
  const noopAsync = async () => {};
  const noopAsyncFalse = async () => false;
  const noopAsyncSize = async () => ({ width: window.innerWidth, height: window.innerHeight });
  const noopAsyncPos = async () => ({ x: window.screenX, y: window.screenY });
  const noopAsyncUnlisten = async () => noop;

  return {
    onCloseRequested: (cb) => {
      window.addEventListener('beforeunload', (e) => {
        e.preventDefault();
        cb({ preventDefault: () => {} });
      });
    },
    close: () => window.close(),
    setFocus: () => window.focus(),
    minimize: noop,
    toggleMaximize: noop,
    isMaximized: noopAsyncFalse,
    setTitle: (title: string) => { document.title = title; },
    setDecorations: noop,
    setAlwaysOnTop: noop,
    innerSize: noopAsyncSize,
    outerSize: noopAsyncSize,
    innerPosition: noopAsyncPos,
    outerPosition: noopAsyncPos,
    setSize: noop,
    setPosition: noop,
    center: noop,
    requestUserAttention: noop,
    onResized: noopAsyncUnlisten,
    onMoved: noopAsyncUnlisten,
    onScaleChanged: noopAsyncUnlisten,
    startDragging: noop,
    listen: async () => noop,
  };
}
