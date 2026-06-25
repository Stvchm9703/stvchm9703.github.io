---
id: X2
title: Verify blog serves assets from the dual-copied path
subsystem: blog (build/assets)
agent: builder
depends_on: [R3]
blocks: []
parallel_wave: 2
gated: false
files:
  modify:
    - src/   # only if an asset URL base needs adjusting; otherwise verification-only
verify:
  - "pnpm check   # (fallback: bunx svelte-check) — plus a manual asset-path check (below)"
---

# Task X2 — Verify asset serving from `static/blog/assets/`

## Objective
After R3 makes the Rust tool dual-copy assets to `static/blog/assets/files/`,
verify the blog actually serves asset URLs (`/blog/assets/files/…`) from that
path, and adjust the blog's asset URL base only if needed.

## Context for a cold agent
- **Depends on R3** (dual asset copy in the Rust tool).
- Gap report: `docs/features.md` Part 3 #8 + Part 1 #7.
- Blog asset URL convention: `/blog/assets/files/<name>` must resolve to
  `static/blog/assets/files/<name>` (SvelteKit serves `static/` at the site root,
  modulo `paths.base`).
- This is primarily a **verification** task: confirm the path the blog requests
  matches where R3 writes. Only modify the blog if the base path is wrong.
- Note: `svelte.config.js` uses `paths.base = process.env.BASE_PATH` in non-dev —
  ensure asset URLs account for `base` consistently with the rest of the blog.

## Target Files
### Modify (only if a mismatch is found)
- `src/` asset URL construction (locate where `/blog/assets/files/...` URLs are built).

## Required Behaviors
- Asset URLs the blog emits resolve to the files R3 writes under
  `static/blog/assets/files/`.
- If a base-path or prefix mismatch exists, it is corrected; otherwise the task
  records that serving already works.

## Implementation Constraints
- Verification-first; change blog code only on a confirmed mismatch.
- Respect `paths.base`.

## Steps
- [ ] Confirm R3 merged and note its output path.
- [ ] Trace where the blog builds `/blog/assets/files/...` URLs.
- [ ] Confirm they resolve to R3's destination (account for `base`).
- [ ] Fix only if mismatched; otherwise record "serving verified".
- [ ] Run verify command + manual path check.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes.
- Documented confirmation that an exported asset is reachable at the blog's
  asset URL (or a fix making it so).

## Verify
```bash
pnpm check
# manual: after an export, confirm a file exists at
#   static/blog/assets/files/<name>
# and that the blog requests /blog/assets/files/<name> (accounting for BASE_PATH).
```

## Suggested Commit
```bash
git commit -m "chore(blog): verify/fix asset serving from static/blog/assets"
```
