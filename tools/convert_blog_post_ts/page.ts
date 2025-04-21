// import { BTreeMap } from 'immutable'; // Assuming you are using Immutable.js for BTreeMap
// import { SnapshotWithType, Error, Tag, PbBlock, ContentBlock } from './your-types'; // Adjust the import according to your types

import type { Block as PbBlock } from "../../protos/anytype/models";
import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "./attribute";
import type { CollectionId } from "./collection";
import { getFieldValue } from "./common";
import type { DataMap } from "./common";
import type { ContentBlockList, ContentBlockMap } from "./content_block";
import * as ContentBlock from "./content_block";
import { ObjectTypes } from "./enum_token";
import type { TagList, TagMap, Tag, TagOptionLabel } from "./tag";
import type { UserId } from "./user";
import type { WorkspaceId } from "./workspace";

export type PageId = string;
export type PageMap = Map<string, Page>;

export interface Page {
  id: PageId;
  title: string;
  description: string;
  snippet: string;
  collectionId: CollectionId;
  collection: Collection;
  attributes: DataMap;
  workspace_id: WorkspaceId;
  creator: UserId;
  publish_date: number | null;
  tags: TagOptionLabel[];
  raw_tag_list: string[];
  styles: DataMap;
  serie: string | null;
  contents: ContentBlockMap;
  reformedContents: ContentBlockList;
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  // raw_contents: any[]; // Adjust the type according to your needs
}

export const getPages = (snapshot_list: SnapshotWithType[]): Page[] => {
  return snapshot_list
    .filter((snapshot) =>
      snapshot.snapshot?.data?.objectTypes.includes(
        ObjectTypes.Page.toString(),
      ),
    )
    .map(fromAnytype);
};

export function fromAnytype(raw: SnapshotWithType): Page {
  const tmp: Page = {
    id: "",
    title: "",
    description: "",
    snippet: "",
    collectionId: "" as CollectionId, // Adjust according to your type
    attributes: {} as AttributeMap, // Adjust according to your type
    workspace_id: "" as WorkspaceId, // Adjust according to your type
    creator: "" as UserId, // Adjust according to your type
    publish_date: null,
    tags: [] as TagOptionLabel[], // Adjust according to your type
    raw_tag_list: [],
    styles: {} as DataMap, // Adjust according to your type
    serie: "",
    contents: new Map<string, ContentBlock.ContentBlock>(), // Adjust according to your type
    reformedContents: [] as ContentBlockList, // Adjust according to your type
    // raw_contents: [],
  };

  const data = raw.snapshot?.data;
  const detailMap = data?.details;

  tmp.id = getFieldValue(detailMap, "id") ?? "";
  tmp.title = getFieldValue(detailMap, "name") ?? "";
  tmp.attributes = detailMap ?? {};
  tmp.collectionId =
    getFieldValue<string[]>(detailMap, "backlinks")?.slice(-1)[0] ?? "";
  tmp.workspace_id = getFieldValue(detailMap, "spaceId") ?? "";
  tmp.creator = getFieldValue(detailMap, "creator") ?? "";
  tmp.snippet = getFieldValue(detailMap, "snippet") ?? "";
  tmp.raw_tag_list = getFieldValue(detailMap, "tag") ?? [];
  tmp.publish_date = getFieldValue<number>(detailMap, "publish date");
  // if (rawPublishDate) {
  // tmp.publish_date = new Date(rawPublishDate);
  // }

  tmp.serie = getFieldValue(detailMap, "series");
  resolveContent(tmp, data?.blocks ?? []);

  resolveReformedContents(tmp);

  // console.log("resolved :", tmp);
  return tmp;
}

function resolveContent(self: Page, rawContent: PbBlock[]): Page {
  rawContent.forEach((block, idx) => {
    const cb_resolved = ContentBlock.fromBlock(idx, block);
    self.contents.set(cb_resolved.id.toString(), cb_resolved);
  });

  return self;
}

export function resolveTags(self: Page, rawTags: Tag): Page {
  console.log(rawTags);
  const resolved = self.raw_tag_list
    .map((elm) => {
      const tagObj = rawTags.options.find((e) => e.id === elm);
      return tagObj
        ? ({
            id: tagObj.id,
            name: tagObj.name,
          } as TagOptionLabel)
        : null;
    })
    .filter((elm) => elm !== null);
  self.tags = resolved;
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
        block.componentType === "FeaturedRelations" ||
        block.componentType === "Relation" ||
        block.componentType === "TableOfContents"
      )
        return null;
      outerLoopCount++;
      block.order = outerLoopCount;
      return block;
    })
    .filter((e) => e !== null)
    .map((elm) =>
      elm.componentType === "Table"
        ? ContentBlock.resolveTableComponent(elm, self.contents)
        : elm,
    );

  self.reformedContents = blockList;

  return self;
}

function resolveChildrenIds(self: Page, id: string): string[] {
  const rootBlock = self.contents.get(id);
  if (!rootBlock) return [];

  // console.log(
  //   rootBlock.id,
  //   rootBlock.componentType,
  //   rootBlock.childrenIds.length,
  //   JSON.stringify({
  //     attr: rootBlock.componentAttr,
  //     fields: rootBlock.fields,
  //   }),
  // );

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
