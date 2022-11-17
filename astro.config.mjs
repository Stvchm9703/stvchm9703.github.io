import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';

import { presetMini, transformerDirectives } from 'unocss';
import unocss from 'unocss/astro'
import transformerCompileClass from '@unocss/transformer-compile-class'
// https://astro.build/config
import svelte from "@astrojs/svelte";
import solid from "@astrojs/solid-js";

// https://astro.build/config
export default defineConfig({
  outDir: 'dist',
  output: 'static',
  site: 'https://stv.chm9703.github.io',
  integrations: [
    sitemap(),
    mdx(),
    svelte(), solid(),
    unocss({
      inspector: false,
      mode: 'dist-chunk',
      transformers: [transformerCompileClass(), transformerDirectives()],
      presets: [
        presetMini()
      ],
    }),
  ],

  // vite: {
  //   build: {
  //     rollupOptions: {
  //       external:['uno.css']
  //     }
  //   }
  // }
});