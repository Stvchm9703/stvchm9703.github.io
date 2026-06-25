---
id: R2
title: Emit optional `description` on Page (Rust tool)
subsystem: convert_blog_post (Rust)
agent: builder
depends_on: []
blocks: []
parallel_wave: 1
gated: false
files:
  modify:
    - tools/convert_blog_post/src/export_model/page/mod.rs
verify:
  - "cargo test --manifest-path tools/convert_blog_post/Cargo.toml"
  - "cargo build --manifest-path tools/convert_blog_post/Cargo.toml"
---

# Task R2 — Emit optional `description`

## Objective
The TS tool emitted `Page.description`; the Rust tool currently only emits
`snippet` (the `description` field is commented out). Add an optional
`description` field to `Page` so the blog can use a separate description distinct
from the plain-text `snippet` excerpt.

## Context for a cold agent
- Gap report: `docs/features.md` Part 1 #2 (severity Low — "Add if the blog needs
  a separate description; otherwise drop the blog reference").
- Output schema: `docs/data_model.md` §3 (`Page`) — `snippet` exists; `description`
  does not. serde `rename_all = "camelCase"`.
- Resolve `description` from the Anytype `description` relation (the commented-out
  path). Keep it **optional** (Rust `Option<String>`), serialized as `null`/omitted
  when absent so it never breaks pages that lack a description.
- This is a LOW-severity, isolated additive field. It touches the same struct as
  R1 (`page/mod.rs`) but a different field — coordinate at merge time but it has no
  logical dependency on R1. If executed in the same wave by separate agents,
  expect a trivial merge in `page/mod.rs`; that is acceptable.

## Target Files
### Modify
- `tools/convert_blog_post/src/export_model/page/mod.rs`

## Required Behaviors
- `Page` JSON includes optional `description` (camelCase) sourced from the
  Anytype description relation.
- Absent description serializes consistently (null or omitted) without error.
- `snippet` behavior is unchanged.

## Implementation Constraints
- Additive only; do not alter `snippet`.
- Optional type; safe default when the relation is missing.
- Add a small test asserting presence when the relation is set.

## Steps
- [ ] codegraph: find the commented-out `description` resolution in `page/mod.rs`.
- [ ] Add `description: Option<String>` (serde camelCase) and populate it.
- [ ] Add a test for present + absent.
- [ ] `cargo test` (scoped) green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `cargo build`/`cargo test` pass for `tools/convert_blog_post`.
- Pages with a description relation emit `description`; others emit null/omit.

## Verify
```bash
cargo test --manifest-path tools/convert_blog_post/Cargo.toml
cargo build --manifest-path tools/convert_blog_post/Cargo.toml
```

## Suggested Commit
```bash
git commit -m "feat(convert_blog_post): emit optional description on Page"
```
