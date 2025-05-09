import type { ContentBlock, ContentBlockMap } from "./index";
import type { Page } from "../page";
/// !deprecated ,move to content_block.ts
export function resolveTextNumberComponent(self: Page): Page {
  // let number_set: ContentBlock.ContentBlock[][] = [[]];
  let mapped = new Map<string, ContentBlock[]>();
  let stacked: ContentBlock[] = [];
  let to_remove: ContentBlock[] = [];

  for (const elm of self.reformedContents) {
    if (
      elm.componentType === "Text" &&
      (elm.componentAttr["style"]?.startsWith("Numbered") ||
        elm.componentAttr["style"]?.startsWith("Marked"))
    ) {
      stacked.push(elm);
    } else if (stacked.length > 0) {
      let last_element = stacked[stacked.length - 1];
      mapped.set(last_element.id, stacked);
      to_remove.push(...stacked.slice(0, -1));

      stacked = [];
    }
  }

  // console.log(mapped);

  let to_remove_id = to_remove.map((elm) => elm.id);

  // console.log(to_remove_id);
  self.reformedContents = self.reformedContents.filter(
    (elm) => to_remove_id.includes(elm.id) === false,
  );
  mapped.forEach((value, key) => {
    let items = value.map((kelm) => ({
      id: kelm.id,
      text: kelm.componentAttr["text"],
      style: kelm["style"],
      marks: kelm.componentAttr["marks"]["marks"],
    }));

    let target = self.reformedContents.find((elm) => elm.id === key);
    if (target) {
      target.componentAttr["items"] = items;
      target.componentAttr["text"] = undefined;
    }
  });

  return self;
}

export const resolveTextNumberComponentV2 = (
  self: ContentBlock,
  ref: ContentBlockMap,
  // idList: string[],
): [ContentBlock, string[]] => {
  const toDel: string[] = [];

  return [self, toDel];
};
