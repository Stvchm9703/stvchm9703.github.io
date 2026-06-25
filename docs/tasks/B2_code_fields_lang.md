---
id: B2
title: Use `fields.lang` for highlight language in code.svelte
subsystem: blog (src/)
agent: builder
depends_on: []
blocks: []
parallel_wave: 1
gated: false
files:
  modify:
    - src/lib/components/post-content-layout/block/code.svelte
verify:
  - "pnpm check   # (fallback: bunx svelte-check --tsconfig ./tsconfig.json) — run from repo root"
---

# Task B2 — `code.svelte` reads `fields.lang`

## Objective
Code blocks from the Rust converter carry `fields.lang` (e.g. `"cs"`). The blog's
`code.svelte` hardcodes `lang = typescript` and never reads `fields.lang`, so all
code blocks highlight as TypeScript. Map `fields.lang` to a `svelte-highlight`
language with a sensible fallback.

## Context for a cold agent
- `ContentBlock.fields` is an optional object, e.g. `{ "lang": "cs" }` — see
  `docs/data_model.md` §4.
- Gap report: `docs/features.md` Part 3 #2 (severity High).
- The blog already uses `svelte-highlight`. Inspect its supported language imports
  and build a small map from Anytype/common lang codes (e.g. `cs` → csharp,
  `ts`/`typescript`, `js`/`javascript`, `py`/`python`, `rs`/`rust`, `sh`/`bash`,
  `json`, `html`, `css`, etc.) to the highlight language objects.
- Fall back to a reasonable default (e.g. plaintext or the current typescript)
  when `fields.lang` is missing or unmapped — do not throw.

## Target Files
### Modify
- `src/lib/components/post-content-layout/block/code.svelte`

## Required Behaviors
- Read `fields.lang` (case-insensitive) from the block.
- Resolve it to the corresponding `svelte-highlight` language.
- Unknown or missing `lang` falls back gracefully (no runtime error).
- Existing code blocks that previously rendered still render.

## Implementation Constraints
- Only add languages that `svelte-highlight` actually exports; lazy/dynamic import
  if that is the existing pattern in the file.
- Keep the change scoped to language resolution.

## Steps
- [ ] Inspect current `code.svelte` highlight import + hardcoded lang.
- [ ] Add a lang-code → highlight-language map with fallback.
- [ ] Read `fields.lang` and select the language.
- [ ] Run verify command.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes with no new type errors from this file.
- A code block with `fields.lang: "cs"` highlights as C# (or nearest supported);
  a block with no `lang` still renders.

## Verify
```bash
pnpm check
# fallback: bunx svelte-kit sync && bunx svelte-check --tsconfig ./tsconfig.json
```

## Suggested Commit
```bash
git commit -m "fix(blog): map fields.lang to highlight language in code.svelte"
```
