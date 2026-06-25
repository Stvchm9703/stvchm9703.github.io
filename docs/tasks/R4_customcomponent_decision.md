---
id: R4
title: CustomComponent — decision + (optional) implementation (Rust tool)
subsystem: convert_blog_post (Rust)
agent: builder
depends_on: []
blocks: [G1]
parallel_wave: 1
gated: false
files:
  create:
    - docs/decisions/customcomponent.md   # decision record (ADR-style)
  modify:
    - tools/convert_blog_post/src/export_model/content_block/   # only if decision = implement
verify:
  - "cargo test --manifest-path tools/convert_blog_post/Cargo.toml   # if any code change"
---

# Task R4 — CustomComponent decision (and optional implementation)

## Objective
`CustomComponent` (relation + file → custom embed) is implemented in NEITHER tool
today; the blog has an empty `CustomComponent` branch. Decide whether this feature
is still wanted before anything relies on it, record the decision, and implement
only if the decision is "yes". This decision is a precondition for safely deleting
the TS tool (G1) because it determines whether a Part-1 gap is being intentionally
dropped vs ported.

## Context for a cold agent
- Gap report: `docs/features.md` Part 1 #5 (Medium) and Part 3 #6 (relation-driven
  custom content is silently dropped via `Unknown` blocks).
- Legacy TS: `page.ts` `resolveRelationCustomComponent` (commented) + a
  `"CustomComponent"` token. Use it as the reference shape if implementing.
- Rust today emits `Unknown { orginal_type: "relation" }` for relation blocks
  (`docs/data_model.md` §6 `Unknown`).
- **This is primarily a decision task.** Produce a short ADR:
  - Option A: drop the feature (document that relation-driven embeds are
    intentionally unsupported; the blog's empty branch stays a no-op).
  - Option B: implement it in the Rust tool (define the `CustomComponent`
    `ComponentAttr` variant + resolution, mirroring the TS token), then a follow-up
    blog task would render it (out of scope here — note it for the next wave).
- If Option B, scope the Rust change to adding the variant + resolver; coordinate
  the output shape with `docs/data_model.md` and add tests.

## Target Files
### Create
- `docs/decisions/customcomponent.md` (ADR: chosen option + rationale).
### Modify (only if Option B chosen)
- `tools/convert_blog_post/src/export_model/content_block/` (new variant + resolver).

## Required Behaviors
- A written decision exists and is unambiguous (drop vs implement).
- If implemented: a `CustomComponent` attr is emitted for relation+file blocks and
  covered by a test; `docs/data_model.md` is updated to describe it.
- If dropped: ADR states relation-driven embeds are unsupported; G1 can proceed
  knowing this Part-1 gap is intentionally not ported.

## Implementation Constraints
- Do not implement speculatively without recording the decision first.
- If implementing, follow the internally-tagged `componentType` union pattern.

## Steps
- [ ] Review TS `resolveRelationCustomComponent` + blog empty branch.
- [ ] Write the ADR (drop or implement + rationale).
- [ ] If implement: add variant + resolver + test; update data_model.md.
- [ ] `cargo test` (scoped) green if code changed.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `docs/decisions/customcomponent.md` exists with a clear decision.
- If Option B: `cargo build`/`cargo test` pass and the variant is emitted/tested.

## Verify
```bash
# only if code changed:
cargo test --manifest-path tools/convert_blog_post/Cargo.toml
```

## Suggested Commit
```bash
git commit -m "docs(convert_blog_post): decision record for CustomComponent (+impl if chosen)"
```
