import { isArrayLike } from "lodash-es";

import type { TextMark, TextMarkType } from "$generateor/common";
export const resolveMarks = (marks: TextMark[], text: string): string => {
  let result = text + "";
  const resultSet = [];
  let marksList = marks;
  if (!isArrayLike(marksList)) {
    if (marksList === undefined) {
      console.warn("unresolved marks", { marks });
      return result;
    }
    if (isArrayLike(marks.marks)) {
      marksList = marks.marks;
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

    switch (mark.type) {
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
export const headerIdResolver = (text: string, id: string) =>
  `${pathResolver(text || "")}-${id.slice(-6)}`;

export const pathResolver = (path: string) =>
  path.replace(/\s+/g, "_").replace(/\W/g, "").toLowerCase();

export const resolveStyle = (style) => {
  if (!style) return [];
  let styleToken = [];

  if (style["backgroundColor"] && style["backgroundColor"] !== "") {
    styleToken.push(`bg-${style["backgroundColor"]}-300/30`);
  }

  if (style["textColor"] && style["textColor"] !== "") {
    styleToken.push(`text-${style["textColor"]}-700`);
  }

  return styleToken;
};
