---
id: R1
title: Emit `coverImage` on Page (Rust tool)
subsystem: convert_blog_post (Rust)
agent: builder
depends_on: []
blocks: [X1, G1]
parallel_wave: 1
gated: false
files:
  modify:
    - tools/convert_blog_post/src/export_model/page/mod.rs
    - tools/convert_blog_post/src/export_model/page/ext.rs   # if a resolver helper fits here
verify:
  - "cargo test --manifest-path tools/convert_blog_post/Cargo.toml"
  - "cargo build --manifest-path tools/convert_blog_post/Cargo.toml"
---

# Task R1 — Emit `coverImage`

## Objective
The blog references `post.coverImage` (OG/cover image, in `+page.server.ts`) but
the Rust converter never emits it, so it is always `undefined`. Add a `coverImage`
field to the `Page` output, resolved from the page's cover relation or the first
image file block.

## Context for a cold agent
- Output schema: `docs/data_model.md` §3 (`Page`). Serialization is `serde`
  `rename_all = "camelCase"`, so a Rust field `cover_image` serializes as
  `coverImage`.
- Gap report: `docs/features.md` Part 1 #1 (severity High) and Part 3 #4.
- Legacy reference shape: TS `page.ts` `Page.coverImage`. Resolve from the Anytype
  cover relation if present; otherwise fall back to the first image file in the
  page (the file/meta extraction path that already feeds `PageMeta.images`).
- Use codegraph to locate the `Page` struct, the file/meta resolution, and where
  `PageMeta.images` is populated (reuse that resolution rather than re-parsing).
- The value should be a URL/path consistent with how other assets are emitted
  (e.g. `files/<name>` or the served `/blog/assets/...` path — coordinate with R3
  conceptually but do NOT depend on R3; emit the same form the blog OG code reads).

## Target Files
### Modify
- `tools/convert_blog_post/src/export_model/page/mod.rs` (add `cover_image` field
  + population)
- Possibly a small resolver in `page/ext.rs` or the existing meta resolver.

## Required Behaviors
- `Page` JSON includes `coverImage` (camelCase) when a cover/first-image exists.
- Field is optional/nullable when no cover image is available (serialize as `null`
  or omit consistently with sibling fields like `serie`).
- Existing fields and tests are unaffected.

## Implementation Constraints
- Reuse existing file/image resolution; do not duplicate parsing logic.
- Match the asset URL convention already used in the output.
- Add/extend a unit or snapshot test asserting `coverImage` is present for a page
  with a cover image and absent/null otherwise.

## Steps
- [ ] codegraph: find `Page` struct + image/meta resolution.
- [ ] Add `cover_image` field (serde camelCase) and populate it.
- [ ] Add a test for present + absent cases.
- [ ] `cargo test` (scoped to convert_blog_post) green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `cargo build` and `cargo test` pass for `tools/convert_blog_post`.
- A page with a cover image emits `coverImage`; one without emits null/omits it.

## Verify
```bash
cargo test --manifest-path tools/convert_blog_post/Cargo.toml
cargo build --manifest-path tools/convert_blog_post/Cargo.toml
```

## Suggested Commit
```bash
git commit -m "feat(convert_blog_post): emit coverImage on Page"
```
