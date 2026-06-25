---
id: X1
title: Blog consumes `coverImage` (OG image)
subsystem: blog (src/)
agent: builder
depends_on: [R1]
blocks: []
parallel_wave: 2
gated: false
files:
  modify:
    - src/routes/posts/**/+page.server.ts   # the OG/cover reference site
    - src/types/page.ts                       # add coverImage to the Page type (matches R1 output)
verify:
  - "pnpm check   # (fallback: bunx svelte-check --tsconfig ./tsconfig.json) — run from repo root"
---

# Task X1 — Blog consumes `coverImage`

## Objective
Once the Rust tool emits `coverImage` (task R1), make the blog consume it: add
`coverImage` to the `src/types/page.ts` `Page` type and ensure the OG/cover
reference in `+page.server.ts` reads the now-present field correctly.

## Context for a cold agent
- **Depends on R1** (Rust tool emitting `coverImage`). Use R1's emitted shape as
  the contract — read the merged R1 task / data_model.md update for the exact
  field (a URL/path string, optional/nullable).
- Gap report: `docs/features.md` Part 3 #4 ("Referenced in `+page.server.ts` (OG
  image) but tool never emits it → always `undefined`").
- `src/types/page.ts` is the new (active after B4) type for `Page`. Add the
  `coverImage` field so the type matches the tool output.
- The blog already references `post.coverImage`; this task makes the type + data
  real, not the reference itself.

## Target Files
### Modify
- `src/routes/posts/**/+page.server.ts` (confirm/adjust the OG image read).
- `src/types/page.ts` (add `coverImage`).

## Required Behaviors
- `Page` type includes `coverImage` matching R1's output (optional/nullable).
- The OG/cover image uses `post.coverImage` when present and degrades gracefully
  when absent.
- `svelte-check` passes (no `coverImage`-related type errors).

## Implementation Constraints
- Match R1's exact field name/shape; do not invent a different one.
- Keep the reference defensive (handle null/undefined).

## Steps
- [ ] Confirm R1 merged and note its `coverImage` shape.
- [ ] Add `coverImage` to `src/types/page.ts`.
- [ ] Verify the OG image read in `+page.server.ts`.
- [ ] Run verify command.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes.
- A post with a cover image exposes it as the OG image; one without does not error.

## Verify
```bash
pnpm check
# fallback: bunx svelte-kit sync && bunx svelte-check --tsconfig ./tsconfig.json
```

## Suggested Commit
```bash
git commit -m "feat(blog): consume coverImage for OG image"
```
