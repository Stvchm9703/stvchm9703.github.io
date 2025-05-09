import type { ContentBlock } from "./index";
import type { Page } from "../page";

/// !deprecated ,move to content_block.ts
export const resolveTextToggleComponent = (self: Page): Page => {
  // let number_set: ContentBlock.ContentBlock[][] = [[]];
  let mapped = new Map<string, ContentBlock[]>();
  let stacked: ContentBlock[] = [];
  // let to_remove: ContentBlock.ContentBlock[] = [];
  let to_remove_id: string[] = [];
  for (const elm of self.reformedContents) {
    if (
      elm.componentType === "Text" &&
      elm.componentAttr["style"]?.startsWith("Toggle")
    ) {
      const children = elm.childrenIds
        .map((id) => {
          return self.reformedContents.find((subElm) => subElm.id === id);
        })
        .filter((p) => p !== undefined);

      elm.componentAttr["items"] = children;
      to_remove_id.push(...elm.childrenIds);
    }
  }

  self.reformedContents = self.reformedContents.filter(
    (elm) => to_remove_id.includes(elm.id) === false,
  );

  return self;
};
