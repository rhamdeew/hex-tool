import { backend } from '$lib/services/backend';

// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

// Runtime asset scope is not persisted, so re-grant it before any page renders images
export const load = async () => {
  await backend.restoreProject();
};
