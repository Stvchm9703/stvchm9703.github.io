"""
serialize.py - Convert captured DataFrames to Table-Schema JSON + build the per-notebook envelope.

The per-DataFrame payload is exactly json.loads(df.to_json(orient="table")), which yields:
  {
    "schema": { "fields": [...], "primaryKey": [...], "pandas_version": "..." },
    "data": [...]
  }

The per-notebook envelope follows §3 of implementation_plan.md:
  {
    "fileUrl": "<notebook path>",
    "generatedAt": "<ISO-8601 UTC timestamp>",
    "captures": [
      {
        "cellIndex": <int>,       # ABSOLUTE notebook cell index (matches convert_blog_post's cellNumber)
        "executionOrder": <int>,  # code-cell execution order
        "isStopPoint": <bool>,
        "dataframes": [
          {
            "name": "<variable name>",
            "shape": [<rows>, <cols>],   # ORIGINAL shape, before any max_rows truncation
            "table": { /* orient="table" payload */ }
          }
        ]
      }
    ]
  }
"""

import json
from datetime import datetime, timezone
from typing import Any


def serialize_dataframe(df: Any, max_rows: int | None = None) -> dict:
    """Convert a pandas DataFrame to Table-Schema JSON payload.

    Args:
        df: A pandas DataFrame.
        max_rows: If given, truncate to at most this many rows in the ``data``
                  array. The ``shape`` field always reflects the ORIGINAL frame.

    Returns:
        A dict with ``schema`` and ``data`` keys (orient="table" payload),
        plus an ``_original_shape`` key (removed before building the envelope,
        used only internally).
    """
    import pandas as pd  # noqa: PLC0415

    if not isinstance(df, pd.DataFrame):
        raise TypeError(f"Expected pandas DataFrame, got {type(df)!r}")

    original_shape = list(df.shape)

    if max_rows is not None and len(df) > max_rows:
        df_serialized = df.head(max_rows)
    else:
        df_serialized = df

    table_payload: dict = json.loads(df_serialized.to_json(orient="table"))

    return {
        "_original_shape": original_shape,
        "table": table_payload,
    }


def build_envelope(
    notebook_path: str,
    cell_snapshots: list[dict],
    max_rows: int | None = None,
    *,
    generated_at: str | None = None,
) -> dict:
    """Build the per-notebook envelope from runner cell_snapshots.

    Args:
        notebook_path: Path (or URL) to the notebook file, used as ``fileUrl``.
        cell_snapshots: The ``cell_snapshots`` list from ``run_notebook()`` output.
            Each entry must have:
              - ``cell_index`` (int) — absolute notebook cell index
              - ``execution_order`` (int) — code-cell execution order
              - ``is_stop_point`` (bool)
              - ``dataframes_after`` (dict of {name: {shape, ..., _dataframe: pd.DataFrame}})
        max_rows: Maximum rows to include in each DataFrame's ``data`` array.
            The ``shape`` field always reflects the original frame size.
        generated_at: ISO-8601 UTC timestamp string. Defaults to now.

    Returns:
        The envelope dict as documented in §3 of implementation_plan.md.
    """
    if generated_at is None:
        generated_at = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    captures = []
    for snap in cell_snapshots:
        dfs_after = snap.get("dataframes_after", {})
        if not dfs_after:
            continue  # skip cells with no DataFrames

        dataframes_list = []
        for name, df_meta in dfs_after.items():
            # Retrieve the live DataFrame reference stored by the runner
            df = df_meta.get("_dataframe")
            if df is None:
                # Fallback: reconstruct shape info only (no live df available)
                original_shape = list(df_meta.get("shape", [0, 0]))
                dataframes_list.append(
                    {
                        "name": name,
                        "shape": original_shape,
                        "table": None,
                    }
                )
                continue

            serialized = serialize_dataframe(df, max_rows=max_rows)
            original_shape = serialized["_original_shape"]

            dataframes_list.append(
                {
                    "name": name,
                    "shape": original_shape,
                    "table": serialized["table"],
                }
            )

        if dataframes_list:
            captures.append(
                {
                    "cellIndex": snap["cell_index"],        # ABSOLUTE notebook cell index
                    "executionOrder": snap["execution_order"],
                    "isStopPoint": bool(snap.get("is_stop_point", False)),
                    "dataframes": dataframes_list,
                }
            )

    return {
        "fileUrl": notebook_path,
        "generatedAt": generated_at,
        "captures": captures,
    }
