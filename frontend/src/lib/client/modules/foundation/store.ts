import { writable } from 'svelte/store';

// --- Theme Store ---

type Theme = 'light' | 'dark';

function createThemeStore() {
  const initial = getStoredTheme();
  const { subscribe, set, update } = writable<Theme>(initial);

  function getStoredTheme(): Theme {
    try {
      if (typeof window !== 'undefined') {
        const stored = localStorage.getItem('batua-theme');
        if (stored === 'light' || stored === 'dark') {
          return stored;
        }
      }
    } catch {
      // SSR or localStorage unavailable
    }
    return 'light';
  }

  function persist(theme: Theme) {
    try {
      if (typeof window !== 'undefined') {
        localStorage.setItem('batua-theme', theme);
        document.documentElement.setAttribute('data-theme', theme);
      }
    } catch {
      // localStorage unavailable
    }
  }

  return {
    subscribe,
    toggle() {
      update((current) => {
        const next: Theme = current === 'light' ? 'dark' : 'light';
        persist(next);
        return next;
      });
    },
    setTheme(theme: Theme) {
      persist(theme);
      set(theme);
    }
  };
}

export const themeStore = createThemeStore();

// --- Sidebar Store ---

type SidebarState = {
  collapsed: boolean;
};

function createSidebarStore() {
  const { subscribe, update } = writable<SidebarState>({ collapsed: false });

  return {
    subscribe,
    toggle() {
      update((state) => ({ ...state, collapsed: !state.collapsed }));
    },
    collapse() {
      update((state) => ({ ...state, collapsed: true }));
    },
    expand() {
      update((state) => ({ ...state, collapsed: false }));
    }
  };
}

export const sidebarStore = createSidebarStore();

// --- Toast Store ---

type ToastLevel = 'success' | 'error' | 'warning' | 'info';

type Toast = {
  id: string;
  message: string;
  level: ToastLevel;
  durationMs: number;
};

type ToastInput = {
  message: string;
  level: ToastLevel;
  durationMs?: number;
};

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);

  let counter = 0;

  function generateId(): string {
    counter += 1;
    return `toast-${Date.now()}-${counter}`;
  }

  return {
    subscribe,
    push(input: ToastInput) {
      const toast: Toast = {
        id: generateId(),
        message: input.message,
        level: input.level,
        durationMs: input.durationMs ?? 4000
      };
      update((toasts) => [...toasts, toast]);
      return toast.id;
    },
    dismiss(id: string) {
      update((toasts) => toasts.filter((t) => t.id !== id));
    },
    clear() {
      update(() => []);
    }
  };
}

export const toastStore = createToastStore();
