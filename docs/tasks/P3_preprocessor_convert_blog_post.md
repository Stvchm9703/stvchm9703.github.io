---
id: P3
title: "Phase 3 — Integration B: pre-processor for convert_blog_post (incl. Rust)"
subsystem: ipynb_data_visualization + convert_blog_post (Rust) + blog
agent: builder
depends_on: [P1]
blocks: [P4]
parallel_wave: 3
gated: false
files:
  modify:
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/cli.py        # sidecar emission next to .ipynb
    - tools/convert_blog_post/src/export_model/content_block/jupyter.rs          # JupyterComponentAttr.data_tables
    - tools/convert_blog_post/src/jupyter_notebook/util.rs                       # load sidecar in add_notebook_file (locate via codegraph)
    - src/lib/components/post-content-layout/block/custom/jupyter.svelte         # render data_tables via Phase 2 component
verify:
  - "cargo test --manifest-path tools/convert_blog_post/Cargo.toml"
  - "cd tools/ipynb_data_visualization && uv run pytest"
  - "pnpm check"
---

# Task P3 — Phase 3: pre-processor for `convert_blog_post`

## Objective
Make the Python tool act as a pre-processor for the Rust converter via a **sidecar
JSON** carrier, extend the Rust `JupyterComponent` to load and attach those data
tables, and render them in the blog using the Phase 2 component.

## Context for a cold agent
- **Depends on P1** (envelope/Table-Schema). Independent of P2 (different carrier)
  but reuses P2's blog renderer for display — if P2 is not yet merged, render
  defensively and note the dependency on its component.
- Plan: `tools/ipynb_data_visualization/docs/implementation_plan.md` §4 Phase 3 +
  open question #4 (sidecar is the chosen carrier, NOT notebook injection).
- Carrier = sidecar JSON: emit `<notebook-stem>.viz.json` next to the `.ipynb` in
  the Anytype export's `files/`, keyed by `cellIndex` + variable name.
- Rust changes (`convert_blog_post`):
  - Extend `JupyterComponentAttr` (`content_block/jupyter.rs`) with an optional
    `data_tables` field.
  - Load the sidecar in `add_notebook_file` / `jupyter_notebook::util` (locate
    exact symbols via codegraph). Key by `cellNumber` (== `cellIndex`).
  - Do NOT take the heavier alternative (injecting `application/json` output into
    the executed notebook / editing `jupyter_notebook/model.rs` `Data`).
- Blog renders `data_tables` via the Phase 2 Table-Schema component.

## Target Files
See frontmatter. Three subsystems touched; keep each change minimal and behind the
optional `data_tables` field so existing output is unaffected when no sidecar
exists.

## Required Behaviors
- Python CLI can emit the sidecar `<stem>.viz.json` next to the `.ipynb`.
- `JupyterComponentAttr` gains optional `data_tables`; populated when a sidecar is
  present, absent/null otherwise (no change to existing output without a sidecar).
- Rust loads + matches sidecar entries by `cellNumber`/variable name.
- Blog renders attached `data_tables` via the Phase 2 component.

## Implementation Constraints
- `data_tables` MUST be optional — zero impact on pages without a sidecar.
- Reuse the absolute `cellIndex`/`cellNumber` join contract.
- Use codegraph to find `JupyterComponentAttr`, `add_notebook_file`, and
  `jupyter_notebook::util` before editing.
- Add Rust tests for sidecar-present and sidecar-absent.

## Steps
- [ ] Python: add sidecar emission to CLI.
- [ ] codegraph: locate `JupyterComponentAttr`, `add_notebook_file`, util loader.
- [ ] Rust: add optional `data_tables` + sidecar load/match + tests.
- [ ] Blog: render `data_tables` via the Phase 2 component.
- [ ] `cargo test` + `uv run pytest` + `pnpm check` green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `cargo test`, `uv run pytest`, `svelte-check` all pass.
- With a sidecar, a Jupyter block carries `data_tables` and the blog renders them;
  without a sidecar, output and rendering are unchanged.

## Verify
```bash
cargo test --manifest-path tools/convert_blog_post/Cargo.toml
cd tools/ipynb_data_visualization && uv run pytest
pnpm check
```

## Suggested Commit
```bash
git commit -m "feat: Phase 3 sidecar pre-processor + Rust data_tables + blog render"
```
