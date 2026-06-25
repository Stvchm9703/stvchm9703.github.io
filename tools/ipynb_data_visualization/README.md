# ipynb_data_visualization

A command-line tool that **executes a Jupyter notebook cell-by-cell**, captures
every pandas `DataFrame` produced, and emits them as
**Table-Schema JSON** (`df.to_json(orient="table")`) wrapped in a
per-notebook envelope.

It ships two integration modes:

- **Standalone** — write `<out-dir>/<stem>.viz.json` for a blog/site to render
  as interactive data tables.
- **Sidecar pre-processor** — write `<stem>.viz.json` next to the `.ipynb` so
  `tools/convert_blog_post` can attach the tables to its `JupyterComponent`
  blocks.

---

## Install

Requires [uv](https://docs.astral.sh/uv/).

```sh
uv sync
```

This installs the package in editable mode together with all dev dependencies.
The `ipynb_data_visualization` command is available inside the venv managed by
uv (`uv run ipynb_data_visualization …`) or after `uv tool install --editable .`.

---

## CLI usage

```
ipynb_data_visualization <notebook> [options]
```

### Positional argument

| Argument   | Description                      |
|------------|----------------------------------|
| `notebook` | Path to the `.ipynb` file to run |

### Options

| Flag / option           | Description |
|-------------------------|-------------|
| `-o FILE`, `--output FILE` | Write the envelope JSON to `FILE`. |
| `--out-dir DIR`         | Write `<DIR>/<stem>.viz.json` (standalone convention). The directory is created if it does not exist. |
| `--sidecar`             | Write `<stem>.viz.json` next to the `.ipynb` (sidecar / pre-processor convention). |
| `--stop-at CELL …`     | Code-cell execution indices (0-indexed, space-separated) at which to pause and capture state. Default: capture only at the end. |
| `--all-cells`           | Capture a DataFrame snapshot after *every* code cell (default: stop points + end only). |
| `--max-rows N`          | Truncate each captured DataFrame to at most `N` rows in the output. The `shape` field always records the original size. |
| `--quiet`               | Suppress per-cell execution output. When no output file is specified, the envelope JSON is printed to stdout only in `--quiet` mode. |

### Output priority

When more than one output flag is given, all destinations are written.
Priority for a single flag: `-o` > `--out-dir` > `--sidecar` > stdout
(stdout only when `--quiet` is set and no file destination is given).

### Examples

```sh
# Execute and print envelope to stdout (quiet mode)
uv run ipynb_data_visualization analysis.ipynb --quiet

# Standalone: write <stem>.viz.json into a dist/ directory
uv run ipynb_data_visualization analysis.ipynb --out-dir dist/data

# Sidecar: write next to the notebook (for convert_blog_post)
uv run ipynb_data_visualization notebooks/analysis.ipynb --sidecar

# Capture after every cell, limit rows, write to an explicit file
uv run ipynb_data_visualization analysis.ipynb --all-cells --max-rows 200 -o out/analysis.viz.json
```

---

## Output format

### Per-DataFrame payload (Table-Schema)

Each captured DataFrame is serialised via `df.to_json(orient="table")`:

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

### Per-notebook envelope

The tool wraps all captures in a single envelope written to the output file:

```jsonc
{
  "fileUrl": "analysis.ipynb",       // path/URL passed as <notebook>
  "generatedAt": "2026-06-24T12:00:00Z",  // ISO-8601 UTC; overridable via API
  "captures": [
    {
      "cellIndex": 3,           // ABSOLUTE index into notebook.cells (matches
                                //   convert_blog_post's cellNumber)
      "executionOrder": 1,      // code-cell execution order (0-indexed)
      "isStopPoint": true,      // true when cell is a --stop-at target
      "dataframes": [
        {
          "name": "df",         // variable name in the notebook namespace
          "shape": [120, 5],    // original shape, before any --max-rows truncation
          "table": { /* Table-Schema payload above */ }
        }
      ]
    }
  ]
}
```

**Indexing contract:** `cellIndex` is the **absolute** index into
`notebook.cells` (not the code-cell-only counter). This matches the
`cellNumber` field that `convert_blog_post` uses when resolving Jupyter embeds.

---

## Integrations

### Standalone — blog "Data" tab

1. Run the tool with `--out-dir <path>`:

   ```sh
   uv run ipynb_data_visualization notebooks/analysis.ipynb --out-dir src/data
   ```

2. The blog's Svelte Jupyter renderer (`src/lib/components/post-content-layout/block/custom/jupyter.svelte`)
   looks up `<stem>.viz.json` by `fileName` + `cellNumber` and renders a
   **"Data"** tab next to the existing *Outputs* and *Source* tabs.

### Sidecar pre-processor — `convert_blog_post`

The Rust converter (`tools/convert_blog_post`) reads sidecar files to populate
`JupyterComponentAttr.data_tables`.

**Pipeline order — important:** run `ipynb_data_visualization --sidecar` BEFORE
`convert_blog_post`, so the sidecar `.viz.json` exists when the Rust tool scans
the notebook's `files/` directory.

```sh
# Step 1 — produce the sidecar
uv run ipynb_data_visualization export/files/analysis.ipynb --sidecar

# Step 2 — run the Rust converter (picks up the sidecar automatically)
convert_blog_post export/
```

---

## Determinism / reproducibility

`generatedAt` defaults to the current UTC time. When using the Python API
directly, pass `generated_at="..."` to `build_envelope()` to produce
byte-for-byte identical output across runs:

```python
from ipynb_data_visualization import run_notebook, build_envelope
result = run_notebook("analysis.ipynb")
envelope = build_envelope(
    notebook_path="analysis.ipynb",
    cell_snapshots=result["cell_snapshots"],
    generated_at="2026-01-01T00:00:00Z",   # fixed timestamp for reproducibility
)
```

For CI / caching purposes, a simple skip-when-unchanged strategy is to compare
the content hash of the `.ipynb` file against the `fileUrl`+`generatedAt` from
the existing `.viz.json` before re-running.

---

## Development

```sh
uv sync              # install deps + dev deps
uv run pytest        # run tests
uv run ruff check .  # lint
uv run ruff format . # format
```

For more details see [docs/development.md](docs/development.md),
[docs/installation.md](docs/installation.md), and
[docs/implementation_plan.md](docs/implementation_plan.md).
