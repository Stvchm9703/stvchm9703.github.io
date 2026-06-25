# `ipynb_data_visualization` — Implementation Plan

## 1. Goal

A command-line tool that **executes a Jupyter notebook cell-by-cell**, captures
the pandas `DataFrame`s produced, and emits them as **pandas Table-Schema JSON
(`df.to_json(orient="table")`)** — i.e. each output carries a `schema`
(`fields`, `primaryKey`) and `data`.

It must work in **two modes**:

1. **Standalone CLI** — produce JSON the blog (`src/`) renders directly as
   interactive tables/charts.
2. **Pre-processor for `tools/convert_blog_post`** — produce sidecar artifacts
   that the Rust converter attaches to its `JupyterComponent` blocks.

Approach: **stabilize the scaffold first**, then layer on the feature work.

---

## 2. Current state (audit)

| Area | Finding | Action |
| ---- | ------- | ------ |
| `ipynb_data_visualization/ipynb_data_visualization.py` | Working notebook **runner**: `load_notebook`, `run_notebook` (exec in shared namespace, stop points, snapshots), `scan_dataframes`, CLI in `__main__`. | Keep & refactor into the package core. |
| `ipynb_data_visualization/data_cell.py` | **Broken**: missing `import json` / `from typing import List`; empty `class Attachments:` (IndentationError); forward-referenced types evaluated eagerly. Won't import. | Replace (use `nbformat` or a minimal typed model) or delete. |
| `ipynb_data_visualization/exec_cell.py` | **Empty**. | Implement (cell executor) or remove; consolidate into the runner. |
| `ipynb_data_visualization/__init__.py` | `from .ipynb_data_visualization import *`, empty `__all__`, **no `main`**. | Export public API + `main`. |
| `main.py` (repo root) | Hello-world stub, not the packaged entry. | Remove or make it `from ipynb_data_visualization import main`. |
| `pyproject.toml` | `dependencies = []` (but code needs **pandas**); entry point `ipynb_data_visualization:main` is **unresolved**; wheel target & tooling assume a **`src/` layout** that doesn't exist. | Add deps; fix entry point; adopt `src/` layout. |
| `tests/test_placeholder.py` | `assert True`. | Real tests. |
| `README.md` / `docs/*` | Copier template placeholders. | Fill in. |

---

## 3. Output format (the "data visualization format")

Per captured DataFrame, the payload is exactly `json.loads(df.to_json(orient="table"))`:

```jsonc
{
  "schema": {
    "fields": [
      { "name": "index", "type": "integer" },
      { "name": "name",  "type": "string"  },
      { "name": "score", "type": "number"  }
    ],
    "primaryKey": ["index"],
    "pandas_version": "2.x"
  },
  "data": [
    { "index": 0, "name": "A", "score": 1.0 },
    { "index": 1, "name": "B", "score": 2.0 }
  ]
}
```

Wrapped in a per-notebook envelope:

```jsonc
{
  "fileUrl": "data-preparating.ipynb",
  "generatedAt": "2026-06-24T…Z",
  "captures": [
    {
      "cellIndex": 1,          // ABSOLUTE notebook cell index (matches convert_blog_post's cellNumber)
      "executionOrder": 0,     // code-cell execution order
      "isStopPoint": true,
      "dataframes": [
        {
          "name": "df",        // variable name in the namespace
          "shape": [120, 5],
          "table": { /* orient="table" payload above */ }
        }
      ]
    }
  ]
}
```

> **Indexing contract (important):** `convert_blog_post` resolves Jupyter
> embeds with `cellNumber` = **absolute index into `notebook.cells`** (see
> `convert_blog_post/src/export_model/content_block/jupyter.rs`). The current
> runner tracks both the absolute `index` and the code-cell `execution_order`.
> Key all output by **`cellIndex` (absolute)** so the two tools line up.

**Bounds:** add `--max-rows` (truncate/sample large frames) and record the
original `shape`, since Table-Schema `data` is row-expanded and can be large.

---

## 4. Phased plan

### Phase 0 — Stabilize (make it install / run / test)
1. **Adopt `src/` layout** (matches `pyproject.toml`):
   `ipynb_data_visualization/ipynb_data_visualization/*` → `src/ipynb_data_visualization/*`.
2. **Declare dependencies:** `uv add pandas` (and `nbformat` if used for parsing).
3. **Add a CLI module** `cli.py` with `main()` (argparse: notebook path, `-o`,
   `--stop-at`, `--all-cells`, `--max-rows`, `--quiet`); export `main` from
   `__init__.py` so `[project.scripts] ipynb_data_visualization = "ipynb_data_visualization:main"` resolves.
4. **Fix `__init__.py`** `__all__` + public exports.
5. **Resolve `data_cell.py` / `exec_cell.py`:** delete the broken/empty files or
   replace with a single, working `runner.py` (refactor of the existing module).
   Prefer `nbformat` for robust `.ipynb` parsing.
6. **Tests:** load a tiny fixture notebook, run it, assert captured DataFrame +
   Table-Schema output. Replace the placeholder.
7. `make` (sync + lint + test) green; `uv tool install --editable .` works.

### Phase 1 — Core feature: execute → capture → Table-Schema JSON
1. Refactor the runner into `runner.py` (`load_notebook`, `run_notebook`,
   `scan_dataframes`) — preserve stop-point/snapshot behavior.
2. Add `serialize.py`: DataFrame → `orient="table"` (with `--max-rows`), build
   the per-notebook envelope (§3).
3. CLI writes the envelope to `-o`.
4. **Execution safety** (documented): arbitrary code runs; recommend running in
   an isolated venv, add a per-notebook `--timeout`, and treat inputs as trusted
   (the author's own notebooks).

### Phase 2 — Integration A: standalone → blog
1. Output convention: `<out>/<notebook-stem>.viz.json` (envelope per notebook).
2. Blog: new Svelte component that renders a Table-Schema payload as an
   interactive table (and optionally a chart). Surface it in the existing Jupyter
   renderer (`src/lib/components/post-content-layout/block/custom/jupyter.svelte`)
   as a new **"Data"** tab next to *Outputs* / *Source*, looked up by
   `fileName` + `cellNumber`.

### Phase 3 — Integration B: pre-processor for `convert_blog_post`
1. **Carrier = sidecar JSON** (decoupled; no notebook mutation): emit
   `<notebook-stem>.viz.json` next to the `.ipynb` in the Anytype export's
   `files/`, keyed by `cellIndex` + variable name.
2. **Rust changes** (`convert_blog_post`):
   - extend `JupyterComponentAttr` (`content_block/jupyter.rs`) with an optional
     `data_tables` field, and load the sidecar in `add_notebook_file` /
     `jupyter_notebook::util`.
   - (Alternative: add an `application/json` field to `Data` in
     `jupyter_notebook/model.rs` and inject a custom output into the executed
     notebook — heavier; not recommended.)
3. Blog renders `data_tables` via the Phase 2 component.

### Phase 4 — Polish
- Replace template `README.md`; document CLI + output schema + both integrations.
- CI (`.github/workflows/ci.yml` already present) runs lint+tests; verify.
- Optional: caching/determinism (skip re-exec when notebook + inputs unchanged),
  publishing via existing `publish.yml`.

---

## 5. Proposed package structure (after Phase 0/1)

```
tools/ipynb_data_visualization/
├── src/ipynb_data_visualization/
│   ├── __init__.py          # exports: main, run_notebook, serialize_dataframe
│   ├── cli.py               # argparse main()
│   ├── runner.py            # load/run/scan (refactor of current module)
│   ├── serialize.py         # DataFrame → orient="table" + envelope
│   └── py.typed
├── tests/
│   ├── fixtures/sample.ipynb
│   └── test_runner.py
└── docs/implementation_plan.md   # this file
```

---

## 6. Open questions / decisions for later

1. **Capture granularity default** — only stop points + final state, or every
   code cell? (Proposed: stop points + final; `--all-cells` to widen.)
2. **Non-DataFrame visuals** (matplotlib/plotly figures) — out of scope for v1
   (DataFrames only, per the chosen format); revisit if needed.
3. **`--max-rows` default** and whether to sample vs head-truncate.
4. **Pre-processor carrier** — confirm sidecar-JSON over notebook-injection
   (Phase 3 assumes sidecar).
5. **Where standalone output lands** for the blog (a `static/blog/assets/…`
   path, or merged into the `convert_blog_post` export `files/`).
```
