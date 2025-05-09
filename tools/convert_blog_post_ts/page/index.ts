// import { BTreeMap } from 'immutable'; // Assuming you are using Immutable.js for BTreeMap
// import { SnapshotWithType, Error, Tag, PbBlock, ContentBlock } from './your-types'; // Adjust the import according to your types

import Bun from "bun";

import type { Block as PbBlock } from "../../../protos/anytype/models";
import { SnapshotWithType } from "../../../protos/anytype/snapshot";
import { castToAttributeMap, type AttributeMap } from "../attribute";
import type { Collection, CollectionId } from "../collection";
import {
  getFieldValue,
  getShortenId,
  GLOBAL_RELATION_IDMAP,
  pathResolver,
  type DataMap,
} from "../common";
import type { ContentBlockList, ContentBlockMap } from "../content_block/index";
import * as ContentBlock from "../content_block/index";
import { ObjectTypes } from "../enum_token";
import type { ExternalBookmarkLink } from "../external_link";
import type { TagList, TagMap, Tag, TagOptionLabel } from "../tag";
import type { UserId } from "../user";
import type { WorkspaceId } from "../workspace";
import {
  type FileObject,
  exportForMetaData,
  type OpenGraphImage,
  type OpenGraphVideos,
  type OpenGraphAudio,
} from "../file_object";

import {
  resolveSeries,
  resolveTOCComponent,
  resolveTags,
} from "./resolve_attribute";

import {
  resolveFileComponent,
  resolveLinkComponent,
  resolveContent,
} from "./resolve_content_block";

export type PageId = string;
export type PageMap = Map<string, Page>;

export type FuncPageAttrbuteResolver = (page: Page) => Page;
export type FuncPageExternalResolver = (page: Page, ...args: any[]) => Page;

export interface Page {
  id: PageId;
  _sid: PageId;
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
  serie?: TagOptionLabel | null;
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
    _sid: "",
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
  tmp._sid = getShortenId(raw);
  tmp.title = getFieldValue(detailMap, "name") ?? "";
  tmp.attributes = detailMap ?? {};
  tmp.collectionId =
    getFieldValue<string[]>(detailMap, "backlinks")?.slice(-1)[0] ?? "";
  tmp.workspace_id = getFieldValue(detailMap, "spaceId") ?? "";
  tmp.creator = getFieldValue(detailMap, "creator") ?? "";
  tmp.snippet = getFieldValue(detailMap, "snippet") ?? "";
  tmp.raw_tag_list = getFieldValue(detailMap, "tag") ?? [];
  tmp.publish_date = getFieldValue<number>(detailMap, "publish date");

  let serie = getFieldValue<string[]>(detailMap, "Series");
  if (serie !== null && serie.length > 0) tmp.serie = serie;

  /// rewrite the pipeline

  resolveContent(tmp, data?.blocks ?? []);
  // resolveReformedContents(tmp);
  // resolveTOCComponent(tmp);
  // resolveTextNumberComponent(tmp);
  // resolveTextToggleComponent(tmp);
  snapshotAsJson(tmp._sid, raw);

  return tmp;
}

function snapshotAsJson(title: string, raw: SnapshotWithType) {
  Bun.write(
    `blog_post_resolved/tmp/${title}.json`,
    JSON.stringify(SnapshotWithType.toJSON(raw), null, 2),
  );
}

// import {  } from './resolve_attribute';

// tampered
import { resolveJupyterComponent } from "../content_block/jupyter_nb";

export {
  resolveSeries,
  resolveTOCComponent,
  resolveTags,
  resolveFileComponent,
  resolveLinkComponent,
  resolveContent,
  resolveJupyterComponent,
};
