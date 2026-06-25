# Converter & Blog — Feature / Gap Report

Scope of this document:

- **Part 1** — features present in the legacy **`tools/convert_blog_post_ts`**
  (TypeScript / Bun) that are missing or incomplete in the current
  **`tools/convert_blog_post`** (Rust). Use this before retiring the TS tool.
- **Part 3** — output features/changes in **`tools/convert_blog_post`** that the
  **blog (`src/`)** does not yet fully consume.

> Background: `convert_blog_post` (Rust) is the rewrite/successor of
> `convert_blog_post_ts`. Both parse Anytype `.pb` snapshots and emit per-post
> JSON. The Rust tool is the maintained one. The TS tool currently survives only
> because `svelte.config.js` aliases `$generateor` → `tools/convert_blog_post_ts`
> and the blog imports its **types** from there.
>
> Output schema reference: [`data_model.md`](./data_model.md).

---

## Part 1 — Features in `convert_blog_post_ts` not (fully) in `convert_blog_post`

| # | Feature (TS) | Where (TS) | Status in Rust | Severity | Recommendation |
|---|--------------|-----------|----------------|----------|----------------|
| 1 | **`coverImage`** on a page | `page.ts` `Page.coverImage` | ❌ not emitted | High | Port it — the blog already references `post.coverImage` (OG/cover image). Resolve from first image file or a cover relation. |
| 2 | **`description`** field | `page.ts` `Page.description` | ⚠️ commented out (only `snippet`) | Low | Add if the blog needs a separate description; otherwise drop the blog reference. |
| 3 | **Normalized table** (`tableData.rowData[].cells[]` with `content`/`marks`/`isHeader`) | `content_block.ts` `resolveTableComponent` | ⚠️ different shape — Rust emits raw nested `items` (`TableColumns`/`TableRows`/`TableRow`/`Text`) | Info | Representational difference. The blog's `table.svelte` was rewritten for the Rust shape, so no action needed unless you prefer the flat shape. |
| 4 | **`LayoutRow` / `LayoutColumn`** flex grouping for `r-…` column blocks | `content_block.ts` `resolveLayoutRowComponent` | ⚠️ covered differently via `layoutStyle: Row/Column/Div` + `items` | Info | Verify multi-column layouts render; otherwise parity is acceptable. |
| 5 | **`CustomComponent`** (relation + file → custom embed) | `page.ts` `resolveRelationCustomComponent` (commented) + `"CustomComponent"` token | ❌ not implemented | Medium | Not implemented in either tool today; the blog has an (empty) `CustomComponent` branch. Decide whether this feature is still wanted before relying on it. |
| 6 | **User / Workspace resolution** | `user.ts`, `workspace.ts` | ⚠️ ids only (`creator`, `workspaceId`) — not resolved to objects | Low | Port only if you need author name/workspace metadata. |
| 7 | **Dual asset copy** to `static/blog/assets/` (in addition to the export dir) | `index.ts` (`Bun.write("static/blog/assets/…")`) | ⚠️ Rust copies only to `<export>/files/` | Medium | Confirm how assets are served in production. If the blog serves from `static/blog/assets/`, add the second copy (or a build step) in the Rust tool. |
| 8 | Per-page raw snapshot dump (`snapshotAsJson`) | `index.ts` / `page.ts` | ✅ superseded by `is_release` debug fields | — | No action (debug aid only). |

### Parity / Rust-only — removing the TS tool is safe once types are migrated

The Rust tool already covers (or improves on) the rest of the TS pipeline:
collection grouping, tag/series resolution + **paged** series/tags indexes,
related-chapters/related-articles, table-of-contents, grouped Numbered/Marked/
**Toggle** lists (`resolve_nest_children`), Jupyter cell embedding **with
`text/html` output sanitization** (`resolve_jupyter_cell_output`), bookmark/link
resolution, Latex, file/meta OpenGraph extraction, and an optional TypeScript
export (`-t/--ts-export`, via `specta`).

**The only blocking consumer of `convert_blog_post_ts` is the `$generateor`
type alias.** Once the alias is repointed to `src/types` (see Part 2 /
`src/types/index.ts`), `tools/convert_blog_post_ts` can be removed — modulo the
genuine porting gaps above (#1, #5, #7 in particular).

---

## Part 2 — Type definitions (status)

- `src/types/post-content.ts` does **not** match the output (confirmed). It is an
  older hand-drafted shape and is **not imported anywhere**. Marked
  `@deprecated`.
- New, accurate types added: `src/types/content_block.ts`, `src/types/page.ts`,
  `src/types/index.ts` (barrel). They mirror the release-mode JSON described in
  [`data_model.md`](./data_model.md).
- **Activation step (1 line):** repoint `$generateor` in `svelte.config.js`:
  ```js
  $generateor: "src/types",
  "$generateor/*": "src/types/*",
  ```
  Existing imports `$generateor/page` / `$generateor/content_block` then resolve
  to the new files.

---

## Part 3 — `convert_blog_post` output not yet handled by the blog (`src/`)

Confirmed against `src/lib/components/post-content-layout/block/*` and
`src/routes/posts/**`.

| # | Tool output | Blog behavior today | Severity | Fix location | Suggested fix |
|---|-------------|---------------------|----------|--------------|---------------|
| 1 | `marks` is a **flat `Mark[]`** on text attrs | `text.svelte` reads `marks.marks` (legacy nested shape) → `tMarks = []` | **High** | blog | Use `marks` directly (it's already the array). Inline **bold/italic/links are currently dropped** on paragraphs/headers. |
| 2 | Code blocks carry `fields.lang` (e.g. `"cs"`) | `code.svelte` hardcodes `lang = typescript` and never reads `fields.lang` | **High** | blog | Map `fields.lang` → a `svelte-highlight` language; fall back sensibly. |
| 3 | Toggle/list `items` use `_text_item_type` (`LevelText` has no `componentAttr`) | `text.svelte` Toggle renders `items` via `<Block {...item}/>`, which needs `componentAttr` → `LevelText` items render nothing | Medium | blog | Handle `_text_item_type` items (render `LevelText` as text/nested list, `Other` via `<Block>`). |
| 4 | `coverImage` | Referenced in `+page.server.ts` (OG image) but tool never emits it → always `undefined` | Medium | tool **or** blog | Emit `coverImage` from the tool (Part 1 #1) or remove the blog reference. |
| 5 | `Latex` with `processor: "Drawio"` (empty `text`, diagram in companion file) | `latex.svelte` renders from `text` only | Medium | tool + blog | Decide how Drawio/Mermaid diagrams are delivered (inline source vs file) and render accordingly. |
| 6 | `Unknown` blocks (`relation`, `dataview`, `featuredRelations`, `widget`, `chat`) | `layout.svelte` renders nothing | Low | by design | Acceptable, but relation-driven custom content is silently dropped (ties to Part 1 #5). |
| 7 | Stale type imports | `+page.server.ts`, `text.svelte`, `table.svelte`, `code.svelte` import `$generateor/*` (legacy TS types: `publish_date`, `relatedPosts`, `PageLink.title`) | Medium | blog/config | Repoint `$generateor` → `src/types` (Part 2). The runtime already uses the new field names (`publishDate`, `relatedArticles`, `label`); only the *types* are stale. |
| 8 | Asset URLs `/blog/assets/files/…` | Depends on assets existing under `static/blog/assets/` (TS dual-copy) | Medium | tool/build | See Part 1 #7 — ensure the Rust export populates the path the blog serves. |

### Notes
- Items #1, #2, #3, #7 are **blog-side** fixes (the tool output is correct).
- Items #4, #5, #8 need a tool-side (or shared) decision.
- The post **renderer** (`[post_id]_[res]/+page.svelte`), tables, jupyter,
  bookmarks, links, files, and TOC/related-chapters already consume the Rust
  output shape correctly.
