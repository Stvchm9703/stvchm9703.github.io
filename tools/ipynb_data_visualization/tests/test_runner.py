"""Tests for the ipynb_data_visualization runner (Phase 0)."""

import json
from pathlib import Path

import pandas as pd
import pytest

from ipynb_data_visualization import load_notebook, run_notebook, scan_dataframes

FIXTURE_DIR = Path(__file__).parent / "fixtures"
SAMPLE_NB = FIXTURE_DIR / "sample.ipynb"


# ---------------------------------------------------------------------------
# load_notebook
# ---------------------------------------------------------------------------


class TestLoadNotebook:
    def test_returns_only_code_cells(self) -> None:
        cells = load_notebook(str(SAMPLE_NB))
        # sample.ipynb has 2 markdown cells + 3 code cells
        assert len(cells) == 3

    def test_absolute_indices_correct(self) -> None:
        """Absolute notebook cell index must match position in notebook.cells."""
        cells = load_notebook(str(SAMPLE_NB))
        # notebook cell layout: md(0), code(1), md(2), code(3), code(4)
        assert cells[0]["index"] == 1
        assert cells[1]["index"] == 3
        assert cells[2]["index"] == 4

    def test_execution_order_sequential(self) -> None:
        cells = load_notebook(str(SAMPLE_NB))
        orders = [c["execution_order"] for c in cells]
        assert orders == list(range(len(cells)))

    def test_source_is_string(self) -> None:
        cells = load_notebook(str(SAMPLE_NB))
        for cell in cells:
            assert isinstance(cell["source"], str)
            assert len(cell["source"]) > 0


# ---------------------------------------------------------------------------
# scan_dataframes
# ---------------------------------------------------------------------------


class TestScanDataframes:
    def test_finds_dataframe(self) -> None:
        ns = {"df": pd.DataFrame({"a": [1, 2]}), "_private": pd.DataFrame({"b": [3]})}
        found = scan_dataframes(ns)
        assert "df" in found
        assert "_private" not in found  # private names skipped

    def test_ignores_non_dataframe(self) -> None:
        ns = {"x": 42, "s": "hello", "lst": [1, 2, 3]}
        found = scan_dataframes(ns)
        assert found == {}


# ---------------------------------------------------------------------------
# run_notebook - full execution
# ---------------------------------------------------------------------------


class TestRunNotebook:
    def test_returns_expected_keys(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        assert set(result.keys()) == {"namespace", "dataframes", "cell_snapshots", "stop_captures"}

    def test_captures_dataframes(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        dfs = result["dataframes"]
        # All three DataFrames created in the notebook should be present
        assert "df" in dfs
        assert "result_df" in dfs
        assert "summary" in dfs

    def test_df_shape_correct(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        df: pd.DataFrame = result["dataframes"]["df"]
        assert df.shape == (3, 2)
        assert list(df.columns) == ["name", "score"]

    def test_result_df_filtered(self) -> None:
        """result_df should contain only rows with score >= 20."""
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        result_df: pd.DataFrame = result["dataframes"]["result_df"]
        assert result_df.shape == (2, 3)
        assert all(result_df["score"] >= 20)

    def test_cell_snapshots_count(self) -> None:
        """All non-empty code cells should produce a snapshot."""
        result = run_notebook(str(SAMPLE_NB), verbose=False, capture_intermediate=True)
        # 3 code cells, all with non-empty source
        assert len(result["cell_snapshots"]) == 3

    def test_cell_index_is_absolute(self) -> None:
        """cell_index in snapshots must be the absolute notebook.cells index."""
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        indices = [s["cell_index"] for s in result["cell_snapshots"]]
        assert indices == [1, 3, 4]

    def test_no_errors(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        errors = [s for s in result["cell_snapshots"] if s["error"]]
        assert errors == [], f"Unexpected cell errors: {errors}"

    def test_stop_at_captures(self) -> None:
        """stop_at=0 should freeze a copy of DataFrames after the first code cell."""
        result = run_notebook(str(SAMPLE_NB), stop_at=0, verbose=False)
        assert 0 in result["stop_captures"]
        captured = result["stop_captures"][0]
        assert "df" in captured
        assert isinstance(captured["df"], pd.DataFrame)

    def test_stop_at_limits_execution(self) -> None:
        """With stop_at=0 (first code cell only), result_df should not exist."""
        result = run_notebook(str(SAMPLE_NB), stop_at=0, verbose=False)
        assert "result_df" not in result["dataframes"]

    def test_dataframes_are_pandas_dataframes(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        for name, df in result["dataframes"].items():
            assert isinstance(df, pd.DataFrame), f"'{name}' is not a DataFrame"


# ---------------------------------------------------------------------------
# Basic Table-Schema serialisation (preview of P1 output format)
# ---------------------------------------------------------------------------


class TestTableSchemaOutput:
    """Verify that captured DataFrames can be serialised to Table-Schema JSON."""

    def test_to_json_orient_table(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        df: pd.DataFrame = result["dataframes"]["df"]
        payload = json.loads(df.to_json(orient="table"))
        assert "schema" in payload
        assert "data" in payload
        assert "fields" in payload["schema"]
        assert len(payload["data"]) == 3  # 3 rows

    def test_schema_fields_match_columns(self) -> None:
        result = run_notebook(str(SAMPLE_NB), verbose=False)
        df: pd.DataFrame = result["dataframes"]["df"]
        payload = json.loads(df.to_json(orient="table"))
        field_names = [f["name"] for f in payload["schema"]["fields"]]
        # pandas adds the index as a field; non-index columns must be present
        for col in df.columns:
            assert col in field_names
