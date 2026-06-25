---
id: B3
title: Render Toggle/list `LevelText` items in text.svelte
subsystem: blog (src/)
agent: builder
depends_on: [B1]
blocks: []
parallel_wave: 2
gated: false
files:
  modify:
    - src/lib/components/post-content-layout/block/text.svelte
verify:
  - "pnpm check   # (fallback: bunx svelte-check --tsconfig ./tsconfig.json) — run from repo root"
---

# Task B3 — Toggle/list `LevelText` item rendering

## Objective
Toggle and grouped list `items` use `_text_item_type`. `LevelText` items have NO
`componentAttr`, but `text.svelte` renders Toggle `items` via `<Block {...item}/>`
which requires `componentAttr` — so `LevelText` items render nothing. Handle the
`_text_item_type` discriminator so `LevelText` (and `Block`/`Other`) items render.

## Context for a cold agent
- `TextItem` is internally tagged on `_text_item_type` (`Block | LevelText | Other
  | Unknown`) — see `docs/data_model.md` §7.
  - `LevelText` / `Block`: a flattened `TextComponentAttr` (`text`, `style`,
    `marks`, optionally nested `items`) — render as text / nested list.
  - `Other`: a full `ContentBlock` (has `componentAttr`) — render via `<Block>`.
- Gap report: `docs/features.md` Part 3 #3 (severity Medium).
- **Dependency:** this task shares `text.svelte` with B1 (flat marks). B1 MUST be
  merged first. When rendering `LevelText`/`Block` text, reuse the flat-`marks`
  reading fixed in B1 — do not reintroduce `marks.marks`.

## Target Files
### Modify
- `src/lib/components/post-content-layout/block/text.svelte`

## Required Behaviors
- Branch on `item._text_item_type`:
  - `LevelText` / `Block` → render the text (with flat marks) and any nested
    `items` (nested list).
  - `Other` → render via `<Block {...item}/>` (it has `componentAttr`).
  - `Unknown` → render nothing / safe no-op.
- Toggle blocks display their `LevelText` children instead of empty content.
- No regression to paragraph/header text or the B1 marks fix.

## Implementation Constraints
- Keep B1's flat-marks reading intact and reuse it for item text.
- Scope to the item-rendering path inside `text.svelte`.

## Steps
- [ ] Confirm B1 is merged (flat marks in place).
- [ ] Add `_text_item_type` dispatch for `items`.
- [ ] Render LevelText/Block (text + nested items) and Other (via Block).
- [ ] Run verify command.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes with no new type errors from this file.
- A Toggle/Numbered/Marked list with `LevelText` items renders visible item text.

## Verify
```bash
pnpm check
# fallback: bunx svelte-kit sync && bunx svelte-check --tsconfig ./tsconfig.json
```

## Suggested Commit
```bash
git commit -m "fix(blog): render LevelText toggle/list items in text.svelte"
```
