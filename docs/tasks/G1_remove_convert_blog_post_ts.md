---
id: G1
title: "[GATED] Remove legacy tools/convert_blog_post_ts"
subsystem: repo (destructive)
agent: builder
depends_on: [B4, R1, R3, R4]
blocks: []
parallel_wave: null   # NOT auto-executable — gated; runs only after explicit human confirmation
gated: true
gate_reason: "DESTRUCTIVE deletion of the legacy TS converter. Requires explicit human sign-off."
files:
  delete:
    - tools/convert_blog_post_ts/
verify:
  - "pnpm check   # blog still type-checks with \$generateor -> src/types"
  - "cargo build --manifest-path tools/convert_blog_post/Cargo.toml"
---

# Task G1 — [GATED] Remove `tools/convert_blog_post_ts`

> ⚠️ **GATED / DESTRUCTIVE.** Do NOT execute in any automatic parallel wave.
> This task deletes the legacy TypeScript converter and may only run AFTER a
> human explicitly confirms (a) the alias is repointed and (b) the Part-1 porting
> decisions are settled. A builder agent must refuse to proceed without that
> confirmation recorded in TASKS.md.

## Objective
Delete the legacy `tools/convert_blog_post_ts/` directory. Its only remaining
consumer is the `$generateor` alias, which B4 repoints to `src/types`. Removal is
safe once the alias is moved and the genuine Part-1 porting gaps are resolved or
explicitly dropped.

## Why it is gated
- Only DESTRUCTIVE action in this initiative; irreversible without git history.
- Must not run until:
  1. **B4** has repointed `$generateor` → `src/types` (no import resolves to the
     TS tool anymore).
  2. The Part-1 gaps that matter are handled: **R1** (coverImage), **R3** (dual
     asset copy), and **R4** (CustomComponent decision — drop or port). R2/R5 are
     low-priority/optional and are NOT hard blockers, but note their status.
  3. A human has signed off (recorded in TASKS.md).

## Context for a cold agent
- Background: `docs/features.md` Part 1 ("removing the TS tool is safe once types
  are migrated") and the "Parity / Rust-only" section listing what Rust already
  covers.
- Before deleting, grep the WHOLE repo for any remaining references to
  `convert_blog_post_ts` and `$generateor` resolving to it. There must be none
  outside this task and historical docs.

## Target Files
### Delete
- `tools/convert_blog_post_ts/` (entire directory).

## Required Behaviors
- After deletion: `pnpm check` still passes (blog resolves `$generateor` →
  `src/types`).
- No source file references `tools/convert_blog_post_ts` or imports from it.
- The Rust tool still builds.

## Implementation Constraints
- Refuse to run without the human confirmation recorded in TASKS.md.
- Verify dependencies (B4, R1, R3, R4) are Done first.
- Grep for stragglers before `git rm -r`.

## Steps
- [ ] Confirm human sign-off recorded in TASKS.md.
- [ ] Confirm B4, R1, R3, R4 are Done.
- [ ] Repo-wide grep for `convert_blog_post_ts` / `$generateor` → TS; resolve any.
- [ ] `git rm -r tools/convert_blog_post_ts`.
- [ ] Run verify commands.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `pnpm check` and `cargo build` pass after deletion.
- No remaining references to the TS tool.

## Verify
```bash
grep -rn "convert_blog_post_ts" . --exclude-dir=node_modules --exclude-dir=.git || echo "no references"
pnpm check
cargo build --manifest-path tools/convert_blog_post/Cargo.toml
```

## Suggested Commit
```bash
git commit -m "chore!: remove legacy convert_blog_post_ts (gated, human-confirmed)"
```
