---
id: P0
title: "Phase 0 — Stabilize ipynb_data_visualization scaffold"
subsystem: ipynb_data_visualization (Python/uv)
agent: builder
depends_on: []
blocks: [P1]
parallel_wave: 1
gated: false
files:
  create:
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/__init__.py
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/cli.py
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/runner.py
    - tools/ipynb_data_visualization/src/ipynb_data_visualization/py.typed
    - tools/ipynb_data_visualization/tests/fixtures/sample.ipynb
    - tools/ipynb_data_visualization/tests/test_runner.py
  modify:
    - tools/ipynb_data_visualization/pyproject.toml
    - tools/ipynb_data_visualization/main.py
  delete:
    - tools/ipynb_data_visualization/ipynb_data_visualization/data_cell.py
    - tools/ipynb_data_visualization/ipynb_data_visualization/exec_cell.py
    - tools/ipynb_data_visualization/ipynb_data_visualization/   # flat package, after migration to src/
verify:
  - "cd tools/ipynb_data_visualization && make"
  - "cd tools/ipynb_data_visualization && uv run pytest"
---

# Task P0 — Phase 0: Stabilize (install / run / test)

## Objective
Make the Python tool installable, runnable, and testable. Adopt the `src/` layout
the `pyproject.toml` already assumes, declare deps, add a real CLI entry point, fix
`__init__.py`, remove the broken/empty modules, and replace the placeholder test.

## Context for a cold agent
- Plan: `tools/ipynb_data_visualization/docs/implementation_plan.md` §2 (audit) and
  §4 Phase 0; target structure in §5.
- Current state (verified):
  - Working runner: `ipynb_data_visualization/ipynb_data_visualization/ipynb_data_visualization.py`
    (`load_notebook`, `run_notebook`, `scan_dataframes`, CLI in `__main__`). Keep &
    refactor into the package core.
  - `data_cell.py` is **broken** (missing `import json` / `from typing import List`;
    empty `class Attachments:` → IndentationError). DELETE or replace.
  - `exec_cell.py` is **empty**. DELETE (consolidate into runner).
  - `__init__.py`: `from .ipynb_data_visualization import *`, empty `__all__`, no
    `main`. Fix to export public API + `main`.
  - `main.py` (tool root): hello-world stub. Make it `from ipynb_data_visualization
    import main` or remove.
  - `pyproject.toml`: `dependencies = []` but code needs **pandas**; entry point
    `ipynb_data_visualization:main` is unresolved; wheel/tooling assume `src/`
    layout that doesn't exist. The empty `src/` dir already exists.
  - `tests/test_placeholder.py`: `assert True`. Replace.
- Build/test: `make` (sync + lint + test) and `uv run pytest`. Run from
  `tools/ipynb_data_visualization/`.

## Target Files
See frontmatter `files`. Net effect: code lives under
`src/ipynb_data_visualization/`; flat `ipynb_data_visualization/` package removed.

## Required Behaviors
- `uv add pandas` (and `nbformat` if used for parsing) — deps declared in pyproject.
- `src/` layout in place; old flat package migrated and deleted.
- `cli.py` provides `main()` (argparse: notebook path, `-o`, `--stop-at`,
  `--all-cells`, `--max-rows`, `--quiet`); exported so
  `[project.scripts] ipynb_data_visualization = "ipynb_data_visualization:main"`
  resolves.
- `__init__.py` exports `main`, `run_notebook`, `load_notebook`, `scan_dataframes`
  with a correct `__all__`.
- `data_cell.py` and `exec_cell.py` removed (logic consolidated into `runner.py`,
  a refactor of the existing module). Prefer `nbformat` for `.ipynb` parsing.
- Tests load a tiny fixture notebook, run it, assert a captured DataFrame + basic
  Table-Schema output. Placeholder removed.
- `make` green; `uv tool install --editable .` works.

## Implementation Constraints
- Preserve the existing runner's stop-point / snapshot behavior during the refactor
  (the full Table-Schema feature is Phase 1 / P1 — keep P0 to stabilization).
- Keep the absolute-cell-index tracking (`index`) and code-cell `execution_order`
  intact — the indexing contract matters downstream (P1/P3).

## Steps
- [ ] Migrate runner into `src/ipynb_data_visualization/runner.py`.
- [ ] Add `cli.py` with argparse `main()`; fix `__init__.py` exports + `__all__`.
- [ ] `uv add pandas` (+ `nbformat` if used); fix entry point in pyproject.
- [ ] Delete `data_cell.py`, `exec_cell.py`, and the flat package dir.
- [ ] Make `main.py` re-export `main` or remove it.
- [ ] Add `tests/fixtures/sample.ipynb` + `tests/test_runner.py`; remove placeholder.
- [ ] `make` and `uv run pytest` green.
- [ ] Mark task complete in TASKS.md via the task-management skill.

## Acceptance Criteria
- `make` and `uv run pytest` pass from `tools/ipynb_data_visualization/`.
- The package imports cleanly; the CLI entry point resolves.

## Verify
```bash
cd tools/ipynb_data_visualization && make
cd tools/ipynb_data_visualization && uv run pytest
```

## Suggested Commit
```bash
git commit -m "chore(ipynb_viz): Phase 0 stabilize - src layout, deps, CLI, tests"
```
