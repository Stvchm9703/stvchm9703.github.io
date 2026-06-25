# Blog Post Data Model

This document describes the **JSON data structure emitted by
`tools/convert_blog_post`** (the Rust converter) and consumed by the SvelteKit
blog under `src/`.

- **Source of truth:** the hand-written Rust `serde` models in
  `tools/convert_blog_post/src/export_model/**` — *not* the generated protobuf
  structs. Regenerating the protobuf layer (`pb-rs`) does **not** change this
  output schema; it only changes the parsing layer.
- **Serialization:** `serde` with `rename_all = "camelCase"`.
- **Release vs. debug output:** several fields are gated by
  `#[serde(skip_serializing_if = "is_release")]`. The committed outputs
  (`blog_post_resolved_*`) are **release-shaped** — debug fields are absent.
  They are listed in [Debug-only fields](#debug-only-fields) for completeness.
- Schema verified against `blog_post_resolved_0624/` (28 pages, 2592 blocks).

---

## 1. Output artifacts / directory layout

```
<export>/
├── index.json                      # PageIndexReference[]  — master post index
├── <collection-slug>_<id6>/        # one dir per collection (e.g. article_wc2ak24y)
│   └── <pageId>.json               # Page object (one per post)
├── series/
│   ├── index.json                  # TagIndexEntry[] (preview, 3 posts each)
│   └── <series-slug>_<id6>/pN.json # SeriesPage (paged, 150 posts/page)
├── tags/
│   ├── index.json                  # TagIndexEntry[] (preview, 3 posts each)
│   └── <tag-slug>_<id6>/pN.json    # TagPage (paged, 200 posts/page)
└── files/                          # copied assets (images, pdf, .ipynb, …)
```

Slugs come from `header_id_resolver(name, id)` = `<kebab-name>_<last-6-of-id>`.

---

## 2. `index.json` — `PageIndexReference[]`

`src/export_model/page/ext.rs :: PageIndexReference`

| field               | type     | notes                                            |
| ------------------- | -------- | ------------------------------------------------ |
| `id`                | string   | full Anytype object id (`bafyrei…`)              |
| `post_id`           | string   | last 6 chars of `id` (the `_sid`)                |
| `label`             | string   | post title                                       |
| `url`               | string   | `/posts/<post_id>_<title-slug>`                  |
| `res`               | string   | title slug (the `[res]` route param)             |
| `page_content_path` | string   | path to the Page JSON, relative to `<export>`    |
| `publish_date`      | number   | unix seconds (0 when unset)                      |

> Note: this object is **snake_case** (it does not use the camelCase rename).

---

## 3. `Page` — per-post JSON

`src/export_model/page/mod.rs :: Page`

| field             | type                  | notes                                                        |
| ----------------- | --------------------- | ------------------------------------------------------------ |
| `id`              | string                | full object id                                               |
| `_sid`            | string                | short id (last 6 chars)                                      |
| `title`           | string                |                                                              |
| `snippet`         | string                | plain-text excerpt                                           |
| `collectionId`    | string                | id of the owning collection                                  |
| `workspaceId`     | string                | Anytype `spaceId`                                            |
| `creator`         | string                | creator object id (not resolved to a name)                   |
| `publishDate`     | number                | unix seconds (0 when unset)                                  |
| `tags`            | `PageExternalLink[]`  | resolved tag links                                           |
| `styles`          | object \| null        | reserved; currently always `null`                            |
| `serie`           | `PageExternalLink` \| null | series link, if the post belongs to a series           |
| `meta`            | `PageMeta`            | OpenGraph media collected from file blocks                   |
| `tableOfContents` | `PageExternalLink[]`  | one entry per heading (`level` set)                          |
| `relatedChapters` | `PageExternalLink[]`  | other posts in the same series                               |
| `relatedArticles` | `PageExternalLink[]`  | other posts sharing a tag                                    |
| `contents`        | `ContentBlock[]`      | ordered, resolved content tree (see below)                   |

---

## 4. `ContentBlock`

`src/export_model/content_block/mod.rs :: ContentBlock`

| field          | type                         | notes                                              |
| -------------- | ---------------------------- | -------------------------------------------------- |
| `id`           | string                       | block id                                           |
| `order`        | number                       | resolved order                                     |
| `childrenIds`  | string[] \| null             | raw child ids                                      |
| `fields`       | object (optional)            | misc block fields, e.g. `{ "lang": "cs" }` on code |
| `style`        | `ComponentStyle`             | layout/alignment                                   |
| `componentAttr`| `ComponentAttr`              | discriminated by `componentType` (see §6)          |

### `ComponentStyle`

```jsonc
{ "backgroundColor": "", "align": "AlignLeft", "verticalAlign": "VerticalAlignTop" }
```

- `align`: `AlignLeft | AlignCenter | AlignRight | AlignJustify`
- `verticalAlign`: `VerticalAlignTop | VerticalAlignMiddle | VerticalAlignBottom`

---

## 5. Enums

| enum         | values                                                                                                            |
| ------------ | --------------------------------------------------------------------------------------------------------------- |
| `TextStyle`  | `Paragraph, Header1, Header2, Header3, Header4, Quote, Code, Title, Checkbox, Marked, Numbered, Toggle, Description, Callout` |
| `MarkType`   | `Strikethrough, Keyboard, Italic, Bold, Underscored, Link, TextColor, BackgroundColor, Mention, Emoji, Object`   |
| `LayoutStyle`| `Div, Row, Column, Header, Table, TableColumns, TableRows, TableRow, TableCol`                                   |
| `ComponentType` (`componentAttr.componentType`) | `Text, Layout, Table, File, Link, Bookmark, Latex, JupyterComponent, Unknown, None` |

---

## 6. `ComponentAttr` variants

`componentAttr` is an **internally-tagged union** keyed on `componentType`
(`ComponentAttrType` in `content_block/mod.rs`).

### `Text`
```jsonc
{
  "componentType": "Text",
  "order": 0,
  "text": "…",
  "style": "Paragraph",        // TextStyle
  "marks": [ Mark ],           // flat array (see §7) — may be []
  "items": [ TextItem ]        // present only for grouped Numbered/Marked/Toggle lists
}
```

### `Layout` / `Table`
Both serialize a `LayoutComponentAttr` (`content_block/layout.rs`):
```jsonc
{
  "componentType": "Layout",   // or "Table"
  "order": 0,
  "style": ComponentStyle,
  "layoutStyle": "Div",        // LayoutStyle
  "items": [ LayoutItem ]      // nested children (see §7)
}
```
A `Table` nests as `items: [ {layoutStyle:"TableColumns", items:[…TableCol]},
{layoutStyle:"TableRows", items:[…TableRow]} ]`, and each `TableRow` carries
`isHeader: boolean` and `items: [ Text ContentBlock per cell ]`.

### `File`
```jsonc
{
  "componentType": "File",
  "name": "x.gif", "type": "Image", "mime": "image/gif", "size": 15192,
  "targetObjectId": "bafy…", "state": "", "style": "Embed",
  "fileUrl": "files/x.gif"
}
```
- `type`: `None | File | Image | Video | Audio | PDF`
- `style`: `Auto | Link | Embed`
- `state`: `Empty | Uploading | Done | Error` (often `""`)

### `Link`
```jsonc
{ "componentType": "Link", "title": "", "description": "", "url": "https://…",
  "targetBlockId": "bafy…", "iconSize": "SizeSmall", "cardStyle": "Card" }
```

### `Bookmark`
```jsonc
{ "componentType": "Bookmark", "url": "https://…", "title": "…", "description": "…",
  "imageHash": "", "faviconHash": "bafy…", "type": "Page", "targetObjectId": "bafy…" }
```

### `Latex`
```jsonc
{ "componentType": "Latex", "text": "", "processor": "Drawio" }
```
- `processor` examples: `Latex`, `Mermaid`, `Drawio`, … For `Drawio`, `text` is
  typically empty (the diagram lives in a companion file).

### `JupyterComponent`
```jsonc
{
  "componentType": "JupyterComponent",
  "text": "/custom_component:jupyter:(data-preparating.ipynb:1)/",
  "style": "Code",
  "marks": [],
  "fileName": "data-preparating.ipynb",
  "cellNumber": 1,
  "cell": Cell            // see §8 — single notebook cell, html outputs sanitized
}
```

### `Unknown` (catch-all)
Emitted for `relation, dataview, featuredRelations, div, icon, tableOfContents,
widget, chat, smartblock, none`:
```jsonc
{ "componentType": "Unknown", "orginal_type": "relation" }  // NOTE: misspelling "orginal_type" is intentional in output
```

### `None`
`{ "componentType": "None" }` — not currently produced.

---

## 7. Nested item structures

### `Mark` (`content_block/mark.rs`)
```jsonc
{ "range": { "from": 0, "to": 25 }, "type": "Bold", "param": null }
```
> ⚠️ In release output `marks` is a **flat `Mark[]`** directly on the text attr
> (not wrapped in `{ "marks": [...] }`). See the blog mismatch in
> [features.md](./features.md#part-3).

### `TextItem` (`content_block/text.rs`)
Internally tagged on `_text_item_type` (`Block | LevelText | Other | Unknown`):
```jsonc
{ "_text_item_type": "LevelText", "order": 21, "text": "…", "style": "Marked", "marks": [] }
```
- `LevelText` / `Block`: a flattened `TextComponentAttr` (has `text/style/marks`,
  optionally nested `items`).
- `Other`: a full `ContentBlock` (has `componentAttr`).

### `LayoutItem` (`content_block/layout.rs`)
An **untagged** union — one of:
- a full `ContentBlock` (leaf: `{ id, order, childrenIds, style, componentAttr }`), or
- a nested `LayoutComponentAttr` (`{ order, style, layoutStyle, items, isHeader? }`
  — note: **no `componentType`** when nested).

Consumers must distinguish by presence of `componentAttr` (leaf) vs `layoutStyle`
(sub-layout).

---

## 8. Jupyter `Cell` model

`src/jupyter_notebook/model.rs` — tagged on `cell_type` (`snake_case`).

```jsonc
// markdown cell
{ "cell_type": "markdown", "metadata": {}, "source": ["# Title\n", …], "attachments": {} }

// code cell
{ "cell_type": "code", "execution_count": 1, "metadata": {}, "source": [...],
  "outputs": [ Output ] }
```
`Output`:
```jsonc
{ "output_type": "execute_result|display_data|stream|error|none",
  "data": { "text/plain": [...], "text/html": [...]?, "image/png": "<base64>"? },
  "execution_count": 1?, "metadata": { "needs_background": "light" }?,
  "name": "stdout"?, "text": [...]?, "ename": "…"?, "evalue": "…"?, "traceback": [...]? }
```
`text/html` outputs are sanitized by `resolve_jupyter_cell_output`.

---

## 9. `PageExternalLink` & `PageMeta`

### `PageExternalLink` (`page/ext.rs`)
| field         | type     | when present                          |
| ------------- | -------- | ------------------------------------- |
| `id`          | string   | always                                |
| `_sid`        | string   | always                                |
| `componentId` | string   | TOC entries only                      |
| `label`       | string   | always                                |
| `url`         | string   | always                                |
| `level`       | number   | TOC entries (heading level 1–4)       |
| `snippet`     | string   | series/tag list entries               |
| `tags`        | `PageMiniExternalLink[]` | series/tag list entries  |
| `serie`       | `PageMiniExternalLink`   | series/tag list entries  |
| `publishDate` | number   | related/series/tag list entries       |

`PageMiniExternalLink` = `{ id, _sid, label, url }`.

### `PageMeta` (`page/meta.rs`)
```jsonc
{ "images": [OpenGraphObj]|null, "videos": [OpenGraphObj]|null, "audio": [OpenGraphObj]|null }
```
`OpenGraphObj` = `{ url, secureUrl?, type?, width?, height?, alt? }`.

---

## 10. Series / Tag index objects

`series/index.json` and `tags/index.json` are arrays of:
```jsonc
{
  "id": "bafy…", "_sid": "rgihc324", "name": "GameLog", "description": "",
  "totalCount": 1, "totalPageNumber": 1, "pageIndex": 0,
  "resultList": [ PageExternalLink ]   // index.json: capped at 3; pN.json: full page (150/200)
}
```
The paged files (`series/<slug>/pN.json`, `tags/<slug>/pN.json`) share the same
shape with the full `resultList` for that page.

---

## Debug-only fields

Present only when **not** built in release mode (absent from committed output):

- `Page`: `rawAttributes`, `rawTagList`, `rawSeries`, `checkBlockOrder`
- `ContentBlock`: `debugType`
- `TextComponentAttr` / `LayoutComponentAttr`: `id`, `childrenIds`

---

## Caveats / gotchas

- The `Unknown` attr key is spelled **`orginal_type`** (sic) in the output.
- `marks` is a flat array, not `{ marks: [...] }` (legacy TS shape).
- `index.json` / `PageIndexReference` is snake_case; everything else is camelCase.
- `LayoutItem` is untagged — disambiguate by `componentAttr` vs `layoutStyle`.
- `coverImage` is **not** emitted (the blog references it — see features.md).
