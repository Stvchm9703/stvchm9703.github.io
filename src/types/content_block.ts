/**
 * Content-block types for blog posts.
 *
 * These mirror the JSON emitted by `tools/convert_blog_post` (Rust) in **release
 * mode** and consumed by the post renderer in
 * `src/lib/components/post-content-layout/block/*`.
 *
 * Source of truth: `tools/convert_blog_post/src/export_model/content_block/**`.
 * See `docs/data_model.md` for the full schema and `docs/features.md` for the
 * known blog/tool mismatches (e.g. `marks` shape).
 *
 * NOTE: debug-only fields (`id`/`childrenIds` on text/layout attrs, `debugType`)
 * are typed as optional because they are stripped from release output.
 */

// ---------------------------------------------------------------------------
// Enums (string unions matching serde Debug formatting)
// ---------------------------------------------------------------------------

export type Align = "AlignLeft" | "AlignCenter" | "AlignRight" | "AlignJustify";

export type VerticalAlign =
  | "VerticalAlignTop"
  | "VerticalAlignMiddle"
  | "VerticalAlignBottom";

export type TextStyle =
  | "Paragraph"
  | "Header1"
  | "Header2"
  | "Header3"
  | "Header4"
  | "Quote"
  | "Code"
  | "Title"
  | "Checkbox"
  | "Marked"
  | "Numbered"
  | "Toggle"
  | "Description"
  | "Callout";

export type MarkType =
  | "Strikethrough"
  | "Keyboard"
  | "Italic"
  | "Bold"
  | "Underscored"
  | "Link"
  | "TextColor"
  | "BackgroundColor"
  | "Mention"
  | "Emoji"
  | "Object";

export type LayoutStyle =
  | "Div"
  | "Row"
  | "Column"
  | "Header"
  | "Table"
  | "TableColumns"
  | "TableRows"
  | "TableRow"
  | "TableCol";

export type FileKind = "None" | "File" | "Image" | "Video" | "Audio" | "PDF";
export type FileStyle = "Auto" | "Link" | "Embed";
export type FileState = "" | "Empty" | "Uploading" | "Done" | "Error";

export type ComponentType =
  | "Text"
  | "Layout"
  | "Table"
  | "File"
  | "Link"
  | "Bookmark"
  | "Latex"
  | "JupyterComponent"
  | "Unknown"
  | "None";

// ---------------------------------------------------------------------------
// Shared structures
// ---------------------------------------------------------------------------

export interface ComponentStyle {
  backgroundColor: string;
  align: Align;
  verticalAlign: VerticalAlign;
}

export interface MarkRange {
  from: number;
  to: number;
}

/** A single inline mark/span. NOTE: emitted as a flat `Mark[]`, not `{ marks }`. */
export interface Mark {
  range: MarkRange;
  type: MarkType;
  param: string | null;
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

export interface TextComponentAttr {
  componentType: "Text";
  order: number;
  text: string;
  style: TextStyle;
  marks?: Mark[];
  /** Present only for grouped Numbered / Marked / Toggle lists. */
  items?: TextItem[];
  /** debug-only */
  id?: string;
  /** debug-only */
  childrenIds?: string[];
}

/** Discriminated on `_text_item_type`. */
export type TextItem = TextItemLevel | TextItemOther;

/** `Block` and `LevelText` are flattened `TextComponentAttr`s. */
export interface TextItemLevel {
  _text_item_type: "Block" | "LevelText";
  order: number;
  text: string;
  style: TextStyle;
  marks?: Mark[];
  items?: TextItem[];
  id?: string;
  childrenIds?: string[];
}

/** `Other` is a full nested ContentBlock. */
export type TextItemOther = ContentBlock & { _text_item_type: "Other" };

// ---------------------------------------------------------------------------
// Layout / Table
// ---------------------------------------------------------------------------

/**
 * Layout & Table share one shape. When it is the `componentAttr` of a top-level
 * block it carries `componentType`; when nested inside `items` (a sub-layout)
 * it does NOT.
 */
export interface LayoutComponentAttr {
  componentType?: "Layout" | "Table";
  order: number;
  isHeader?: boolean;
  style: ComponentStyle;
  layoutStyle: LayoutStyle;
  items?: LayoutItem[];
  /** debug-only */
  id?: string;
  /** debug-only */
  childrenIds?: string[];
}

/**
 * Untagged union: a leaf is a full `ContentBlock` (has `componentAttr`); a
 * sub-layout is a `LayoutComponentAttr` (has `layoutStyle`, no `componentAttr`).
 */
export type LayoutItem = ContentBlock | LayoutComponentAttr;

// ---------------------------------------------------------------------------
// File / Link / Bookmark / Latex / Jupyter / Unknown
// ---------------------------------------------------------------------------

export interface FileComponentAttr {
  componentType: "File";
  name: string;
  type: FileKind;
  mime: string;
  size: number;
  targetObjectId: string;
  state: FileState;
  style: FileStyle;
  fileUrl: string;
}

export interface LinkComponentAttr {
  componentType: "Link";
  title: string;
  description: string;
  url: string;
  targetBlockId: string;
  iconSize: string;
  cardStyle: string;
}

export interface BookmarkComponentAttr {
  componentType: "Bookmark";
  url: string;
  title: string;
  description: string;
  imageHash: string;
  faviconHash: string;
  type: string;
  targetObjectId: string;
}

export interface LatexComponentAttr {
  componentType: "Latex";
  text: string;
  /** e.g. "Latex" | "Mermaid" | "Drawio" */
  processor: string;
}

export interface JupyterComponentAttr {
  componentType: "JupyterComponent";
  text: string;
  style: TextStyle;
  marks?: Mark[];
  fileName: string;
  cellNumber: number;
  cell: JupyterCell | null;
}

/** Catch-all for relation/dataview/featuredRelations/div/icon/widget/chat/etc. */
export interface UnknownComponentAttr {
  componentType: "Unknown";
  /** sic — the output key is misspelled "orginal_type". */
  orginal_type: string;
  style?: string;
  [key: string]: unknown;
}

export interface NoneComponentAttr {
  componentType: "None";
}

export type ComponentAttr =
  | TextComponentAttr
  | LayoutComponentAttr
  | FileComponentAttr
  | LinkComponentAttr
  | BookmarkComponentAttr
  | LatexComponentAttr
  | JupyterComponentAttr
  | UnknownComponentAttr
  | NoneComponentAttr;

// ---------------------------------------------------------------------------
// ContentBlock
// ---------------------------------------------------------------------------

export interface ContentBlock {
  id: string;
  order: number;
  childrenIds: string[] | null;
  /** misc raw block fields, e.g. `{ lang: "cs" }` on code blocks */
  fields?: Record<string, unknown>;
  style: ComponentStyle;
  componentAttr: ComponentAttr;
  /** debug-only */
  debugType?: string;
}

// ---------------------------------------------------------------------------
// Jupyter notebook cell model
// (tools/convert_blog_post/src/jupyter_notebook/model.rs)
// ---------------------------------------------------------------------------

export type JupyterCell = JupyterMarkdownCell | JupyterCodeCell;

export interface JupyterMarkdownCell {
  cell_type: "markdown";
  metadata: Record<string, unknown>;
  source: string[];
  attachments?: Record<string, unknown>;
}

export interface JupyterCodeCell {
  cell_type: "code";
  execution_count?: number;
  metadata: Record<string, unknown>;
  source: string[];
  outputs?: JupyterOutput[];
}

export type JupyterOutputType =
  | "none"
  | "display_data"
  | "error"
  | "execute_result"
  | "stream";

export interface JupyterOutput {
  output_type: JupyterOutputType;
  data?: JupyterOutputData;
  execution_count?: number;
  metadata?: { needs_background?: "light" };
  name?: string;
  text?: string[];
  ename?: string;
  evalue?: string;
  traceback?: string[];
}

export interface JupyterOutputData {
  "text/plain": string[];
  "text/html"?: string[];
  "image/png"?: string;
}
