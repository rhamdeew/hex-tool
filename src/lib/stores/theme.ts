import { writable } from 'svelte/store';

export type Theme = 'light' | 'dark' | 'auto';

function createThemeStore() {
  const { subscribe, set, update } = writable<Theme>('auto');

  return {
    subscribe,
    set,
    toggle: () => update((theme) => (theme === 'light' ? 'dark' : 'light')),
    setAuto: () => set('auto'),
  };
}

export const theme = createThemeStore();
