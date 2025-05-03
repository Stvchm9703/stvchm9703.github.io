// import { BTreeMap } from 'immutable'; // Assuming you are using Immutable.js for BTreeMap
// import { SnapshotWithType, Error, Tag, PbBlock, ContentBlock } from './your-types'; // Adjust the import according to your types

import { stack } from "d3";
import type { Block as PbBlock } from "../../protos/anytype/models";
import type { SnapshotWithType } from "../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "./attribute";
import type { Collection, CollectionId } from "./collection";
import { getFieldValue, GLOBAL_RELATION_IDMAP } from "./common";
import type { DataMap } from "./common";
import type { ContentBlockList, ContentBlockMap } from "./content_block";
import * as ContentBlock from "./content_block";
import { ObjectTypes } from "./enum_token";
import { ExternalBookmarkLink } from "./external_link";
import type { TagList, TagMap, Tag, TagOptionLabel } from "./tag";
import type { UserId } from "./user";
import type { WorkspaceId } from "./workspace";
import {
  FileObject,
  exportForMetaData,
  OpenGraphImage,
  OpenGraphVideos,
  OpenGraphAudio,
} from "./file_object";

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
  serie: TagOptionLabel | null;
  contents: ContentBlockMap;
  reformedContents: ContentBlockList;
  coverImage: string | null;
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  // raw_contents: any[]; // Adjust the type according to your needs
  tableOfContents: PageLink[];
  relatedChapters: PageLink[];
  relatedPosts: PageLink[];
  meta: {
    images?: OpenGraphImage[];
    videos?: OpenGraphVideos[];
    audio?: OpenGraphAudio[];
  };
}

export interface PageLink {
  id: PageId;
  title: string;
  componentId: string;
  url: string;
  coverImage?: string;
  snippet?: string;
  tags?: TagOptionLabel[];
  serie?: TagOptionLabel;
  publish_date?: number;
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
    serie: null,
    contents: new Map<string, ContentBlock.ContentBlock>(), // Adjust according to your type
    reformedContents: [] as ContentBlockList, // Adjust according to your type
    // raw_contents: [],
    //
    tableOfContents: [],
    relatedChapters: [],
    relatedPosts: [],
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

  tmp.serie = getFieldValue(detailMap, "Series");

  resolveContent(tmp, data?.blocks ?? []);

  resolveReformedContents(tmp);

  resolveTOCComponent(tmp);

  resolveTextNumberComponent(tmp);
  resolveTextToggleComponent(tmp);
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
  // console.log(rawTags);
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
        // block.componentType === "Relation" ||
        block.componentType === "Div"

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

let _seriesRel: Tag | null = null;

export function resolveSeries(self: Page): Page {
  if (self.serie === null || self.serie.length === 0) return null;

  if (_seriesRel == null) {
    _seriesRel = GLOBAL_RELATION_IDMAP.values().find(
      (elm) => elm.name === "Series",
    );
  }
  // console.log(self.serie);

  const resolved = self.serie
    .map((elm) => {
      const tagObj = _seriesRel.options.find((e) => e.id === elm);
      return tagObj
        ? ({
            id: tagObj.id,
            name: tagObj.name,
          } as TagOptionLabel)
        : elm;
    })
    .filter((elm) => elm !== null);

  self.serie = { ...resolved[0] };
  return self;
}

export function resolveTOCComponent(self: Page): Page {
  const resolved = self.reformedContents
    .filter(
      (elm) =>
        elm.componentType === "Text" &&
        elm.componentAttr["style"]?.startsWith("Header"),
    )
    .map((elm) => {
      const eid =
        elm.componentAttr["text"]
          .replace(/\s+/g, "_")
          .replace(/\W/g, "")
          .toLowerCase() +
        "_" +
        elm.id.slice(-6);

      return {
        id: eid,
        componentId: elm.id,
        title: elm.componentAttr["text"].toString(),
        url: "#" + eid,
        level: Number.parseInt(
          elm.componentAttr["style"]?.replace("Header", ""),
        ),
      } as PageLink;
    })
    .filter((elm) => elm !== null);

  // console.log(resolved);

  self.tableOfContents = resolved;
  return self;
}

export function resolveLinkComponent(
  self: Page,
  ref: ExternalBookmarkLink[],
): Page {
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
}

export function resolveTextNumberComponent(self: Page): Page {
  // let number_set: ContentBlock.ContentBlock[][] = [[]];
  let mapped = new Map<string, ContentBlock.ContentBlock[]>();
  let stacked: ContentBlock.ContentBlock[] = [];
  let to_remove: ContentBlock.ContentBlock[] = [];

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

export function resolveTextToggleComponent(self: Page): Page {
  // let number_set: ContentBlock.ContentBlock[][] = [[]];
  let mapped = new Map<string, ContentBlock.ContentBlock[]>();
  let stacked: ContentBlock.ContentBlock[] = [];
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
}
import * as Jupyter from "./jupyter";

const _jupyter_component_regex =
  /^\/custom_component\:jupyter\:\((?<fileName>[\w\-\.]+)\:(?<cellNumber>\d+)\)\/$/;

export function resolveJupyterComponent(
  self: Page,
  jupyters: Jupyter.JupyterNotebookRoot[],
) {
  let toDelete: string[] = [];
  let list_buffer = self.reformedContents
    .map((elm, idx) => {
      if (
        !(elm.componentType === "Text" && elm.componentAttr["style"] === "Code")
      )
        return elm;

      const { text } = elm.componentAttr;
      if (!text.match(_jupyter_component_regex)) return elm;

      let next_item = self.reformedContents[idx + 1];
      if (next_item.componentType !== "File") return null;
      // console.log({ next_item });
      // if (!next_item.componentAttr["fileName"] ) return null;

      const { fileName, cellNumber } = text.match(
        _jupyter_component_regex,
      ).groups;
      // console.log({ fileName, cellNumber });

      const targetJupyter = jupyters.find((jupyter) =>
        jupyter.fileUrl?.includes(fileName),
      );
      if (!targetJupyter) return elm;

      const cell = targetJupyter.cells[cellNumber];
      if (!cell) return elm;
      // console.log({ cell });
      if (cell.cell_type === "code") {
        const { outputs } = cell as Jupyter.CodeCell;
        for (const output of outputs) {
          if (output.data["text/html"]) {
            const html_str = output.data["text/html"];
            let result = Jupyter.resolveJupyterCellOutput(html_str);
            output.data["text/html"] = result;
          }
          if (output.data["text/plain"]) {
            output.data["text/plain"] = undefined;
          }
        }
        (targetJupyter.cells[cellNumber] as Jupyter.CodeCell).outputs = outputs;
      }

      elm.componentType = "JupyterComponent";
      elm.componentAttr = {
        ...elm.componentAttr,
        fileName,
        cellNumber,
        cell,
      };
      toDelete.push(next_item.id);

      // const code = cell.source.join("");
      // const output = cell.outputs.map((output) => output.text).join("\n");

      return elm;
    })
    .filter((p) => p !== null);
  self.reformedContents = list_buffer.filter(
    (p) => toDelete.includes(p.id) === false,
  );

  return self;
}

export function resolveRelationCustomComponent(
  self: Page,
  extFileList: FileObject[],
): Page {
  let toDelete: string[] = [];
  let list_buffer = self.reformedContents
    .map((elm, idx) => {
      if (elm.componentType !== "Relation") return elm;
      const fieldKey = elm.componentAttr["key"];
      const tag = GLOBAL_RELATION_IDMAP.values().find(
        (p) => p.relationKey === fieldKey,
      );
      if (tag === undefined) return null;

      if (tag.name !== "custom-component") return null;
      let next_item = self.reformedContents[idx + 1];
      if (next_item.componentType !== "File") return null;

      const targetFileObject = extFileList.find(
        (f) => f.id === next_item.componentAttr["targetObjectId"],
      );

      const customComponentFieldValue = self.attributes[fieldKey];
      const commandList: string[] = customComponentFieldValue
        .split(";")
        .filter((p) => p !== "");

      const targetCommand = commandList.find((f) =>
        f.includes(targetFileObject?.title),
      );

      elm.componentType = "CustomComponent";
      // elm.componentAttr["fileId"] = next_item.componentAttr["targetObjectId"];
      elm.componentAttr["targetCommand"] = targetCommand;
      elm.componentAttr["origanalCommand"] = commandList;
      // elm.componentAttr["fileAttr"] = next_item.componentAttr;
      // targetFileObject?.attributes = undefined;
      elm.componentAttr["fileObject"] = {
        id: targetFileObject?.id,
        title: targetFileObject?.title,
        fileUrl: targetFileObject?.fileUrl,
        fileExt: targetFileObject?.fileExt,
      };

      toDelete.push(next_item.id);
      return elm;
    })
    .filter((elm) => elm !== null);

  // console.log({ toDelete });
  // console.log(list_buffer.filter(elm => elm.))

  self.reformedContents = list_buffer.filter(
    (p) => toDelete.includes(p.id) === false,
  );

  return self;
}

export function resolveFileComponent(self: Page, extFileList: FileObject[]) {
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
}
