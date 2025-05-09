import * as ContentBlock from "../content_block/index";
import type {
  Page,
  FuncPageExternalResolver,
  FuncPageAttrbuteResolver,
} from "./index.ts";
import type { Block as PbBlock } from "../../../protos/anytype/models";

import { exportForMetaData } from "../file_object.ts";

// import { FuncPageExternalResolver } from "./index.ts";

/**
 * Resolve content blocks and their children.
  C1. resolve-content
      - create the map<string, content-book>

  C2. sort the content-block-id
      - create the array<content-book-id>

  C3.  the content-block
      - recreate the content block based the type

  C4.
 */

export function resolveContent(self: Page, rawContent: PbBlock[]): Page {
  console.log("resolve-content");

  rawContent.forEach((block, idx) => {
    const cb_resolved = ContentBlock.fromBlock(idx, block);
    self.contents.set(cb_resolved.id.toString(), cb_resolved);
  });

  // 1. get the Smart-block as root block
  const rootBlock = self.contents.get(self.id);
  if (!rootBlock) return self;

  // 2. sort the content-block-id
  let orderlist: string[] = resolveChildrenIds(self, self.id);
  orderlist = orderlist.filter(
    (elm) => ["header", "title", "featuredRelations"].includes(elm) === false,
  );

  // console.log(orderlist);
  const toDeleteList: string[] = [];
  self.contents.forEach((block, idx) => {
    // console.log(block, idx);
    if (block.componentType === "Table") {
      const [bk, toDel] = ContentBlock.resolveTableComponent(
        block,
        self.contents,
      );
      toDeleteList.push(...toDel);
      // console.log({ bk, toDel, objIs: Object.is(block, bk) });
    } else if (block.componentType === "Layout") {
      const [bk, toDel] = ContentBlock.resolveLayoutRowComponent(
        block,
        self.contents,
      );
      // console.log({ bk, toDel, objIs: Object.is(block, bk) });
      toDeleteList.push(...toDel);
    } else if (block.componentType === "File") {
      // const [bk, toDel ] = ContentBlock.resolveFileComponent()
    } else if (block.componentType === "Text") {
      const style = block.componentAttr["style"];
    }
    // block.fields["checked"] = true;
  });

  return self;
}

function resolveReformedContents(self: Page): Page {
  const rootBlock = self.contents.get(self.id);
  if (!rootBlock) return self;

  const orderlist: string[] = resolveChildrenIds(self, self.id);

  // console.log({ orderlist });
  let outerLoopCount = 0;
  const blockList = orderlist
    .map((elm, ord) => {
      const block = self.contents.get(elm);
      if (!block) return null;
      if (
        block.componentType === "Smartblock" ||
        block.componentType === "FeaturedRelations"
        // block.componentType === "Relation" ||
        // block.componentType === "Div"

        // || block.componentType === "TableOfContents"
      )
        return null;
      if (block.id === "header" || block.id === "title") return null;
      outerLoopCount++;
      block.order = outerLoopCount;
      return block;
    })
    .filter((e) => e !== null)
    .map((elm) => {
      if (elm.componentType === "Table")
        return ContentBlock.resolveTableComponent(elm, self.contents);
      if (elm.componentType === "Layout")
        return ContentBlock.resolveLayoutRowComponent(elm, self.contents);
      return elm;
    });

  self.reformedContents = blockList;

  return self;
}

function resolveChildrenIds(self: Page, id: string): string[] {
  const rootBlock = self.contents.get(id);
  if (!rootBlock) return [];
  const orderlist: string[] = [];

  if (rootBlock.componentType !== "Div") {
    orderlist.push(id);
  }

  // skip table
  if (rootBlock.componentType === "Table") {
    return orderlist;
  }

  for (const elm of rootBlock.childrenIds) {
    const children = resolveChildrenIds(self, elm);
    orderlist.push(...children);
  }

  if (rootBlock.componentType === "Div") {
    orderlist.push(id);
  }

  return orderlist;
}

export const resolveLinkComponent: FuncPageExternalResolver = (
  self: Page,
  ref: ExternalBookmarkLink[],
): Page => {
  for (const elm of self.reformedContents) {
    if (elm.componentType === "Link") {
      const target = ref.find(
        (e) => e.id === elm.componentAttr["targetBlockId"],
      );

      if (target) {
        elm.componentAttr["href"] = target.href;
        elm.componentAttr["title"] = target.title;
      }
    }
  }
  return self;
};

export const resolveFileComponent: FuncPageExternalResolver = (
  self: Page,
  extFileList: FileObject[],
) => {
  let included: FileObject[] = [];
  for (const block of self.reformedContents) {
    if (block.componentType === "File") {
      const targetFile = extFileList.find(
        (p) => p.id === block.componentAttr["targetObjectId"],
      );
      if (targetFile) {
        block.componentAttr = {
          ...block.componentAttr,
          fileUrl: targetFile.fileUrl,
        };
        included.push(targetFile);
      }
    }
  }
  self.meta = exportForMetaData(included);
  return self;
};
