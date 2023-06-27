import { sveltekit } from "@sveltejs/kit/vite";
// import unocss from '@unocss/svelte-scoped/vite';
import unocss from '@unocss/vite';

import { extractorSvelte } from "@unocss/core";

import {
  presetTypography,
  presetWebFonts,
  presetMini, presetWind, transformerDirectives
} from 'unocss';
import { presetDaisy } from 'unocss-preset-daisy';


import { imagetools } from "vite-imagetools";
// import type { UserConfig } from "vite";
import { defineConfig, loadEnv } from "vite";
// import transformerDirectives from '@unocss/transformer-directives'

export default defineConfig(({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd()) };
  // console.log(process.env)
  return {
    plugins: [
      imagetools({
        force: true,
      }),
      unocss({
        mode: 'dist-chunk',
      }),
      sveltekit(),
    ],
  };
});
