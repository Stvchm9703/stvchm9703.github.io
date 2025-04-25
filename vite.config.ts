// import { resolve } from "path";
import { sveltekit } from "@sveltejs/kit/vite";
// import unocss from '@unocss/svelte-scoped/vite';
import unocss from "@unocss/vite";
import ViteCompression from "vite-plugin-compression";
// import { extractorSvelte } from "@unocss/core";
import { chunkSplitPlugin } from "vite-plugin-chunk-split";
import EntryShakingPlugin from "vite-plugin-entry-shaking";

import { visualizer } from "rollup-plugin-visualizer";
import { imagetools } from "vite-imagetools";
import dynamicImport from "vite-plugin-dynamic-import";
// import type { UserConfig } from "vite";
import { defineConfig, loadEnv } from "vite";
import deadFile from "vite-plugin-deadfile";
// import transformerDirectives from '@unocss/transformer-directives'

export default defineConfig(({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };
  // console.log(process.env)
  return {
    plugins: [
      // deadFile({
      //   include: ["src/**"],
      // }),
      //
      // EntryShakingPlugin({
      //   targets: [
      //     // Or using glob patterns.
      //     {
      //       glob: "src/routes/**",
      //       // globOptions: { ignore: ["excluded.ts"] },
      //     },
      //   ],
      // }),
      imagetools({
        // force: true,
        removeMetadata: true,
      }),
      dynamicImport({}),
      unocss({
        // mode: 'dist-chunk',
      }),
      sveltekit(),
      ViteCompression({ algorithm: "gzip" }),
      chunkSplitPlugin(),
      // visualizer({ open: true, filename: "bundle-visualization.html" }),
    ],

    optimizeDeps: {
      // include: ["pixi.js"],
    },

    build: {
      cssMinify: "lightningcss",

      terserOptions: {
        parse: {
          html5_comments: false,
        },
      },
      rollupOptions: {
        treeshake: true, // Ensure tree shaking is enabled

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
          compact: true,
        },
      },
    },
  };
});
