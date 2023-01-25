// import adapter from '@sveltejs/adapter-auto';
import adapter from "@sveltejs/adapter-static";

import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess(),
  ],

  kit: {
    adapter: adapter({
      fallback: "200.html",
      precompress: true,
    }),
    // files: {
    //   hooks: "src/hooks",
    // },
    alias: {
      '$assets' : 'src/assets',
      '$assets/*' : 'src/assets/*',

    }
  },
  compilerOptions: {
    css: "external",
  },
};

export default config;
