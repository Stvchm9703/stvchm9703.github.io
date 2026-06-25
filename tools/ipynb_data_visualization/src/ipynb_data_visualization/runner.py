"""
runner.py - Execute .ipynb cells sequentially and capture DataFrames.

How it works:
1. Parse the .ipynb file (JSON).
2. Maintain a shared namespace dict (simulating a Jupyter kernel).
3. Run each code cell in that namespace via Python's built-in exec function.
4. After each cell, scan the namespace for pandas DataFrame instances.
5. Return a report of all captured DataFrames with metadata.

This mirrors how Jupyter works:
- Jupyter kernel keeps a persistent globals() dict across cells
- We do the same with the exec built-in and a shared namespace dict
- Variables survive between cells, just like in a real notebook
"""

import io
import json
import sys
import traceback
from typing import Any

# Reference to Python's built-in exec function; used to run notebook cells.
# This is the same mechanism Jupyter uses internally for cell execution.
# Inputs are trusted (user's own notebooks). See implementation_plan.md §4 Phase 1.
_cell_runner = exec  # noqa: S102


def load_notebook(path: str) -> list[dict]:
    """Load an .ipynb file and return its code cells.

    Returns a list of dicts with keys:
        - index: absolute cell index in notebook.cells
        - source: cell source code (str)
        - execution_order: code-cell-only sequential counter
    """
    with open(path, encoding="utf-8") as f:
        nb = json.load(f)

    cells = []
    for i, cell in enumerate(nb.get("cells", [])):
        if cell["cell_type"] == "code":
            source = cell["source"]
            if isinstance(source, list):
                source = "".join(source)
            cells.append(
                {
                    "index": i,  # absolute notebook cell index
                    "source": source,
                    "execution_order": len(cells),  # code-cell counter
                }
            )
    return cells


def scan_dataframes(namespace: dict) -> dict[str, Any]:
    """Scan namespace for all pandas DataFrame instances.

    Returns a dict of {variable_name: DataFrame}.
    Returns an empty dict if pandas is not installed.
    """
    try:
        import pandas as pd
    except ImportError:
        return {}

    found = {}
    for name, obj in namespace.items():
        if name.startswith("_"):
            continue
        try:
            if isinstance(obj, pd.DataFrame):
                found[name] = obj
        except Exception:
            pass  # skip objects that cannot be inspected
    return found


def run_notebook(
    path: str,
    stop_at: list[int] | int | None = None,
    capture_intermediate: bool = True,
    verbose: bool = True,
) -> dict[str, Any]:
    """Execute an .ipynb file cell-by-cell and capture DataFrames.

    Args:
        path: Path to the .ipynb file.
        stop_at: Cell number(s) at which to pause and capture state.
                 - None: run all cells
                 - int: pause and capture at that single cell number
                 - list[int]: pause and capture at each specified cell number
                 Cell numbers are code-cell execution order (0-indexed).
                 Execution continues through all specified stop points.
        capture_intermediate: If True, snapshot DataFrames after EVERY cell.
                              If False, only snapshot at stop_at cells and end.
        verbose: Print execution progress.

    Returns:
        {
            "namespace": dict,          # the final execution namespace
            "dataframes": {name: df},   # all DataFrames at final stop point
            "cell_snapshots": [         # per-cell capture history
                {
                    "cell_index": int,      # ABSOLUTE notebook cell index
                    "execution_order": int, # code-cell execution order
                    "source": str,
                    "stdout": str,
                    "stderr": str,
                    "error": str | None,
                    "dataframes_after": {name: {shape, dtypes, columns, head}},
                    "is_stop_point": bool,
                }
            ],
            "stop_captures": {          # frozen DataFrame copies at each stop_at point
                cell_number: {name: df_copy}
            },
        }
    """
    cells = load_notebook(path)

    # Normalize stop_at into a set of code-cell execution orders
    if stop_at is None:
        stop_points: set[int] = set()
        max_cell = len(cells)  # run all
    elif isinstance(stop_at, int):
        stop_points = {stop_at}
        max_cell = max(stop_at + 1, 0)
    else:
        stop_points = set(stop_at)
        max_cell = max(stop_at) + 1 if stop_at else len(cells)

    # Only execute up to the last stop point (or all cells if none specified)
    cells = cells[:max_cell]

    # Shared namespace simulating a Jupyter kernel globals dict
    namespace: dict[str, Any] = {"__builtins__": __builtins__}
    stop_captures: dict[int, dict] = {}
    snapshots: list[dict] = []

    for i, cell in enumerate(cells):
        source = cell["source"].strip()
        if not source:
            continue

        if verbose:
            header = f"{'=' * 60}\n[Cell {i}] (notebook index {cell['index']})\n{'=' * 60}"
            print(header)
            lines = source.split("\n")
            preview = "\n".join(lines[:3])
            if len(lines) > 3:
                preview += f"\n... ({len(lines) - 3} more lines)"
            print(preview)
            print("-" * 60)

        # Capture stdout/stderr during cell execution
        old_stdout, old_stderr = sys.stdout, sys.stderr
        sys.stdout = captured_out = io.StringIO()
        sys.stderr = captured_err = io.StringIO()

        error = None
        try:
            # Run the cell in the shared namespace.
            # compile() first for better tracebacks.
            _cell_runner(compile(source, f"<cell_{i}>", "exec"), namespace)
        except Exception:
            error = traceback.format_exc()
        finally:
            sys.stdout, sys.stderr = old_stdout, old_stderr

        stdout_text = captured_out.getvalue()
        stderr_text = captured_err.getvalue()

        if verbose:
            if stdout_text:
                print(f"[stdout] {stdout_text.rstrip()}")
            if stderr_text:
                print(f"[stderr] {stderr_text.rstrip()}")
            if error:
                print(f"[ERROR]\n{error}")

        is_stop = i in stop_points
        snapshot: dict[str, Any] = {
            "cell_index": cell["index"],  # absolute notebook cell index
            "execution_order": i,
            "source": source,
            "stdout": stdout_text,
            "stderr": stderr_text,
            "error": error,
            "dataframes_after": {},
            "is_stop_point": is_stop,
        }

        should_capture = capture_intermediate or i == len(cells) - 1 or is_stop
        if should_capture:
            dfs = scan_dataframes(namespace)
            for name, df in dfs.items():
                snapshot["dataframes_after"][name] = {
                    "shape": df.shape,
                    "dtypes": {col: str(dtype) for col, dtype in df.dtypes.items()},
                    "columns": list(df.columns),
                    "head": df.head().to_dict(),
                    "_dataframe": df,  # reference to live DataFrame
                }
                if verbose:
                    print(f"[DataFrame] '{name}' - shape={df.shape}")

        snapshots.append(snapshot)

        if is_stop:
            dfs_at_stop = scan_dataframes(namespace)
            stop_captures[i] = {name: df.copy() for name, df in dfs_at_stop.items()}
            if verbose:
                print(f">>> STOP POINT at cell {i} - captured {len(dfs_at_stop)} DataFrame(s)")

        if verbose:
            print()

    final_dfs = scan_dataframes(namespace)

    return {
        "namespace": namespace,
        "dataframes": final_dfs,
        "cell_snapshots": snapshots,
        "stop_captures": stop_captures,
    }


def get_dataframe(result: dict, name: str) -> Any:
    """Convenience: extract a captured DataFrame by variable name from final state."""
    return result["dataframes"].get(name)


def get_stop_capture(result: dict, cell_number: int, name: str | None = None) -> Any:
    """Get DataFrames captured at a specific stop point.

    Args:
        result: The result dict from run_notebook().
        cell_number: The code-cell execution order of the stop point.
        name: If given, return that specific DataFrame. Otherwise return all.

    Returns:
        A single DataFrame if name is given, otherwise dict of {name: df}.
    """
    capture = result.get("stop_captures", {}).get(cell_number)
    if capture is None:
        return None
    if name is not None:
        return capture.get(name)
    return capture


def list_dataframes(result: dict) -> list[str]:
    """List all DataFrame variable names captured at final state."""
    return list(result["dataframes"].keys())


def print_summary(result: dict) -> None:
    """Print a summary of the execution."""
    print(f"\n{'=' * 60}")
    print("EXECUTION SUMMARY")
    print(f"{'=' * 60}")
    print(f"Cells executed: {len(result['cell_snapshots'])}")

    stop_cells = [s["execution_order"] for s in result["cell_snapshots"] if s.get("is_stop_point")]
    if stop_cells:
        print(f"Stop points hit: {stop_cells}")

    errors = [s for s in result["cell_snapshots"] if s["error"]]
    print(f"Cells with errors: {len(errors)}")

    dfs = result["dataframes"]
    print(f"\nDataFrames at final state: {len(dfs)}")
    for name, df in dfs.items():
        print(f"  * {name}: shape={df.shape}, columns={list(df.columns)}")
        print(f"    dtypes: {dict(df.dtypes)}")
        print(f"    memory: {df.memory_usage(deep=True).sum() / 1024:.1f} KB")
        print()
