// import adapter from '@sveltejs/adapter-auto';
import adapter from "@sveltejs/adapter-static";

// import UnoCSS from "@unocss/svelte-scoped/preprocess";
// import preprocess from "svelte-preprocess";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

import { svelte_client_components as clientComponents } from "svelte-client-components";

import { fileURLToPath } from "node:url";
// import { resolve } from "path";
import { sveltekit } from "@sveltejs/kit/vite";
// import unocss from '@unocss/svelte-scoped/vite';
import unocss from "unocss/vite";
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

      unocss({
        // configOrPath: 'uno.config.ts',
        // mode: 'dist-chunk',
        // injectReset: "@unocss/reset/tailwind.css",
        // extractors: [extractorSvelte()],
        configFile: "uno.config.ts",
      }),


      sveltekit({
        // Consult https://github.com/sveltejs/svelte-preprocess
        // for more information about preprocessors
        preprocess: [clientComponents(), vitePreprocess()],
        compilerOptions: { css: "external" },
        // vitePlugin: { inspector: { showToggleButton: "always" } },
        appDir: "app",
        // inspector: process.argv.includes('dev'),
        adapter: adapter({
          fallback: "200.html",
          // precompress: true,
          pages: "build",
          assets: "build",
          precompress: !process.argv.includes("dev"),
          strict: true
        }),

        prerender: {
          // "*" = prerender every route that has no required params; routes with
          // params supply their own `entries` generator. This is the "prerender
          // everything" switch.
          entries: ["*"],
          crawl: true,
          // Report every page that fails to prerender, but stay quiet about CMS
          // asset files (served by miniserve, not part of the built site).
          // Throw instead of warn here to make a failed page break the build.
          handleHttpError: ({ path, referrer, message }) => {
            if (path.startsWith("/blog/assets/")) return;
            console.warn(
              `[prerender] ${message}${referrer ? ` (from ${referrer})` : ""}`
            );
          },
          handleMissingId: "warn",
          handleEntryGeneratorMismatch: "warn"
        },
        // files: {
        //   hooks: "src/hooks",
        // },
        paths: {
          base: process.argv.includes("dev") ? "" : process.env.BASE_PATH
        }
      }),
      process.env.NODE_ENV === "production"
        ? stripComments({ type: "none", enforce: "post" })
        : null,
      ViteCompression({ algorithm: "gzip" })
    ],
    // visualizer({ open: true, filename: "bundle-visualization.html" }),
    optimizeDeps: {},
    // include: ["pixi.js"],
    resolve: {
      alias: {
        $assets: fileURLToPath(new URL("./src/assets", import.meta.url)),
        $type: fileURLToPath(new URL("./src/types", import.meta.url)),
        $types: fileURLToPath(new URL("./src/types", import.meta.url)),
        $components: fileURLToPath(new URL("./src/lib/components", import.meta.url)),
        $generateor: fileURLToPath(new URL("./src/types", import.meta.url)),
      },
    },
    build: {
      // REQUIRED for UnoCSS to emit any CSS. @unocss/vite caches Vite's internal
      // `vite:css-post` plugin in a Map keyed by the resolved `build.outDir`,
      // then looks it up at renderChunk time using rollup's `options.dir`.
      // SvelteKit emits to .svelte-kit/output/{client,server}, but leaves
      // build.outDir at Vite's default "dist" — so the lookup missed and UnoCSS
      // silently skipped injection, shipping its raw `#--unocss--` placeholder
      // and ZERO utility classes. Pointing outDir at the client dir makes the
      // keys line up. (The server pass still warns; harmless, its CSS is not
      // shipped.) Symptom if this is removed: site renders unstyled and
      // build/app/immutable/assets/__uno*.css is ~48 bytes.
      outDir: ".svelte-kit/output/client",
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
