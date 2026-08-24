import { isArrayLike, isEmpty } from "lodash-es";
// import { pathResolver as pr } from "$lib/utils";
import type { Mark, ComponentStyle } from "$generateor/content_block";

/** Internal mark type that tolerates legacy numeric values from older JSON. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type MarkTypeOrLegacy = any;

/** Escape the five significant HTML characters. */
const escapeHtml = (s: string): string =>
  s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");

/** Escape a value destined for a double-quoted attribute. */
const escapeAttr = (s: string): string =>
  s.replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;");

type ResolvedMark = { start: number; end: number; open: string; close: string };

/** Map a single mark to its opening/closing tag strings. Returns null for unsupported types. */
const resolveMarkTags = (mark: Mark): ResolvedMark | null => {
  const start = mark.range?.from ?? 0;
  const end = mark.range?.to ?? 0;
  if (end <= start) return null;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const markType: MarkTypeOrLegacy = (mark as any).type;
  let tag = "";
  let attr = "";
  const classToken: string[] = [];

  switch (markType) {
    case 0:
    case "Strikethrough":
      classToken.push("line-through");
      tag = "del";
      break;
    case 1:
    case "Keyboard":
      classToken.push("keyboard", "bg-slate-300/30", "text-sm", "p-1");
      tag = "kbd";
      break;
    case 2:
    case "Italic":
      classToken.push("italic");
      tag = "i";
      break;
    case 3:
    case "Bold":
      classToken.push("font-semibold");
      tag = "b";
      break;
    case 4:
    case "Underscored":
      classToken.push("underline");
      tag = "u";
      break;
    case 5:
    case "Link":
      classToken.push("link", "font-medium", "hover:underline");
      tag = "a";
      attr = mark.param ? ` href="${escapeAttr(mark.param)}"` : "";
      break;
    case 6:
    case "TextColor":
      classToken.push(`text-${mark.param}`);
      tag = "span";
      break;
    case 7:
    case "BackgroundColor":
      classToken.push(`bg-${mark.param}`);
      tag = "span";
      break;
    // case 8:
    // case "Mention":
    // case 9:
    // case "Emoji":
    // case 10:
    // case "Object":
    default:
      return null;
  }

  const classAttr = classToken.length ? ` class="${classToken.join(" ")}"` : "";
  const open = `<${tag}${classAttr}${attr}>`;
  const close = `</${tag}>`;
  return { start, end, open, close };
};

/**
 * Render `text` with inline `marks` as safe HTML.
 *
 * Walks the text once, splitting it at every mark boundary, escapes each
 * segment, and wraps it in the tags of every mark active across that segment.
 * Overlapping marks become combined tags on the shared span instead of
 * corrupting each other (the old code spliced tagged strings at stale indices
 * and never escaped, so `<T>` or an overlapping `<kbd>` leaked raw markup into
 * the page).
 */
export const resolveMarks = (marks: Mark[] | unknown, text: string): string => {
  const source = text + "";

  let marksList: Mark[] = marks as Mark[];
  if (!isArrayLike(marksList)) {
    if (marksList === undefined) {
      console.warn("unresolved marks", { marks });
      return escapeHtml(source);
    }
    // Legacy: some callers pass a wrapped object { marks: [...] }
    const wrapped = marks as { marks?: unknown };
    marksList = isArrayLike(wrapped.marks) ? (wrapped.marks as Mark[]) : [];
  }

  const resolved = marksList
    .map(resolveMarkTags)
    .filter((m): m is ResolvedMark => m !== null);

  if (!resolved.length) return escapeHtml(source);

  // Boundary points across all mark ranges; segments between them carry a
  // stable set of active marks, so wrapping is always well-nested.
  const points = new Set<number>([0, source.length]);
  for (const m of resolved) {
    points.add(Math.max(0, m.start));
    points.add(Math.min(source.length, m.end));
  }
  const bounds = [...points].sort((a, b) => a - b);

  let out = "";
  for (let i = 0; i + 1 < bounds.length; i++) {
    const p = bounds[i]!;
    const q = bounds[i + 1]!;
    if (q <= p) continue;
    // A mark is active across [p, q) iff it covers the whole segment.
    const active = resolved.filter((m) => m.start <= p && m.end >= q);
    const seg = escapeHtml(source.slice(p, q));
    out +=
      active.map((m) => m.open).join("") +
      seg +
      active
        .slice()
        .reverse()
        .map((m) => m.close)
        .join("");
  }

  return out;
};
export const headerIdResolver = (text: string, id?: string) =>
  `${pathResolver(text || "")}_${id? id.slice(-8) : ''}`;

export const pathResolver = (path: string) =>
  path
    .replace(/\s+/g, "_")
    .replace(/\W/g, "")
    .toLowerCase()
    .split("_")
    .filter((e) => !isEmpty(e))
    .join("-");

/**
 * Resolves a ComponentStyle (or string style) to an array of Tailwind class tokens.
 * Accepts ComponentStyle objects, strings (e.g. FileStyle), or null/undefined.
 */
export const resolveStyle = (style: ComponentStyle | string | null | undefined): string[] => {
  if (!style) return [];
  if (typeof style === "string") return [];
  let styleToken: string[] = [];

  if (style.backgroundColor && style.backgroundColor !== "") {
    styleToken.push(`bg-${style.backgroundColor}-300/30`);
  }

  return styleToken;
};
