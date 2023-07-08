import {
  defineConfig,
  presetTypography,
  presetWebFonts,
  presetMini, presetWind
} from 'unocss';
import { presetDaisy } from 'unocss-preset-daisy';
import transformerVariantGroup from '@unocss/transformer-variant-group'

export default defineConfig({
  shortcuts: [
    // { logo: 'i-logos:svelte-icon w-7em h-7em transform transition-300' },
  ],
  transformers: [transformerVariantGroup()],
  presets: [
    presetWind(), presetMini(), presetDaisy(),

    presetTypography(),
    presetWebFonts({
      provider: 'bunny',
      fonts: {
        // these will extend the default theme
        // sans: 'Roboto',
        mono: ['JetBrains Mono', 'JetBrains Mono:400,700'],
        sans: ['Zen Kaku Gothic Antique', 'Inter', 'sans-serif'],
      },
    }),
  ],
  preflights: [
    {
      getCSS: ({ theme }) => `
      html {
        scroll-behavior: smooth;
      }
      *{
        font-family: ${theme.fontFamily.sans};
      }
    `
    }
  ]
  // safelist: ['bg-orange-300', 'prose'],
})