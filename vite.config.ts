// import { resolve } from "path";
import { sveltekit } from "@sveltejs/kit/vite";
// import unocss from '@unocss/svelte-scoped/vite';
import unocss from "@unocss/vite";
import ViteCompression from "vite-plugin-compression";
import extractorSvelte from "@unocss/extractor-svelte";
// import { chunkSplitPlugin } from "vite-plugin-chunk-split";
// import entryShakingPlugin from "vite-plugin-entry-shaking";
import stripComments from "vite-plugin-strip-comments";
import { visualizer } from "rollup-plugin-visualizer";
import { imagetools } from "vite-imagetools";
import devtoolsJson from "vite-plugin-devtools-json";
import dynamicImport from "vite-plugin-dynamic-import";
// import type { UserConfig } from "vite";
import { defineConfig, loadEnv } from "vite";
// import deadFile from "vite-plugin-deadfile";
// import transformerDirectives from '@unocss/transformer-directives'

// console.log("schould be here");

export default defineConfig(async ({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };
  // console.log(process.env);
  return {
    plugins: [
      imagetools({
        // force: true,
        removeMetadata: true,
      }),
      dynamicImport({
        filter(id) {
          // `node_modules` is exclude by default, so we need to include it explicitly
          // https://github.com/vite-plugin/vite-plugin-dynamic-import/blob/v1.3.0/src/index.ts#L133-L135
          if (id.includes("/node_modules/svelte-highlight/languages")) {
            return true;
          }
        },
      }),
      devtoolsJson(),
      sveltekit(),

      unocss({
        // configOrPath: 'uno.config.ts',
        // mode: 'dist-chunk',
        // injectReset: "@unocss/reset/tailwind.css",
        extractors: [extractorSvelte()],
        configFile: "uno.config.ts",
      }),
      process.env.NODE_ENV === "production"
        ? stripComments({ type: "none", enforce: "post" })
        : null,
      ViteCompression({ algorithm: "gzip" }),
      // visualizer({ open: true, filename: "bundle-visualization.html" }),
    ],

    optimizeDeps: {
      // include: ["pixi.js"],
    },

    build: {
      cssMinify: "lightningcss",
      minify: "terser",
      terserOptions: {
        parse: {
          html5_comments: false,
        },
        compress: true,
      },
      rolldownOptions: {
        output: {
          manualChunks: (id) => {
            if (id.includes("/svelte/") || id.includes("/@svelte/")) {
              return "svelte";
            } else if (id.includes("node_modules")) {
              return "vendor";
            } else if (id.includes("routes")) {
              return "pages";
            } else if (id.includes("lib")) {
              return "lib";
            } else {
              return "main";
            }
          },
        },
      },
    },
  };
});
