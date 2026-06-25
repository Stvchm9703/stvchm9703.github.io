---
id: P2
title: "Phase 2 — Integration A: standalone viz JSON → blog component"
subsystem: ipynb_data_visualization + blog (src/)
agent: builder
depends_on: [P1]
blocks: [P4]
parallel_wave: 3
gated: false
files:
  create:
    - src/lib/components/post-content-layout/block/custom/data_table.svelte   # Table-Schema renderer (name per existing convention)
  modify:
    - src/lib/components/post-content-layout/block/custom/jupyter.svelte       # add a "Data" tab
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/cli.py       # output convention <out>/<stem>.viz.json
verify:
  - "pnpm check   # (fallback: bunx svelte-check) — blog component type-checks"
  - "cd tools/ipynb_data_visualization && uv run pytest"
---

# Task P2 — Phase 2: standalone → blog

## Objective
Wire the standalone tool output into the blog: emit `<out>/<notebook-stem>.viz.json`
and add a Svelte component that renders a Table-Schema payload as an interactive
table, surfaced as a new "Data" tab in the existing Jupyter renderer.

## Context for a cold agent
- **Depends on P1** (envelope + Table-Schema output exists).
- Plan: `tools/ipynb_data_visualization/docs/implementation_plan.md` §4 Phase 2.
- Output convention: `<out>/<notebook-stem>.viz.json` (one envelope per notebook).
- Blog: add a new component that renders a Table-Schema payload (`schema.fields`
  + `data`) as an interactive table (chart optional). Surface it in
  `src/lib/components/post-content-layout/block/custom/jupyter.svelte` as a new
  **"Data"** tab next to *Outputs* / *Source*, looked up by `fileName` +
  `cellNumber` (which equals the envelope's `cellIndex`).
- Existing custom block components live in
  `src/lib/components/post-content-layout/block/custom/` (`jupyter.svelte`,
  `jk.svelte`, `index.svelte`). Follow their conventions.

## Target Files
### Create
- A Table-Schema renderer Svelte component under
  `src/lib/components/post-content-layout/block/custom/`.
### Modify
- `custom/jupyter.svelte` — add the "Data" tab, looked up by `fileName` +
  `cellNumber`.
- `cli.py` — ensure standalone output lands as `<out>/<stem>.viz.json`.

## Required Behaviors
- CLI emits `<out>/<notebook-stem>.viz.json`.
- New component renders a Table-Schema payload as an interactive table without
  runtime error on empty/missing data.
- Jupyter renderer shows a "Data" tab when a matching viz payload exists for the
  `fileName` + `cellNumber`; hides/disables it otherwise.

## Implementation Constraints
- Match the existing tab/lookup pattern in `jupyter.svelte`.
- `cellNumber` (blog) == `cellIndex` (envelope) — use that to join.
- Keep the Python change minimal (output-path convention only).

## Steps
- [ ] Confirm P1 envelope shape.
- [ ] Add the Table-Schema renderer component.
- [ ] Add the "Data" tab + lookup in `jupyter.svelte`.
- [ ] Set CLI output convention `<out>/<stem>.viz.json`.
- [ ] `pnpm check` + `uv run pytest` green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `svelte-check` passes; `uv run pytest` passes.
- A notebook with a captured frame shows a working "Data" tab in the blog.

## Verify
```bash
pnpm check
cd tools/ipynb_data_visualization && uv run pytest
```

## Suggested Commit
```bash
git commit -m "feat: Phase 2 standalone viz JSON + blog Data tab"
```
