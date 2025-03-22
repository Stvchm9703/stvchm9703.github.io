import {
  defineConfig,
  presetTypography,
  presetWebFonts,
  // presetMini,
  presetWind,
} from "unocss";

import presetWind4 from "@unocss/preset-wind4";
// import { presetDaisy } from 'unocss-preset-daisy';
import transformerVariantGroup from "@unocss/transformer-variant-group";
import { presetFluid } from "unocss-preset-fluid";
import transformerDirectives from "@unocss/transformer-directives";
import presetIcons from "@unocss/preset-icons/browser";

import { presetScrollbar } from "unocss-preset-scrollbar";

import { presetAnimations } from "unocss-preset-animations";

export default defineConfig({
  content: {
    pipeline: {
      include: [
        // the default
        /\.(vue|svelte|[jt]sx|mdx?|astro|elm|php|phtml|html)($|\?)/,
        // include js/ts files
        "src/**/*.{js,ts,svelte}",
        "./node_modules/@selemondev/svelte-marquee/dist/*.svelte",
      ],
      // exclude files
      // exclude: []
    },
  },

  shortcuts: [
    // { logo: 'i-logos:svelte-icon w-7em h-7em transform transition-300' },
    // [/^bg-gradient-(.*)$/, match=>`bg-gradient-to-r from-${match[1]}-400 to-${match[1]}-600`],
    { title: "text-light-50 lg:text-5xl text-2xl leading-loose" },
    { subtitle: "text-light-50 lg:text-3xl text-xl leading-loose" },
  ],
  transformers: [transformerVariantGroup(), transformerDirectives()],
  presets: [
    presetAnimations(),
    presetWind(),
    presetWind4(),
    presetFluid(),
    presetIcons({
      collections: {
        carbon: () =>
          import("@iconify-json/carbon/icons.json").then((i) => i.default),
      },
    }),
    // presetAtoUI(),
    presetTypography(),
    presetWebFonts({
      provider: "bunny",
      fonts: {
        // these will extend the default theme
        // sans: 'Roboto',
        mono: ["JetBrains Mono", "JetBrains Mono:400,700"],
        sans: ["Zen Kaku Gothic Antique", "Inter", "sans-serif"],
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
    `,
    },
  ],
  // safelist: ['bg-orange-300', 'prose'],
  theme: {
    breakpoints: {
      sm: "20rem",
      // Because uno does not support comparison sorting of different unit sizes, please convert to the same unit.
      md: "42rem",
      // md: `${40 * 16}px`,
      lg: "66rem",
      xl: "82rem",
      "2xl": "98rem",
    },
    extend: {
      colors: {
        border: "hsl(var(--border) / <alpha-value>)",
        input: "hsl(var(--input) / <alpha-value>)",
        ring: "hsl(var(--ring) / <alpha-value>)",
        background: "hsl(var(--background) / <alpha-value>)",
        foreground: "hsl(var(--foreground) / <alpha-value>)",
        primary: {
          DEFAULT: "hsl(var(--primary) / <alpha-value>)",
          foreground: "hsl(var(--primary-foreground) / <alpha-value>)",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary) / <alpha-value>)",
          foreground: "hsl(var(--secondary-foreground) / <alpha-value>)",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive) / <alpha-value>)",
          foreground: "hsl(var(--destructive-foreground) / <alpha-value>)",
        },
        muted: {
          DEFAULT: "hsl(var(--muted) / <alpha-value>)",
          foreground: "hsl(var(--muted-foreground) / <alpha-value>)",
        },
        accent: {
          DEFAULT: "hsl(var(--accent) / <alpha-value>)",
          foreground: "hsl(var(--accent-foreground) / <alpha-value>)",
        },
        popover: {
          DEFAULT: "hsl(var(--popover) / <alpha-value>)",
          foreground: "hsl(var(--popover-foreground) / <alpha-value>)",
        },
        card: {
          DEFAULT: "hsl(var(--card) / <alpha-value>)",
          foreground: "hsl(var(--card-foreground) / <alpha-value>)",
        },
        sidebar: {
          DEFAULT: "hsl(var(--sidebar-background))",
          foreground: "hsl(var(--sidebar-foreground))",
          primary: "hsl(var(--sidebar-primary))",
          "primary-foreground": "hsl(var(--sidebar-primary-foreground))",
          accent: "hsl(var(--sidebar-accent))",
          "accent-foreground": "hsl(var(--sidebar-accent-foreground))",
          border: "hsl(var(--sidebar-border))",
          ring: "hsl(var(--sidebar-ring))",
        },
      },
      borderRadius: {
        xl: "calc(var(--radius) + 4px)",
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      keyframes: {
        "accordion-down": {
          from: { height: "0" },
          to: { height: "var(--bits-accordion-content-height)" },
        },
        "accordion-up": {
          from: { height: "var(--bits-accordion-content-height)" },
          to: { height: "0" },
        },
        "caret-blink": {
          "0%,70%,100%": { opacity: "1" },
          "20%,50%": { opacity: "0" },
        },
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "caret-blink": "caret-blink 1.25s ease-out infinite",
      },
    },
  },
});
