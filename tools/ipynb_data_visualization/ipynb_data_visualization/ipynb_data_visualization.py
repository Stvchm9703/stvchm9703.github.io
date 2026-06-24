"""
ipynb_runner.py — Execute .ipynb cells sequentially and capture DataFrames.

How it works:
1. Parse the .ipynb file (it's just JSON).
2. Maintain a shared namespace dict (simulating a Jupyter kernel).
3. exec() each code cell in that namespace.
4. After each cell, scan the namespace for pandas DataFrame instances.
5. Return a report of all captured DataFrames with metadata.

This mirrors how Jupyter works:
- Jupyter kernel keeps a persistent globals() dict across cells
- We do the same with exec(code, namespace)
- Variables survive between cells, just like in a real notebook
"""

import json
import sys
import io
import traceback
from typing import Any
from copy import deepcopy


def load_notebook(path: str) -> list[dict]:
    """Load an .ipynb file and return its code cells."""
    with open(path, "r", encoding="utf-8") as f:
        nb = json.load(f)

    cells = []
    for i, cell in enumerate(nb.get("cells", [])):
        if cell["cell_type"] == "code":
            # source can be a list of lines or a single string
            source = cell["source"]
            if isinstance(source, list):
                source = "".join(source)
            cells.append({
                "index": i,
                "source": source,
                "execution_order": len(cells),
            })
    return cells


def scan_dataframes(namespace: dict) -> dict[str, "pd.DataFrame"]:
    """
    Scan namespace for all pandas DataFrame instances.

    This is the key trick — after exec(), all variables the cell created
    live in `namespace`. We just check isinstance() on each value.
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
            pass  # skip objects that can't be inspected
    return found


def run_notebook(
    path: str,
    stop_at: list[int] | int | None = None,
    capture_intermediate: bool = True,
    verbose: bool = True,
) -> dict[str, Any]:
    """
    Execute an .ipynb file cell-by-cell and capture DataFrames.

    Args:
        path: Path to the .ipynb file.
        stop_at: Cell number(s) at which to pause and capture state.
                 - None: run all cells
                 - int: pause and capture at that single cell number
                 - list[int]: pause and capture at each specified cell number
                 Cells are 0-indexed by code cell execution order.
                 Execution continues through all specified stop points.
        capture_intermediate: If True, snapshot DataFrames after EVERY cell.
                              If False, only snapshot at stop_at cells and the end.
        verbose: Print execution progress.

    Returns:
        {
            "namespace": dict,          # the final execution namespace
            "dataframes": {name: df},   # all DataFrames at final stop point
            "cell_snapshots": [         # per-cell capture history
                {
                    "cell_index": int,
                    "source": str,
                    "stdout": str,
                    "stderr": str,
                    "error": str | None,
                    "dataframes_after": {name: (shape, dtypes, head)},
                    "is_stop_point": bool,
                }
            ],
            "stop_captures": {          # snapshots at each stop_at point
                cell_number: {name: df_copy}
            },
        }
    """
    cells = load_notebook(path)

    # Normalize stop_at into a sorted set of cell numbers
    if stop_at is None:
        stop_points = set()
        max_cell = len(cells)  # run all
    elif isinstance(stop_at, int):
        stop_points = {stop_at}
        max_cell = max(stop_at + 1, 0)
    else:
        stop_points = set(stop_at)
        max_cell = max(stop_at) + 1 if stop_at else len(cells)

    # Only execute up to the last stop point (or all cells if none specified)
    cells = cells[:max_cell]

    # --- The shared namespace (simulates a Jupyter kernel's globals) ---
    # We seed it with __builtins__ so exec'd code has access to print(), etc.
    namespace: dict[str, Any] = {"__builtins__": __builtins__}
    stop_captures: dict[int, dict] = {}

    snapshots = []

    for i, cell in enumerate(cells):
        source = cell["source"].strip()
        if not source:
            continue

        if verbose:
            header = f"{'='*60}\n[Cell {i}] (notebook index {cell['index']})\n{'='*60}"
            print(header)
            # Show first 3 lines of code as preview
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
            # ============================================================
            # THIS IS THE CORE: exec() with a persistent namespace
            #
            # exec(code, globals_dict) runs the code using globals_dict
            # as both the global and local scope. Any variables assigned
            # become entries in globals_dict, persisting across cells.
            #
            # This is *exactly* what IPython/Jupyter does internally
            # (simplified — real IPython also compiles with ast and
            # handles magics, display hooks, etc.)
            # ============================================================
            exec(compile(source, f"<cell_{i}>", "exec"), namespace)
        except Exception as e:
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

        # --- Scan for DataFrames after this cell ---
        is_stop = i in stop_points
        snapshot = {
            "cell_index": cell["index"],
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
                    # Store a reference to the actual DataFrame
                    "_dataframe": df,
                }
                if verbose:
                    print(f"[DataFrame] '{name}' — shape={df.shape}")

        snapshots.append(snapshot)

        # --- Capture frozen copies at stop points ---
        if is_stop:
            dfs_at_stop = scan_dataframes(namespace)
            stop_captures[i] = {name: df.copy() for name, df in dfs_at_stop.items()}
            if verbose:
                print(f">>> STOP POINT at cell {i} — captured {len(dfs_at_stop)} DataFrame(s)")

        if verbose:
            print()

    # Final state
    final_dfs = scan_dataframes(namespace)

    return {
        "namespace": namespace,
        "dataframes": final_dfs,
        "cell_snapshots": snapshots,
        "stop_captures": stop_captures,
    }


def get_dataframe(result: dict, name: str):
    """Convenience: extract a captured DataFrame by variable name from final state."""
    return result["dataframes"].get(name)


def get_stop_capture(result: dict, cell_number: int, name: str | None = None):
    """
    Get DataFrames captured at a specific stop point.

    Args:
        result: The result dict from run_notebook().
        cell_number: The cell number of the stop point.
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
    """List all DataFrame variable names captured."""
    return list(result["dataframes"].keys())


def print_summary(result: dict):
    """Print a nice summary of the execution."""
    print(f"\n{'='*60}")
    print("EXECUTION SUMMARY")
    print(f"{'='*60}")
    print(f"Cells executed: {len(result['cell_snapshots'])}")

    stop_cells = [s["execution_order"] for s in result["cell_snapshots"] if s.get("is_stop_point")]
    if stop_cells:
        print(f"Stop points hit: {stop_cells}")

    errors = [s for s in result["cell_snapshots"] if s["error"]]
    print(f"Cells with errors: {len(errors)}")

    dfs = result["dataframes"]
    print(f"\nDataFrames at final state: {len(dfs)}")
    for name, df in dfs.items():
        print(f"  • {name}: shape={df.shape}, columns={list(df.columns)}")
        print(f"    dtypes: {dict(df.dtypes)}")
        print(f"    memory: {df.memory_usage(deep=True).sum() / 1024:.1f} KB")
        print()


# =====================================================================
# DEMO / CLI
# =====================================================================
if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Run .ipynb and capture DataFrames")
    parser.add_argument("notebook", help="Path to .ipynb file")
    parser.add_argument("--stop-at", type=int, nargs="+", default=None,
                        help="Cell number(s) to pause and capture at (0-indexed). "
                             "E.g. --stop-at 1 3 5")
    parser.add_argument("--quiet", action="store_true",
                        help="Suppress per-cell output")
    args = parser.parse_args()

    result = run_notebook(
        args.notebook,
        stop_at=args.stop_at,
        verbose=not args.quiet,
    )
    print_summary(result)

    # Show stop-point captures
    if result["stop_captures"]:
        print(f"\n{'='*60}")
        print("STOP-POINT CAPTURES")
        print(f"{'='*60}")
        for cell_num, dfs in sorted(result["stop_captures"].items()):
            print(f"\n  Cell {cell_num}:")
            for name, df in dfs.items():
                print(f"    {name}: shape={df.shape}, columns={list(df.columns)}")

    # Interactive: drop into REPL with captured data
    if result["dataframes"]:
        print("\nTip: DataFrames are available in result['dataframes']")
        print("     Stop-point snapshots in result['stop_captures'][cell_num]")
