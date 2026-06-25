---
id: P1
title: "Phase 1 — Core: execute → capture → Table-Schema JSON"
subsystem: ipynb_data_visualization (Python/uv)
agent: builder
depends_on: [P0]
blocks: [P2, P3]
parallel_wave: 2
gated: false
files:
  create:
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/serialize.py
  modify:
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/runner.py
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/cli.py
    - tools/ipynb_data_visualization/tests/test_runner.py
verify:
  - "cd tools/ipynb_data_visualization && uv run pytest"
---

# Task P1 — Phase 1: Core feature (Table-Schema JSON)

## Objective
Execute a notebook, capture the pandas DataFrames produced, and emit them as
pandas Table-Schema JSON (`df.to_json(orient="table")`) inside a per-notebook
envelope. This is the core deliverable of the tool.

## Context for a cold agent
- **Depends on P0** (stabilized `src/` package, runner, CLI).
- Plan: `tools/ipynb_data_visualization/docs/implementation_plan.md` §1, §3, §4
  Phase 1.
- Per-DataFrame payload is exactly `json.loads(df.to_json(orient="table"))` →
  `{ schema: { fields, primaryKey, pandas_version }, data: [...] }`.
- Per-notebook envelope (§3):
  ```jsonc
  {
    "fileUrl": "data-preparating.ipynb",
    "generatedAt": "2026-...Z",
    "captures": [
      { "cellIndex": 1, "executionOrder": 0, "isStopPoint": true,
        "dataframes": [ { "name": "df", "shape": [120,5], "table": { /* orient=table */ } } ] }
    ]
  }
  ```
- **Indexing contract (critical):** key every capture by `cellIndex` = ABSOLUTE
  index into `notebook.cells` (matches `convert_blog_post`'s `cellNumber`, see
  `convert_blog_post/src/export_model/content_block/jupyter.rs`). Also record the
  code-cell `executionOrder`. The runner already tracks both.
- Bounds: `--max-rows` truncates/samples large frames; always record the original
  `shape` (Table-Schema `data` is row-expanded and can be large).
- Execution safety (document): arbitrary code runs — recommend isolated venv, add a
  per-notebook `--timeout`, treat inputs as trusted (author's own notebooks).

## Target Files
### Create
- `serialize.py` — DataFrame → `orient="table"` (honoring `--max-rows`) + envelope builder.
### Modify
- `runner.py` — finalize `load_notebook` / `run_notebook` / `scan_dataframes`,
  preserving stop-point/snapshot behavior; expose captures with absolute cellIndex.
- `cli.py` — write the envelope to `-o`; wire `--max-rows`, `--timeout`.
- `tests/test_runner.py` — assert envelope shape, Table-Schema payload, cellIndex,
  shape, and `--max-rows` truncation.

## Required Behaviors
- For each captured DataFrame, emit `table` = `json.loads(df.to_json(orient="table"))`.
- Build the per-notebook envelope keyed by absolute `cellIndex` with
  `executionOrder`, `isStopPoint`, per-frame `name`/`shape`/`table`.
- `--max-rows` truncates/samples; `shape` always reflects the ORIGINAL frame.
- `--timeout` enforced per notebook.
- CLI writes the envelope JSON to `-o`.

## Implementation Constraints
- Do not break the stop-point/snapshot semantics from P0's runner.
- `cellIndex` MUST be the absolute notebook cell index (downstream contract).
- Deterministic JSON output where feasible (stable key order).

## Steps
- [ ] Implement `serialize.py` (frame → table payload + envelope builder).
- [ ] Finalize runner capture API (absolute cellIndex + executionOrder).
- [ ] Wire CLI: write envelope to `-o`, `--max-rows`, `--timeout`.
- [ ] Tests: envelope, table payload, indexing, max-rows.
- [ ] `uv run pytest` green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `uv run pytest` passes.
- Running the CLI on the fixture notebook produces a valid envelope with at least
  one `table` payload keyed by the correct absolute `cellIndex`.

## Verify
```bash
cd tools/ipynb_data_visualization && uv run pytest
```

## Suggested Commit
```bash
git commit -m "feat(ipynb_viz): Phase 1 execute->capture->Table-Schema envelope"
```
