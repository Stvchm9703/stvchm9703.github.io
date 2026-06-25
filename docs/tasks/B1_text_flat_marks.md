---
id: B1
title: Fix text.svelte to read flat `marks` array (not `marks.marks`)
subsystem: blog (src/)
agent: builder
depends_on: []
blocks: [B3]
parallel_wave: 1
gated: false
files:
  modify:
    - src/lib/components/post-content-layout/block/text.svelte
verify:
  - "pnpm check   # (fallback: bunx svelte-check --tsconfig ./tsconfig.json) — run from repo root"
---

# Task B1 — Fix `text.svelte` flat `marks`

## Objective
The Rust converter emits `marks` as a **flat `Mark[]`** directly on the text
component attr (see `docs/data_model.md` §7). The blog's `text.svelte` currently
reads `marks.marks` (the legacy nested TS shape), so `tMarks` resolves to `[]`
and **inline bold/italic/link formatting is silently dropped** on paragraphs and
headers. Fix the component to consume the flat array.

## Context for a cold agent
- Output schema source of truth: `docs/data_model.md` (§6 `Text` variant, §7 `Mark`).
- `Mark` shape: `{ "range": { "from": 0, "to": 25 }, "type": "Bold", "param": null }`.
  `type` is one of `MarkType`: `Strikethrough, Keyboard, Italic, Bold, Underscored,
  Link, TextColor, BackgroundColor, Mention, Emoji, Object`.
- In release output there is **no** `{ marks: [...] }` wrapper — `attr.marks` is
  already the array.
- Gap report: `docs/features.md` Part 3 #1 (severity High).
- Do NOT touch the Toggle / `LevelText` item rendering in this file — that is a
  separate task (B3) which depends on this one. Keep your change scoped to how
  `marks` is read for paragraph/header text.

## Target Files
### Modify
- `src/lib/components/post-content-layout/block/text.svelte`

## Required Behaviors
- Read marks from the flat `marks` array on the text attr (e.g. `attr.marks`),
  not `attr.marks.marks`.
- Bold, Italic, and Link marks render on paragraph and header text.
- Empty / missing `marks` ([] or undefined) renders plain text without error.
- Mark `range.from` / `range.to` continue to map onto the correct substring.

## Implementation Constraints
- Scope strictly to the marks-read path. Do not refactor unrelated rendering.
- Follow the existing component's TypeScript types from `$generateor/*` (note: the
  alias repoint is task B4, run in the same wave on a different file — do not block
  on it; the runtime field names already match).

## Steps
- [ ] Locate where `text.svelte` derives `tMarks` from the attr.
- [ ] Replace nested `marks.marks` access with the flat `marks` array.
- [ ] Verify bold/italic/link render; verify empty marks is safe.
- [ ] Run verify command.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes with no new type errors from this file.
- A paragraph containing a Bold/Italic/Link mark renders the formatting (manual
  or snapshot check against a `blog_post_resolved_*` page).

## Verify
```bash
# from repo root. pnpm-lock.yaml is the newer lockfile; bun is the fallback.
pnpm check
# fallback if pnpm unavailable:
# bunx svelte-kit sync && bunx svelte-check --tsconfig ./tsconfig.json
```

## Suggested Commit
```bash
git commit -m "fix(blog): read flat marks array in text.svelte"
```
