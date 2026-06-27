// Web shim for @tauri-apps/api/event
// Replaces Tauri listen() with SSE (Server-Sent Events)

export type UnlistenFn = () => void;

// SSE-based event listener
export function listen<T>(event: string, handler: (payload: { payload: T }) => void): Promise<UnlistenFn> {
  return new Promise((resolve) => {
    const es = new EventSource(`/api/events`);
    es.addEventListener(event, (e: MessageEvent) => {
      try {
        const data = JSON.parse(e.data);
        handler({ payload: data });
      } catch {
        handler({ payload: e.data as T });
      }
    });
    es.onerror = () => {
      // Auto-reconnect is handled by EventSource
    };
    resolve(() => es.close());
  });
}

// emit is not needed on the client side, but export a no-op for safety
export function emit(_event: string, _payload?: unknown): void {
  // no-op in web mode
}
