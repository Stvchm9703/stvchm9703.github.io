import type {
  ContentBlock,
  ContentBlockBasicStyle,
  ContentBlockMap,
} from "./index";

interface FlexItem {
  id: string;
  style: ContentBlockBasicStyle;
  order: number;
  children?: Array<FlexItem | ContentBlock>;
  componentType: string;
  [key: string]: any;
}

export function resolveLayoutRowComponent(
  self: ContentBlock,
  refs: ContentBlockMap,
): [ContentBlock, string[]] {
  const toDel: string[] = [];
  // should
  if (self.id.startsWith("r-")) {
    // const result = self;
    const columns = self.childrenIds
      .map((id) => refs.get(id))
      .filter((p) => p !== undefined)
      .map((col, colIndex) => {
        const colChildren = col.childrenIds
          .map((id) => refs.get(id))
          .filter((p) => p !== undefined)
          .map((child, childIdx) => {
            toDel.push(child.id);
            return {
              ...child,
              order: childIdx,
            } as ContentBlock;
          });

        toDel.push(col.id);
        return {
          id: col.id,
          style: col.style,
          order: colIndex,
          children: colChildren,
          componentType: "LayoutColumn",
        } as FlexItem;
      });
    // self.columns = columns;
    self.componentAttr["columns"] = columns;
    self.componentType = "LayoutRow";
  }

  return [self, toDel];
}
