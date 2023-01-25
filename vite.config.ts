import { sveltekit } from "@sveltejs/kit/vite";
import unocss from "unocss/vite";
import { extractorSvelte } from "@unocss/core";
import { imagetools } from "vite-imagetools";
// import type { UserConfig } from "vite";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };
  // console.log(process.env)
  return {
    plugins: [
      imagetools({
        force: true,
      }),
      unocss({
        mode: process.env.NODE_ENV === "development"
          ? "svelte-scoped"
          : "dist-chunk",
        extractors: [extractorSvelte],
      }),
      sveltekit(),
    ],
  };
});
