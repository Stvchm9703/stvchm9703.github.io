"""
cli.py - Command-line interface for ipynb_data_visualization.

Entry point: ipynb_data_visualization:main (registered in pyproject.toml).
"""

import argparse
import json
import sys
from pathlib import Path

from .runner import list_dataframes, print_summary, run_notebook


def main(argv: list[str] | None = None) -> int:
    """Main CLI entry point.

    Args:
        argv: Argument list (defaults to sys.argv[1:] if None).

    Returns:
        Exit code (0 = success, non-zero = error).
    """
    parser = argparse.ArgumentParser(
        prog="ipynb_data_visualization",
        description="Execute a Jupyter notebook cell-by-cell and capture DataFrames.",
    )
    parser.add_argument(
        "notebook",
        help="Path to the .ipynb file to execute.",
    )
    parser.add_argument(
        "-o",
        "--output",
        metavar="FILE",
        default=None,
        help="Write captured DataFrame summary as JSON to FILE (default: stdout).",
    )
    parser.add_argument(
        "--stop-at",
        type=int,
        nargs="+",
        default=None,
        metavar="CELL",
        help=(
            "Code-cell execution order(s) at which to pause and capture state "
            "(0-indexed). E.g. --stop-at 1 3 5. Default: capture only at end."
        ),
    )
    parser.add_argument(
        "--all-cells",
        action="store_true",
        help="Capture DataFrame snapshot after every cell (default: stop points + end only).",
    )
    parser.add_argument(
        "--max-rows",
        type=int,
        default=None,
        metavar="N",
        help="Truncate each captured DataFrame to at most N rows in the output.",
    )
    parser.add_argument(
        "--quiet",
        action="store_true",
        help="Suppress per-cell execution output.",
    )

    args = parser.parse_args(argv)

    notebook_path = Path(args.notebook)
    if not notebook_path.exists():
        print(f"Error: notebook not found: {notebook_path}", file=sys.stderr)
        return 1

    result = run_notebook(
        str(notebook_path),
        stop_at=args.stop_at,
        capture_intermediate=args.all_cells,
        verbose=not args.quiet,
    )

    if not args.quiet:
        print_summary(result)

    # Build a JSON-serialisable summary
    df_names = list_dataframes(result)
    output_data: dict = {
        "notebook": str(notebook_path),
        "cells_executed": len(result["cell_snapshots"]),
        "dataframes": [],
    }

    import pandas as pd  # noqa: PLC0415 (late import after successful run)

    for name in df_names:
        df: pd.DataFrame = result["dataframes"][name]
        rows = df
        if args.max_rows is not None and len(df) > args.max_rows:
            rows = df.head(args.max_rows)
        output_data["dataframes"].append(
            {
                "name": name,
                "shape": list(df.shape),
                "columns": list(df.columns),
                "dtypes": {col: str(dtype) for col, dtype in df.dtypes.items()},
                "data": json.loads(rows.to_json(orient="records")),
            }
        )

    json_output = json.dumps(output_data, indent=2, default=str)

    if args.output:
        out_path = Path(args.output)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(json_output, encoding="utf-8")
        if not args.quiet:
            print(f"\nOutput written to: {out_path}")
    else:
        # Only print JSON to stdout when not in verbose mode to avoid mixing with log output
        if args.quiet:
            print(json_output)

    return 0


if __name__ == "__main__":
    sys.exit(main())
