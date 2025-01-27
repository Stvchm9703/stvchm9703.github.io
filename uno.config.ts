import {
  defineConfig,
  presetTypography,
  presetWebFonts,
  presetMini, presetWind
} from 'unocss';
// import { presetDaisy } from 'unocss-preset-daisy';
import transformerVariantGroup from '@unocss/transformer-variant-group'
import { presetFluid } from 'unocss-preset-fluid';
import transformerDirectives from '@unocss/transformer-directives';
import presetIcons from '@unocss/preset-icons/browser';

import { presetScrollbar } from 'unocss-preset-scrollbar'


export default defineConfig({
  shortcuts: [
    // { logo: 'i-logos:svelte-icon w-7em h-7em transform transition-300' },
    // [/^bg-gradient-(.*)$/, match=>`bg-gradient-to-r from-${match[1]}-400 to-${match[1]}-600`],
  ],
  transformers: [transformerVariantGroup(), transformerDirectives()],
  presets: [
    presetWind(),
    presetFluid(),
    presetIcons({
      collections: {
        carbon: () => import('@iconify-json/carbon/icons.json').then(i => i.default),
      }
    }),
    // presetAtoUI(),
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

    presetScrollbar({
      // config
    }),
  ],
  preflights: [
    {
      getCSS: ({ theme }) => `
      html { scroll-behavior: smooth; }
      *{ font-family: ${theme.fontFamily.sans}; }
    `
    }
  ],
  // safelist: ['bg-orange-300', 'prose'],
  theme: {
    breakpoints: {
      sm: '20rem',
      // Because uno does not support comparison sorting of different unit sizes, please convert to the same unit.
      md: '42rem',
      // md: `${40 * 16}px`,
      lg: '66rem',
      xl: '82rem',
      '2xl': '98rem',
    },
  }
})