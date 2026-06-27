// Web shim for @tauri-apps/plugin-dialog
// Replaces Tauri dialog plugin with Web APIs

export type DialogFilter = {
  name: string;
  extensions: string[];
};

export async function confirm(message: string, options?: { title?: string; type?: 'info' | 'warning' | 'error' }): Promise<boolean> {
  return window.confirm(message);
}

export async function ask(message: string, options?: { title?: string; type?: 'info' | 'warning' | 'error' }): Promise<boolean> {
  return window.confirm(message);
}

export async function message(message: string, options?: { title?: string; type?: 'info' | 'warning' | 'error' }): Promise<void> {
  window.alert(message);
}

export async function open(options?: {
  title?: string;
  defaultPath?: string;
  filters?: DialogFilter[];
  multiple?: boolean;
  directory?: boolean;
}): Promise<string | string[] | null> {
  // File open dialog using input element
  return new Promise((resolve) => {
    const input = document.createElement('input');
    input.type = 'file';
    if (options?.multiple) input.multiple = true;
    if (options?.directory) {
      // @ts-expect-error webkitdirectory is non-standard
      input.webkitdirectory = true;
    }
    if (options?.filters) {
      const extensions = options.filters.flatMap(f => f.extensions.map(e => `.${e}`));
      if (extensions.length) input.accept = extensions.join(',');
    }
    input.onchange = () => {
      if (!input.files?.length) {
        resolve(null);
        return;
      }
      if (options?.multiple) {
        resolve(Array.from(input.files).map(f => f.name));
      } else {
        resolve(input.files[0].name);
      }
    };
    input.oncancel = () => resolve(null);
    input.click();
  });
}

export async function save(options?: {
  title?: string;
  defaultPath?: string;
  filters?: DialogFilter[];
}): Promise<string | null> {
  // Web can't really do save dialogs, return the default path
  return options?.defaultPath || null;
}
