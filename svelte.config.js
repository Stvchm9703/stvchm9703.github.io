// import adapter from '@sveltejs/adapter-auto';
import adapter from "@sveltejs/adapter-static";
// import UnoCSS from "@unocss/svelte-scoped/preprocess";
// import preprocess from "svelte-preprocess";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { svelte_client_components as clientComponents } from "svelte-client-components";
/** @type {import('@sveltejs/kit').Config} */
const config = {
  compilerOptions: {
    css: "external",
  },
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [clientComponents(), vitePreprocess()],

  kit: {
    // inspector: process.argv.includes('dev'),
    adapter: adapter({
      fallback: "200.html",
      // precompress: true,
      pages: "build",
      assets: "build",
      precompress: !process.argv.includes("dev"),
      strict: true,
    }),
    prerender: {
      // default: true,
      handleHttpError: "ignore",
      handleMissingId: "ignore",
      handleEntryGeneratorMismatch: "ignore",
      crawl: true,
    },
    // files: {
    //   hooks: "src/hooks",
    // },
    alias: {
      $lib: "src/lib",
      "$lib/*": "src/lib/*",
      $assets: "src/assets",
      "$assets/*": "src/assets/*",
      $type: "src/types",
      "$type/*": "src/types/*",
      $types: "src/types",
      "$types/*": "src/types/*",

      $generateor: "tools/convert_blog_post_ts",
      "$generateor/*": "tools/convert_blog_post_ts/*",
    },
    paths: {
      base: process.argv.includes("dev") ? "" : process.env.BASE_PATH,
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
