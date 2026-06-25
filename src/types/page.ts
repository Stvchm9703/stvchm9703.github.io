/**
 * Page / index types for blog posts.
 *
 * These mirror the JSON emitted by `tools/convert_blog_post` (Rust) in **release
 * mode**. Source of truth:
 * `tools/convert_blog_post/src/export_model/page/**`. See `docs/data_model.md`.
 */

import type { ContentBlock } from "./content_block";

// ---------------------------------------------------------------------------
// Links
// ---------------------------------------------------------------------------

export interface PageMiniExternalLink {
  id: string;
  _sid: string;
  label: string;
  url: string;
}

/**
 * Generic link used for tags, series, TOC entries and related-post lists.
 * Optional fields are only present in the contexts noted in `docs/data_model.md`.
 */
export interface PageExternalLink {
  id: string;
  _sid: string;
  /** TOC entries only: id of the source heading block. */
  componentId?: string;
  label: string;
  url: string;
  /** TOC entries only: heading level 1–4. */
  level?: number;
  /** series/tag list entries. */
  snippet?: string;
  /** series/tag list entries. */
  tags?: PageMiniExternalLink[];
  /** series/tag list entries. */
  serie?: PageMiniExternalLink;
  /** related / series / tag list entries — unix seconds. */
  publishDate?: number;
}

// ---------------------------------------------------------------------------
// OpenGraph / meta
// ---------------------------------------------------------------------------

export interface OpenGraphObject {
  url: string;
  secureUrl?: string;
  type?: string;
  width?: number;
  height?: number;
  alt?: string;
}

export interface PageMeta {
  images: OpenGraphObject[] | null;
  videos: OpenGraphObject[] | null;
  audio: OpenGraphObject[] | null;
}

// ---------------------------------------------------------------------------
// Page
// ---------------------------------------------------------------------------

export interface Page {
  id: string;
  _sid: string;
  title: string;
  snippet: string;
  /** optional long description (Anytype `description` relation). */
  description?: string;
  /** cover/OG image: the bare file name (blog prepends `/blog/assets/files/`). */
  coverImage?: string;
  collectionId: string;
  workspaceId: string;
  /** creator object id (not resolved to a display name). */
  creator: string;
  /** unix seconds (0 when unset). */
  publishDate: number;
  tags: PageExternalLink[];
  /** reserved; currently always null. */
  styles: Record<string, unknown> | null;
  serie: PageExternalLink | null;
  meta: PageMeta;
  tableOfContents: PageExternalLink[];
  relatedChapters: PageExternalLink[];
  relatedArticles: PageExternalLink[];
  contents: ContentBlock[];

  // --- debug-only (absent from release output) ---
  rawAttributes?: Record<string, unknown>;
  rawTagList?: string[];
  rawSeries?: string[] | null;
  checkBlockOrder?: string[];
}

// ---------------------------------------------------------------------------
// Index / series / tags
// ---------------------------------------------------------------------------

/** Entry in the top-level `index.json` (snake_case — not camelCase). */
export interface PageIndexReference {
  id: string;
  post_id: string;
  label: string;
  url: string;
  res: string;
  page_content_path: string;
  publish_date: number;
}

/** Entry in `series/index.json` and `tags/index.json`, and the paged `pN.json`. */
export interface TagIndexEntry {
  id: string;
  _sid: string;
  name: string;
  description: string;
  totalCount: number;
  totalPageNumber?: number;
  pageIndex?: number;
  resultList: PageExternalLink[];
}
