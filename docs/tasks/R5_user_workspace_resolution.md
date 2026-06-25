---
id: R5
title: User / Workspace resolution (optional, Rust tool)
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

# Task R5 — User / Workspace resolution (optional)

## Objective
The Rust tool emits ids only (`creator`, `workspaceId`) and does not resolve them
to objects (author name, workspace metadata). The TS tool had `user.ts` /
`workspace.ts` resolution. Port this **only if** author/workspace metadata is
needed by the blog. This is a LOW-severity, optional enhancement.

## Context for a cold agent
- Gap report: `docs/features.md` Part 1 #6 (severity Low — "Port only if you need
  author name/workspace metadata").
- Current output: `Page.creator` = creator object id, `Page.workspaceId` =
  Anytype `spaceId` (`docs/data_model.md` §3) — neither resolved.
- Legacy reference: TS `user.ts`, `workspace.ts`.
- This task is OPTIONAL and additive. If the blog does not currently need resolved
  author/workspace objects, the agent may record "not needed now" in the task and
  leave ids as-is — but must state that explicitly so it isn't silently skipped.
- If implementing: add resolved fields (e.g. `author: Option<...>`,
  `workspace: Option<...>`) WITHOUT removing the existing id fields, to avoid
  breaking the blog.

## Target Files
### Modify
- `tools/convert_blog_post/src/export_model/page/mod.rs` (additive resolved fields).

## Required Behaviors
- If implemented: resolved author/workspace objects are emitted alongside the
  existing `creator` / `workspaceId` ids, optional/nullable when unresolved.
- Existing id fields remain unchanged.
- If deferred: a clear note in the task explaining it is intentionally not done.

## Implementation Constraints
- Additive only; never remove `creator` / `workspaceId`.
- Resolution must be optional and not fail the export when objects are missing.

## Steps
- [ ] Determine whether the blog needs resolved author/workspace (check
      `+page.server.ts` and post components for `creator`/`workspaceId` usage).
- [ ] If needed: add optional resolved fields + resolver + test.
- [ ] If not needed: record the deferral decision in the task file.
- [ ] `cargo test` (scoped) green if code changed.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `cargo build`/`cargo test` pass (if code changed).
- Either resolved fields are emitted (optional) OR a documented deferral exists.

## Verify
```bash
cargo test --manifest-path tools/convert_blog_post/Cargo.toml
cargo build --manifest-path tools/convert_blog_post/Cargo.toml
```

## Suggested Commit
```bash
git commit -m "feat(convert_blog_post): optional user/workspace resolution"
```
