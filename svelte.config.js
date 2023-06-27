// import adapter from '@sveltejs/adapter-auto';
import adapter from "@sveltejs/adapter-static";
import UnoCSS from "@unocss/svelte-scoped/preprocess";
import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess(),
    // UnoCSS({
    //   // classPrefix: "me-",
    // }),
  ],

  kit: {
    adapter: adapter({
      fallback: "200.html",
      precompress: true,
    }),
    prerender: {
      // default: true,
      handleHttpError: "warn",
      handleMissingId: "warn",
    },
    // files: {
    //   hooks: "src/hooks",
    // },
    alias: {
      "$assets": "src/assets",
      "$assets/*": "src/assets/*",
    },
  },
  compilerOptions: {
    css: "external",
  },
  vitePlugin: {
    inspector: {
      showToggleButton: "always",
    },
  },
};

export default config;
