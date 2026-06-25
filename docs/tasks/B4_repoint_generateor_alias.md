---
id: B4
title: Repoint `$generateor` alias to `src/types`
subsystem: blog (config)
agent: builder
depends_on: []
blocks: [G1]
parallel_wave: 1
gated: false
files:
  modify:
    - svelte.config.js
verify:
  - "pnpm check   # (fallback: bunx svelte-check --tsconfig ./tsconfig.json) — run from repo root"
---

# Task B4 — Repoint `$generateor` → `src/types`

## Objective
Activate the new, accurate types by repointing the `$generateor` SvelteKit alias
from the legacy TS tool to `src/types`. The blog runtime already uses the new
field names (`publishDate`, `relatedArticles`, `label`); only the *types* are
stale. This is the single activation step for `src/types/{content_block,page,index}.ts`.

## Context for a cold agent
- Background: `docs/features.md` Part 2 ("Activation step (1 line)") and Part 3 #7.
- Current `svelte.config.js` `kit.alias` block contains:
  ```js
  $generateor: "tools/convert_blog_post_ts",
  "$generateor/*": "tools/convert_blog_post_ts/*",
  ```
  Change to:
  ```js
  $generateor: "src/types",
  "$generateor/*": "src/types/*",
  ```
- `src/types/` already contains `content_block.ts`, `page.ts`, `index.ts` (barrel),
  and a `@deprecated` `post-content.ts`. Do NOT create or modify type files here.
- Importers `$generateor/page` / `$generateor/content_block` will then resolve to
  the new files (`+page.server.ts`, `text.svelte`, `table.svelte`, `code.svelte`).
- **This task unblocks the gated TS-tool deletion (G1).** It is independent at the
  file level (only `svelte.config.js`) so it runs in wave 1 alongside B1/B2.

## Target Files
### Modify
- `svelte.config.js`

## Required Behaviors
- `$generateor` and `$generateor/*` resolve to `src/types` and `src/types/*`.
- The blog type-checks against the new types (no `publish_date` / `relatedPosts` /
  `PageLink.title` legacy-type errors).

## Implementation Constraints
- Change ONLY the two `$generateor` alias lines. Leave all other aliases intact.
- Do not delete `tools/convert_blog_post_ts/` in this task — deletion is gated (G1).

## Steps
- [ ] Edit the two `$generateor` alias lines in `svelte.config.js`.
- [ ] Run `pnpm check` and confirm legacy-type errors are gone.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes (no new errors introduced by the repoint; ideally fewer
  errors than before since stale types are replaced).
- Imports from `$generateor/*` resolve to `src/types/*`.

## Verify
```bash
pnpm check
# fallback: bunx svelte-kit sync && bunx svelte-check --tsconfig ./tsconfig.json
```

## Suggested Commit
```bash
git commit -m "chore(blog): repoint \$generateor alias to src/types"
```
