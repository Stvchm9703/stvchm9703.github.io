import {
  defineConfig,
  presetTypography,
  presetWebFonts,
  presetMini, presetWind, transformerDirectives
} from 'unocss';
import { presetDaisy } from 'unocss-preset-daisy';

export default defineConfig({
  shortcuts: [
    { logo: 'i-logos:svelte-icon w-7em h-7em transform transition-300' },
  ],
  transformers: [transformerDirectives()],
  presets: [
    presetWind(), presetMini(), presetDaisy(),

    presetTypography(),
    presetWebFonts({
      fonts: {
        // these will extend the default theme
        // sans: 'Roboto',
        mono: ['Fira Code', 'Fira Mono:400,700'],
      },
    }),
  ],
  // safelist: ['bg-orange-300', 'prose'],
})