---
id: R3
title: Dual asset copy to `static/blog/assets/` (Rust tool)
subsystem: convert_blog_post (Rust)
agent: builder
depends_on: []
blocks: [X2, G1]
parallel_wave: 1
gated: false
files:
  modify:
    - tools/convert_blog_post/src/   # asset/file copy path (locate via codegraph: where files are copied to <export>/files/)
verify:
  - "cargo test --manifest-path tools/convert_blog_post/Cargo.toml"
  - "cargo build --manifest-path tools/convert_blog_post/Cargo.toml"
---

# Task R3 — Dual asset copy to `static/blog/assets/`

## Objective
The TS tool wrote assets to BOTH the export dir and `static/blog/assets/`
(`Bun.write("static/blog/assets/…")`). The Rust tool copies only to
`<export>/files/`. The blog serves asset URLs from `/blog/assets/files/…`
(see `docs/features.md` Part 3 #8). Add the second copy (or an equivalent build
step) so the path the blog serves is populated.

## Context for a cold agent
- Gap report: `docs/features.md` Part 1 #7 (Medium) and Part 3 #8 (Medium).
- Blog asset URL convention: `/blog/assets/files/<name>` ⇒ files must exist under
  `static/blog/assets/files/<name>`.
- Current Rust behavior: copies into `<export>/files/` only — locate this with
  codegraph (search for the file-copy routine and `files/` path construction).
- Make the destination configurable (a CLI flag or sensible default like
  `static/blog/assets/`) rather than hardcoding an absolute path. Prefer a flag
  with a default so the tool stays portable.
- This is independent of R1/R2 (different code path: asset copy vs Page model).

## Target Files
### Modify
- The Rust asset-copy module (locate via codegraph; likely under
  `tools/convert_blog_post/src/` near the existing `<export>/files/` copy).

## Required Behaviors
- After (or alongside) copying to `<export>/files/`, also copy each asset to
  `static/blog/assets/files/<name>` (path configurable, with that default).
- Behavior is idempotent and does not fail if the target directory must be created.
- Existing `<export>/files/` copy is unchanged.

## Implementation Constraints
- Do not hardcode a machine-specific absolute path; use a relative default or a
  CLI option (document it).
- Reuse the existing copy helper; add the second destination, do not fork logic.
- Add a test (or integration check) that both destinations receive the asset.

## Steps
- [ ] codegraph: find the asset/file copy routine + `files/` path build.
- [ ] Add the second destination (configurable, default `static/blog/assets`).
- [ ] Ensure target dirs are created; make it idempotent.
- [ ] Add a test asserting dual copy.
- [ ] `cargo test` (scoped) green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `cargo build`/`cargo test` pass for `tools/convert_blog_post`.
- Running an export populates both `<export>/files/` and the
  `static/blog/assets/files/` path.

## Verify
```bash
cargo test --manifest-path tools/convert_blog_post/Cargo.toml
cargo build --manifest-path tools/convert_blog_post/Cargo.toml
```

## Suggested Commit
```bash
git commit -m "feat(convert_blog_post): dual-copy assets to static/blog/assets"
```
