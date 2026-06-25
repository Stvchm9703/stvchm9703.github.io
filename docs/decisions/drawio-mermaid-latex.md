---
id: D1
title: Drawio / Mermaid Latex diagram delivery and rendering
status: decided
date: 2026-06-24
relates_to: docs/features.md Part 3 #5
---

# Decision: Drawio / Mermaid `Latex` Block Delivery and Rendering

## Context

The Anytype `Block.Content.Latex` type is a **multi-purpose embed block** that
carries a `processor` enum and a `text` field. It is used for many things
beyond actual LaTeX math: Mermaid diagrams, Graphviz, YouTube embeds, Google
Maps, GitHub Gist, and Draw.io (Drawio) diagrams, among others (25 processor
variants in the proto).

The Rust tool (`tools/convert_blog_post`) converts these into
`{ componentType: "Latex", text: "...", processor: "Drawio" }` objects. The
blog renders them in `latex.svelte`.

### Current processor coverage in `latex.svelte`

| Processor    | Blog handles? | Mechanism                                 |
|-------------|---------------|-------------------------------------------|
| `Latex`      | No branch     | Renders nothing (no `{#if}` branch exists) |
| `Mermaid`    | Yes           | Client-side render via `mermaid` npm pkg  |
| `Graphviz`   | Yes           | Client-side render via `@viz-js/viz`      |
| `Youtube`    | Yes           | `<iframe>` embed                          |
| `GoogleMaps` | Yes           | `<iframe>` embed, URL transformed         |
| `GithubGist` | No branch     | Renders nothing                           |
| `Drawio`     | No branch     | Renders nothing (text is always empty)    |
| All others   | No branch     | Render nothing silently                   |

### The Drawio problem (core gap)

Confirmed with real data from `blog_post_resolved_0624/`:

- The Drawio block (block id `68afc826e212b567bdd1cf55`) has `text: ""` and
  `processor: "Drawio"`.
- There is **no companion file** in `files/` linked to this block — no `.drawio`,
  `.xml`, or exported `.svg`/`.png` is associated with it at the JSON level.
- The Rust tool copies the block `text` field verbatim from the proto (which is
  empty for Drawio). No file reference or `fileUrl` field is added to the
  `LatexComponentAttr`.
- Anytype stores Draw.io content as XML inside `text`; when `text` is empty it
  likely means the diagram was not fully serialized into the export snapshot, or
  it was imported as a linked file rather than embedded content.
- The blog has no `{#if processor === "Drawio"}` branch in `latex.svelte`, so
  the block silently renders nothing.

### Mermaid — current status

- `text` is populated with the Mermaid diagram source (confirmed: full
  `flowchart TD ...` syntax present in the resolved JSON).
- The blog renders Mermaid client-side using the `mermaid` npm library.
- This works today as long as the `mermaid` package is available.
- Risk: SSR / prerender environments cannot run `mermaid.render()` — the
  `onMount` guard handles this, but the diagram is invisible if JS is disabled.

### True `Latex` (math) — current status

- `processor: "Latex"` carries TeX math source in `text`.
- The blog has **no `{#if}` branch** for `processor === "Latex"`. The block
  renders nothing. This is a silent gap.

### Other unhandled processors observed in data

- `GithubGist` — `text` contains a gist URL; the blog does nothing with it.
- (Potential) `Miro`, `Figma`, `Excalidraw`, `Kroki`, etc. — not seen in the
  snapshot data but exist in the proto enum.

---

## Decision

### Per-processor delivery and rendering approach

#### Processors where `text` holds all the content

These processors embed their full source or URL in `text`. **No tool-side
change is needed.** The tool correctly passes `text` through today.

| Processor    | `text` holds       | Recommended blog rendering                         |
|-------------|--------------------|----------------------------------------------------|
| `Latex`      | TeX math source    | Client-side: KaTeX or MathJax in a `<div>`         |
| `Mermaid`    | Diagram source     | Client-side: `mermaid.render()` (already done)     |
| `Graphviz`   | DOT-lang source    | Client-side: `@viz-js/viz` (already done)          |
| `Youtube`    | Watch URL          | `<iframe>` embed (already done)                    |
| `GoogleMaps` | Maps URL           | `<iframe>` embed, URL transformed (already done)   |
| `GithubGist` | Gist URL           | `<script>` embed or `<iframe>` using the gist JS   |
| `Miro`       | Board URL          | `<iframe>` embed                                   |
| `Figma`      | Figma file URL     | `<iframe>` embed via Figma embed API               |
| `Excalidraw` | Scene JSON         | Client-side via Excalidraw React component         |
| `Kroki`      | Diagram source     | Server-side: Kroki HTTP API or client-side script  |
| `Spotify`    | Spotify URL        | `<iframe>` embed                                   |
| `Vimeo`      | Video URL          | `<iframe>` embed                                   |
| `Soundcloud` | Track URL          | `<iframe>` embed                                   |
| `Bilibili`   | Video URL          | `<iframe>` embed                                   |
| `Twitter`    | Tweet URL          | `<blockquote>` + Twitter widgets.js script         |
| `Facebook`   | Post URL           | `<div class="fb-post">` + FB SDK                  |
| `Instagram`  | Post URL           | `<blockquote>` + Instagram embed.js               |
| `Telegram`   | Post URL           | `<script>` telegram embed                         |
| `Reddit`     | Post URL           | `<blockquote>` + Reddit embed.js                  |
| `OpenStreetMap` | Map URL         | `<iframe>` embed                                   |
| `Codepen`    | Pen URL            | `<iframe>` embed                                   |
| `Sketchfab`  | Model URL          | `<iframe>` embed                                   |

#### Drawio — special case (text is empty, no file reference)

Drawio is the only processor where `text` is consistently empty in the
exported snapshots. Two root causes are possible:

1. Anytype exports the diagram XML into a separate binary file object (a
   sibling block in the Anytype object tree that is a `File` block), not
   inside the `Latex.text` field.
2. The diagram was created via the Drawio desktop app and linked by reference
   rather than embedded.

**There is no `files/` asset associated with the Drawio block** in the
current export (`blog_post_resolved_0624`). The `files/` directory has no
`.drawio`, `.xml`, or obviously Drawio-related file.

**Tool-side investigation required before the blog can render Drawio.** The
Rust tool must determine whether:
- A sibling `File` block in the Anytype block tree carries the `.drawio` XML
  or a pre-rendered image, and if so, associate the `fileUrl` with the
  `LatexComponentAttr`.
- Or whether Anytype genuinely does not export Drawio content in the `.pb`
  snapshot (in which case the block is unrenderable from the export).

Until that investigation, the correct blog behavior for `Drawio` is to render
a **graceful placeholder** (e.g. "Diagram unavailable") rather than silence.

#### Image processor

`processor: "Image"` presumably carries an image URL or base64 in `text`. Not
seen in the current data; treat like `File` (render `<img>`).

---

## Tool vs Blog split

### Tool responsibilities (Rust `convert_blog_post`)

1. **No change needed for text-carrying processors** — the current
   `LatexComponentAttr { text, processor }` shape is correct and sufficient.
2. **Drawio investigation** (follow-up task D1-T1): Inspect the raw Anytype
   `.pb` block tree for a Drawio block to determine whether a sibling `File`
   block carries the diagram content. If found, either:
   - Add an optional `fileUrl: Option<String>` field to `LatexComponentAttr`
     and populate it from the sibling file block during resolution, or
   - Pre-render the Drawio XML to SVG using a conversion library/CLI and emit
     a `fileUrl` pointing to the rendered SVG in `files/`.
3. **No schema change otherwise** — the current `{ text, processor }` shape is
   the correct contract for all other processors.

### Blog responsibilities (SvelteKit `src/`)

1. **Add `Latex` (math) branch** (follow-up task D1-T2): Render `text` via
   KaTeX or MathJax when `processor === "Latex"`.
2. **Add `GithubGist` branch** (follow-up task D1-T2): Embed the gist using a
   `<script>` tag pointing to `text + ".js"`.
3. **Add `Drawio` placeholder branch** (follow-up task D1-T2): Show a
   "Diagram unavailable" notice until the tool-side is resolved. When the tool
   emits a `fileUrl`, switch to `<img src={componentAttr.fileUrl}>` or an
   inline SVG.
4. **Add `{:else}` fallback** in `latex.svelte` for unknown processors: log
   a warning or show a minimal placeholder so that new processor types don't
   silently vanish.

---

## Proposed follow-up tasks

### D1-T1: Rust tool — investigate Drawio companion file linkage

**Scope**: Tool-only (`tools/convert_blog_post`).

Inspect the raw `.pb` block tree for a page that contains a `Drawio` block.
Determine whether:
- A sibling or child `File` block is linked to the Drawio block (check
  `childrenIds` / parent block `childrenIds`).
- The Anytype `.pb` snapshot for this page exports a `.drawio` file object.

If a linkage is found, add `pub file_url: Option<String>` to
`LatexComponentAttr` and populate it in `from_block_content` (may need
access to the file object map, similar to how `FileComponentAttr` resolves
`file_url`). Emit the companion file to `files/` during asset copy.

If no linkage exists in the proto export, document that Drawio content is
unrecoverable from a `.pb` snapshot and close this path.

Verify: `cargo build --manifest-path tools/convert_blog_post/Cargo.toml`

### D1-T2: Blog — render Latex math, GithubGist, Drawio placeholder, and fallback

**Scope**: Blog-only (`src/lib/components/post-content-layout/block/latex.svelte`).

1. Add `processor === "Latex"` branch: render `text` with KaTeX (preferred for
   SSR compatibility) or MathJax. Install `katex` and add a `<div>` branch.
2. Add `processor === "GithubGist"` branch: inject a `<script
   src="{text}.js">` into the DOM on mount, with a `<div>` placeholder.
3. Add `processor === "Drawio"` branch:
   - If `componentAttr.fileUrl` is present (future, after D1-T1): render
     `<img src={componentAttr.fileUrl}>` or inline the SVG.
   - Otherwise: render a styled `<figure>` with a "Diagram not available"
     notice.
4. Add `{:else}` fallback branch to log and show placeholder for unknown
   processors.

Verify: `pnpm check`

---

## What is NOT recommended

- Pre-rendering all Mermaid/Graphviz diagrams to SVG at conversion time in
  the Rust tool. The existing client-side rendering works and avoids a headless
  browser dependency in the tool.
- Adding `fileUrl` to `LatexComponentAttr` speculatively before confirming
  the Drawio linkage in the proto.
- Changing the `{ componentType: "Latex", text, processor }` schema — it is
  correct for all text-carrying processors. Only Drawio may need an additive
  `fileUrl` field.

---

## Summary

The current `LatexComponentAttr` schema and tool conversion are correct for
all processors that embed their source or URL in `text` (Mermaid, Graphviz,
YouTube, GoogleMaps, etc.). The only structural gap is `Drawio`, where `text`
is empty and no companion file linkage is established by the tool. The blog
additionally has missing branches for `Latex` (math), `GithubGist`, `Drawio`,
and has no fallback for unknown processors — these are all silent no-ops today.
No tool schema change is needed until D1-T1 confirms whether Drawio content is
recoverable from the `.pb` snapshot. The two follow-up tasks (D1-T1: tool
investigation; D1-T2: blog rendering) are independent and can be worked in
parallel.
