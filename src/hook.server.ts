// src/hooks.server.ts
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
  return resolve(event, {
    transformPageChunk: (html) => {
      // Replaces UnoCSS global injection placeholder
      return html.html.replace('%unocss-svelte-scoped.global%', '');
    }
  });
};
