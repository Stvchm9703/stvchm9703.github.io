import type { ClassValue } from "clsx";
import { clsx } from "clsx";
import { cubicOut } from "svelte/easing";
import { derived, get, writable } from "svelte/store";
import type { TransitionConfig } from "svelte/transition";
import { twMerge } from "tailwind-merge";
import { error } from "@sveltejs/kit";
import { persisted } from "svelte-persisted-store";
import type { DocResolver } from "$lib/types/docs.js";
import { isEmpty } from "lodash-es";

// export const cn = (inputs) => {
//   return twMerge(clsx(inputs));
// };
//
//

export const pathResolver = (path: string) => {
  return path
    .replace(/\s+/g, "_")
    .replace(/\W/g, "")
    .toLowerCase()
    .split("_")
    .filter((e) => !isEmpty(e))
    .join("-");
};
export const displayDate = (publish_date: number) =>
  publish_date !== 0 ? new Date(publish_date * 1000).toLocaleDateString() : "";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const isBrowser = typeof document !== "undefined";

export function slugFromPath(path: string) {
  return path.replace("/src/content/", "").replace(".md", "");
}

export function hexToHsl(hex: string): [number, number, number] {
  if (hex) {
    const sanitizedHex = hex.replace("#", "");

    const red = Number.parseInt(sanitizedHex.substring(0, 2), 16);
    const green = Number.parseInt(sanitizedHex.substring(2, 4), 16);
    const blue = Number.parseInt(sanitizedHex.substring(4, 6), 16);

    const normalizedRed = red / 255;
    const normalizedGreen = green / 255;
    const normalizedBlue = blue / 255;

    const max = Math.max(normalizedRed, normalizedGreen, normalizedBlue);
    const min = Math.min(normalizedRed, normalizedGreen, normalizedBlue);

    let hue, saturation, lightness;

    if (max === min) {
      hue = 0;
    } else if (max === normalizedRed) {
      hue = ((normalizedGreen - normalizedBlue) / (max - min)) % 6;
    } else if (max === normalizedGreen) {
      hue = (normalizedBlue - normalizedRed) / (max - min) + 2;
    } else {
      hue = (normalizedRed - normalizedGreen) / (max - min) + 4;
    }

    hue = Math.round(hue * 60);

    if (hue < 0) {
      hue += 360;
    }

    lightness = (max + min) / 2;

    if (max === min) {
      saturation = 0;
    } else if (lightness <= 0.5) {
      saturation = (max - min) / (max + min);
    } else {
      saturation = (max - min) / (2 - max - min);
    }

    saturation = Math.round(saturation * 100);
    lightness = Math.round(lightness * 100);

    return [hue, saturation, lightness];
  }
  return [0, 0, 0];
}

export function hexToRgb(hex: string): [number, number, number] {
  if (hex) {
    const sanitizedHex = hex.replace("#", "");

    const red = Number.parseInt(sanitizedHex.substring(0, 2), 16);
    const green = Number.parseInt(sanitizedHex.substring(2, 4), 16);
    const blue = Number.parseInt(sanitizedHex.substring(4, 6), 16);

    return [red, green, blue];
  }
  return [0, 0, 0];
}

export function createCopyCodeButton() {
  const codeString = writable("");
  const copied = writable(false);
  let copyTimeout = 0;

  function copyCode() {
    if (!isBrowser) return;
    navigator.clipboard.writeText(get(codeString));
    copied.set(true);
    clearTimeout(copyTimeout);
    copyTimeout = window.setTimeout(() => {
      copied.set(false);
    }, 2500);
  }

  function setCodeString(node: HTMLElement) {
    codeString.set(node.innerText.trim() ?? "");
  }

  return {
    copied,
    copyCode,
    setCodeString,
    codeString,
  };
}

export function updateTheme(activeTheme: string, path: string) {
  if (!isBrowser) return;
  document.body.classList.forEach((className) => {
    if (className.match(/^theme.*/)) {
      document.body.classList.remove(className);
    }
  });

  const theme = path === "/themes" ? activeTheme : null;
  if (theme) {
    return document.body.classList.add(`theme-${theme}`);
  }
}

type FlyAndScaleParams = {
  y?: number;
  x?: number;
  start?: number;
  duration?: number;
};

export function styleToString(
  style: Record<string, number | string | undefined>,
): string {
  return Object.keys(style).reduce((str, key) => {
    if (style[key] === undefined) return str;
    return `${str}${key}:${style[key]};`;
  }, "");
}

export function flyAndScale(
  node: Element,
  params: FlyAndScaleParams = { y: -8, x: 0, start: 0.95, duration: 150 },
): TransitionConfig {
  const style = getComputedStyle(node);
  const transform = style.transform === "none" ? "" : style.transform;

  const scaleConversion = (
    valueA: number,
    scaleA: [number, number],
    scaleB: [number, number],
  ) => {
    const [minA, maxA] = scaleA;
    const [minB, maxB] = scaleB;

    const percentage = (valueA - minA) / (maxA - minA);
    const valueB = percentage * (maxB - minB) + minB;

    return valueB;
  };

  return {
    duration: params.duration ?? 200,
    delay: 0,
    css: (t) => {
      const y = scaleConversion(t, [0, 1], [params.y ?? 5, 0]);
      const x = scaleConversion(t, [0, 1], [params.x ?? 0, 0]);
      const scale = scaleConversion(t, [0, 1], [params.start ?? 0.95, 1]);

      return styleToString({
        transform: `${transform} translate3d(${x}px, ${y}px, 0) scale(${scale})`,
        opacity: t,
      });
    },
    easing: cubicOut,
  };
}

type Modules = Record<string, () => Promise<unknown>>;

function findMatch(slug: string, modules: Modules) {
  let match: { path?: string; resolver?: DocResolver } = {};

  for (const [path, resolver] of Object.entries(modules)) {
    if (slugFromPath(path) === slug) {
      match = { path, resolver: resolver as unknown as DocResolver };
      break;
    }
  }
  if (!match.path) {
    match = getIndexDocIfExists(slug, modules);
  }

  return match;
}

function getIndexDocIfExists(slug: string, modules: Modules) {
  let match: { path?: string; resolver?: DocResolver } = {};

  for (const [path, resolver] of Object.entries(modules)) {
    if (path.includes(`/${slug}/index.md`)) {
      match = { path, resolver: resolver as unknown as DocResolver };
      break;
    }
  }

  return match;
}

export async function getDoc(slug: string) {
  const modules = import.meta.glob(`/src/content/**/*.md`);
  const match = findMatch(slug, modules);
  const doc = await match?.resolver?.();

  if (!doc || !doc.metadata) {
    error(404);
  }

  return {
    component: doc.default,
    metadata: doc.metadata,
    title: doc.metadata.title,
  };
}

export function slugFromPathname(pathname: string) {
  return pathname.split("/").pop() ?? "";
}

const liftMode = persisted<string[]>("lift-mode", []);

export function getLiftMode(name: string) {
  function toggleLiftMode(name: string) {
    liftMode.update((prev) => {
      return prev.includes(name)
        ? prev.filter((n) => n !== name)
        : [...prev, name];
    });
  }

  const isLiftMode = derived(liftMode, ($configStore) =>
    $configStore.includes(name),
  );

  return {
    isLiftMode,
    toggleLiftMode,
  };
}

export const packageManagers = ["pnpm", "bun", "yarn", "npm"] as const;
export type PackageManager = (typeof packageManagers)[number];

export const selectedPackageManager = persisted<PackageManager>(
  "package-manager",
  "npm",
);

const packageManagerToScriptCmd: Record<PackageManager, string> = {
  npm: "npx",
  yarn: "yarn dlx",
  pnpm: "pnpm dlx",
  bun: "bunx",
};

export function getPackageManagerScriptCmd(pm: PackageManager): string {
  return packageManagerToScriptCmd[pm];
}

const packageManagerToInstallCmd: Record<PackageManager, string> = {
  npm: "install",
  yarn: "add",
  pnpm: "add",
  bun: "add",
};

export function getPackageManagerInstallCmd(pm: PackageManager): string {
  return packageManagerToInstallCmd[pm];
}

// eslint-disable-next-line ts/no-explicit-any
export function isPackageManager(value: any): value is PackageManager {
  return packageManagers.includes(value);
}

export function once(fn) {
  return function (event) {
    if (fn) fn.call(this, event);
    fn = null;
  };
}

export function preventDefault(fn) {
  return function (event) {
    event.preventDefault();
    fn.call(this, event);
  };
}
