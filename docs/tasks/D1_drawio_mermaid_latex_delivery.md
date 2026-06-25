---
id: D1
title: Decide Drawio/Mermaid Latex diagram delivery + rendering
subsystem: convert_blog_post (Rust) + blog (src/) — decision/spike
agent: builder
depends_on: []
blocks: []
parallel_wave: 1
gated: false
files:
  create:
    - docs/decisions/drawio-mermaid-latex.md
  modify:
    # only if the decision is to implement a small fix now; otherwise spin off follow-up tasks
    - tools/convert_blog_post/src/export_model/content_block/latex.rs
    - src/lib/components/post-content-layout/block/latex.svelte
verify:
  - "cargo build --manifest-path tools/convert_blog_post/Cargo.toml   # only if Rust touched"
  - "pnpm check   # only if blog touched"
---

# Task D1 — Drawio / Mermaid `Latex` delivery decision

## Objective
`Latex` content blocks carry a `processor` (e.g. `Latex`, `Mermaid`, `Drawio`).
For `Drawio` (and often `Mermaid`) the `text` field is **empty** — the diagram
lives in a companion file, so the blog cannot render anything from `text` alone.
Produce a written decision on how these diagrams are delivered by the tool and
rendered by the blog, and either implement a small fix or spin off follow-up
tasks.

## Context for a cold agent
- Output schema: `docs/data_model.md` §6 `Latex` variant
  (`{ componentType: "Latex", text, processor }`).
- Gap report: `docs/features.md` Part 3 #5 (severity Medium) — open tool+blog
  decision, no committed direction yet.
- Rust side: `tools/convert_blog_post/src/export_model/content_block/latex.rs`
  (`LatexComponentAttr`). The companion diagram, if any, is among the copied
  `files/` assets.
- Blog side: `src/lib/components/post-content-layout/block/latex.svelte` renders
  from `text` today.
- Use codegraph to find how the Latex block is built and whether any file/asset
  reference is associated with it.

## Required outcome
1. A decision doc at `docs/decisions/drawio-mermaid-latex.md` covering:
   - How Drawio/Mermaid sources are delivered (inline source in `text`, a file
     reference/URL on the block, or rendered-to-SVG/PNG at conversion time).
   - How the blog renders each `processor` value.
   - Effort/owner split (tool vs blog) and any new follow-up task ids.
2. If a low-risk fix is obvious (e.g. populate a `fileUrl`/source reference on the
   Latex block + render it), implement it and update the relevant verify commands.
   Otherwise, create the follow-up task docs under `docs/tasks/` and link them.

## Implementation Constraints
- This is decision-first. Do not over-build; prefer a documented decision + scoped
  follow-ups over a large speculative implementation.
- If you touch Rust or blog code, keep it minimal and run the matching verify.

## Steps
- [ ] codegraph: inspect Latex block construction (Rust) + `latex.svelte` (blog).
- [ ] Inspect a real `Drawio`/`Mermaid` block in `blog_post_resolved_0624/` and its
      companion asset (if any) in `files/`.
- [ ] Write `docs/decisions/drawio-mermaid-latex.md`.
- [ ] Implement the small fix OR create follow-up task docs.
- [ ] Update TASKS.md via the task-management skill.

## Acceptance Criteria
- A committed decision doc that unambiguously states delivery + rendering approach.
- Either a working minimal change (verify green) or linked follow-up task(s).

## Verify
```bash
# only the relevant ones, depending on what (if anything) you changed:
cargo build --manifest-path tools/convert_blog_post/Cargo.toml
pnpm check
```

## Suggested Commit
```bash
git commit -m "docs(decision): drawio/mermaid latex diagram delivery"
```
