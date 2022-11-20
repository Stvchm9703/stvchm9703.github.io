import { sveltekit } from '@sveltejs/kit/vite';
import unocss from 'unocss/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
  plugins: [sveltekit(), unocss({
    mode:'svelte-scoped',
  })]
};

export default config;
