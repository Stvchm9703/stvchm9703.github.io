import { isArrayLike, isEmpty } from "lodash-es";
// import { pathResolver as pr } from "$lib/utils";
import type { Mark, ComponentStyle } from "$generateor/content_block";

/** Internal mark type that tolerates legacy numeric values from older JSON. */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type MarkTypeOrLegacy = any;

export const resolveMarks = (marks: Mark[] | unknown, text: string): string => {
  let result = text + "";
  const resultSet = [];
  let marksList: Mark[] = marks as Mark[];
  if (!isArrayLike(marksList)) {
    if (marksList === undefined) {
      console.warn("unresolved marks", { marks });
      return result;
    }
    // Legacy: some callers pass a wrapped object { marks: [...] }
    const wrapped = marks as { marks?: unknown };
    if (isArrayLike(wrapped.marks)) {
      marksList = wrapped.marks as Mark[];
    }
  }

  for (const mark of marksList) {
    const capture = text.substring(
      mark.range?.from || 0,
      mark.range?.to || text.length,
    );
    let tag = "";
    const classToken: string[] = [];

    let attr = "";

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const markType: MarkTypeOrLegacy = (mark as any).type;
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
        attr = `href="${mark.param}"`;
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
      //   // return Block_Content_Text_Mark_Type.Mention;
      // case 9:
      // case "Emoji":
      // // return Block_Content_Text_Mark_Type.Emoji;
      // case 10:
      // case "Object":
      // return Block_Content_Text_Mark_Type.Object;
    }

    const tmp = `<${tag} class="${classToken.join(" ")}" ${attr}>${capture}</${tag}>`;
    resultSet.push({
      text: tmp,
      start: mark.range?.from || 0,
      end: mark.range?.to || text.length,
    });
  }

  resultSet.sort((a, b) => b.end - a.end);
  resultSet.forEach((item) => {
    // result = result.replace(text.substring(item.start, item.end), item.text);
    result =
      result.substring(0, item.start) +
      item.text +
      result.substring(item.end, result.length);
  });

  return result;
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
