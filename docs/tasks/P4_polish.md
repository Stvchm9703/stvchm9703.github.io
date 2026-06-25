---
id: P4
title: "Phase 4 — Polish (docs, CI, optional caching)"
subsystem: ipynb_data_visualization (Python/uv)
agent: builder
depends_on: [P2, P3]
blocks: []
parallel_wave: 4
gated: false
files:
  modify:
    - tools/ipynb_data_visualization/README.md
    - tools/ipynb_data_visualization/.github/   # verify ci.yml runs lint+tests
verify:
  - "cd tools/ipynb_data_visualization && make"
  - "cd tools/ipynb_data_visualization && uv run pytest"
---

# Task P4 — Phase 4: Polish

## Objective
Finalize the Python tool: replace the template README with real docs (CLI + output
schema + both integrations), verify CI runs lint+tests, and optionally add
caching/determinism.

## Context for a cold agent
- **Depends on P2 and P3** (both integrations must exist to document them).
- Plan: `tools/ipynb_data_visualization/docs/implementation_plan.md` §4 Phase 4.
- Replace the Copier-template `README.md`.
- CI: `.github/workflows/ci.yml` already present — verify it runs lint+tests; fix
  if it references the old flat layout.
- Optional: caching/determinism (skip re-exec when notebook + inputs unchanged);
  publishing via existing `publish.yml`. Treat as optional/stretch.

## Target Files
### Modify
- `README.md` — document CLI flags, the envelope/Table-Schema output schema (§3 of
  the plan), and both integrations (standalone → blog "Data" tab; sidecar
  pre-processor → Rust `data_tables`).
- `.github/workflows/ci.yml` — confirm lint+tests run on the `src/` layout.

## Required Behaviors
- README accurately documents CLI, output schema, and both integration modes.
- CI passes on the current layout.
- Optional caching is gated behind a flag and does not change default behavior.

## Implementation Constraints
- Docs must match the actual CLI flags and output shape shipped in P0–P3.
- Keep optional caching opt-in.

## Steps
- [ ] Rewrite README (CLI + schema + integrations).
- [ ] Verify/fix CI workflow for the `src/` layout.
- [ ] (Optional) Add caching/determinism behind a flag.
- [ ] `make` + `uv run pytest` green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `make` and `uv run pytest` pass.
- README documents CLI, schema, and both integrations; CI is green.

## Verify
```bash
cd tools/ipynb_data_visualization && make
cd tools/ipynb_data_visualization && uv run pytest
```

## Suggested Commit
```bash
git commit -m "docs(ipynb_viz): Phase 4 polish - README, CI, optional caching"
```
