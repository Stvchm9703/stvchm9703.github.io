"""Tests for the ipynb_data_visualization runner (Phase 0) and serialize (Phase 1)."""

import json
from pathlib import Path

import pandas as pd
import pytest

from ipynb_data_visualization import load_notebook, run_notebook, scan_dataframes
from ipynb_data_visualization.serialize import build_envelope, serialize_dataframe

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


# ---------------------------------------------------------------------------
# Phase 1 — serialize_dataframe
# ---------------------------------------------------------------------------


class TestSerializeDataframe:
    """Unit tests for serialize_dataframe()."""

    def test_returns_table_and_shape(self) -> None:
        df = pd.DataFrame({"a": [1, 2, 3], "b": ["x", "y", "z"]})
        result = serialize_dataframe(df)
        assert "table" in result
        assert "_original_shape" in result
        assert result["_original_shape"] == [3, 2]

    def test_table_has_schema_and_data(self) -> None:
        df = pd.DataFrame({"name": ["Alice", "Bob"], "score": [10.0, 20.0]})
        result = serialize_dataframe(df)
        table = result["table"]
        assert "schema" in table
        assert "data" in table
        assert "fields" in table["schema"]

    def test_schema_fields_match_columns(self) -> None:
        df = pd.DataFrame({"name": ["Alice", "Bob", "Carol"], "score": [10, 20, 30]})
        result = serialize_dataframe(df)
        field_names = [f["name"] for f in result["table"]["schema"]["fields"]]
        for col in df.columns:
            assert col in field_names

    def test_data_rows_match_dataframe(self) -> None:
        df = pd.DataFrame({"x": [1, 2, 3]})
        result = serialize_dataframe(df)
        assert len(result["table"]["data"]) == 3

    def test_max_rows_truncates_data(self) -> None:
        df = pd.DataFrame({"x": list(range(10))})
        result = serialize_dataframe(df, max_rows=3)
        assert len(result["table"]["data"]) == 3
        # Original shape still reflects the full frame
        assert result["_original_shape"] == [10, 1]

    def test_max_rows_no_truncation_when_under_limit(self) -> None:
        df = pd.DataFrame({"x": list(range(5))})
        result = serialize_dataframe(df, max_rows=10)
        assert len(result["table"]["data"]) == 5
        assert result["_original_shape"] == [5, 1]

    def test_raises_on_non_dataframe(self) -> None:
        with pytest.raises(TypeError):
            serialize_dataframe([1, 2, 3])  # type: ignore[arg-type]


# ---------------------------------------------------------------------------
# Phase 1 — build_envelope
# ---------------------------------------------------------------------------


class TestBuildEnvelope:
    """Unit tests for build_envelope() using the fixture notebook."""

    def _run(self, **kwargs) -> dict:
        return run_notebook(str(SAMPLE_NB), verbose=False, capture_intermediate=True, **kwargs)

    def test_envelope_top_level_keys(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        assert set(envelope.keys()) == {"fileUrl", "generatedAt", "captures"}

    def test_file_url(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        assert envelope["fileUrl"] == str(SAMPLE_NB)

    def test_generated_at_is_utc_string(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        ts = envelope["generatedAt"]
        assert isinstance(ts, str)
        assert ts.endswith("Z")

    def test_captures_list_is_nonempty(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        assert len(envelope["captures"]) > 0

    def test_capture_keys(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        for cap in envelope["captures"]:
            assert "cellIndex" in cap
            assert "executionOrder" in cap
            assert "isStopPoint" in cap
            assert "dataframes" in cap

    def test_cell_index_is_absolute(self) -> None:
        """cellIndex must use absolute notebook cell indices, not code-cell order.

        sample.ipynb layout: md(0), code(1), md(2), code(3), code(4)
        The first code cell has absolute index 1, NOT 0.
        """
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        cell_indices = [cap["cellIndex"] for cap in envelope["captures"]]
        # All absolute indices must be within [1, 4] (the actual code cell positions)
        for idx in cell_indices:
            assert idx in {1, 3, 4}, f"Unexpected cellIndex {idx}; expected 1, 3, or 4"

    def test_first_capture_cell_index(self) -> None:
        """First code cell is at absolute notebook index 1."""
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        # The first capture should correspond to absolute cell index 1
        assert envelope["captures"][0]["cellIndex"] == 1

    def test_dataframes_list_in_capture(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        # Every capture should have at least one DataFrame entry
        for cap in envelope["captures"]:
            assert len(cap["dataframes"]) > 0
            for df_entry in cap["dataframes"]:
                assert "name" in df_entry
                assert "shape" in df_entry
                assert "table" in df_entry

    def test_dataframe_entry_has_schema_and_data(self) -> None:
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        # Find the capture for cell 1 (first code cell; creates 'df')
        cap1 = next((c for c in envelope["captures"] if c["cellIndex"] == 1), None)
        assert cap1 is not None, "No capture for cellIndex=1"
        df_entry = next((d for d in cap1["dataframes"] if d["name"] == "df"), None)
        assert df_entry is not None, "No 'df' in capture for cellIndex=1"
        table = df_entry["table"]
        assert "schema" in table
        assert "data" in table
        assert "fields" in table["schema"]
        # df has columns: name, score → both must appear in fields
        field_names = [f["name"] for f in table["schema"]["fields"]]
        assert "name" in field_names
        assert "score" in field_names

    def test_shape_reflects_original(self) -> None:
        """shape must reflect the original DataFrame size, even with max_rows truncation."""
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"], max_rows=1)
        cap1 = next((c for c in envelope["captures"] if c["cellIndex"] == 1), None)
        assert cap1 is not None
        df_entry = next((d for d in cap1["dataframes"] if d["name"] == "df"), None)
        assert df_entry is not None
        # 'df' has 3 rows × 2 columns; shape must be the original
        assert df_entry["shape"] == [3, 2]
        # But data should be truncated to 1 row
        assert len(df_entry["table"]["data"]) == 1

    def test_max_rows_truncation_in_envelope(self) -> None:
        """max_rows=2 should limit data rows but not the shape field."""
        result = self._run()
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"], max_rows=2)
        for cap in envelope["captures"]:
            for df_entry in cap["dataframes"]:
                if df_entry["table"] is not None:
                    assert len(df_entry["table"]["data"]) <= 2

    def test_stop_point_flag(self) -> None:
        """isStopPoint must be True for cells matching stop_at."""
        result = run_notebook(str(SAMPLE_NB), verbose=False, stop_at=0, capture_intermediate=True)
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"])
        # execution_order 0 is at absolute cellIndex 1
        stop_caps = [c for c in envelope["captures"] if c["isStopPoint"]]
        assert len(stop_caps) >= 1
        assert stop_caps[0]["cellIndex"] == 1

    def test_custom_generated_at(self) -> None:
        result = self._run()
        ts = "2026-01-01T00:00:00Z"
        envelope = build_envelope(str(SAMPLE_NB), result["cell_snapshots"], generated_at=ts)
        assert envelope["generatedAt"] == ts


# ---------------------------------------------------------------------------
# Phase 2 — CLI --out-dir output convention
# ---------------------------------------------------------------------------


class TestCliOutDir:
    """Verify ``--out-dir`` writes ``<DIR>/<stem>.viz.json``."""

    def test_out_dir_creates_viz_json(self, tmp_path: Path) -> None:
        from ipynb_data_visualization.cli import main

        rc = main([str(SAMPLE_NB), "--out-dir", str(tmp_path), "--quiet"])
        assert rc == 0
        expected = tmp_path / f"{SAMPLE_NB.stem}.viz.json"
        assert expected.exists(), f"Expected {expected} to exist"

    def test_out_dir_content_is_valid_envelope(self, tmp_path: Path) -> None:
        from ipynb_data_visualization.cli import main

        main([str(SAMPLE_NB), "--out-dir", str(tmp_path), "--quiet"])
        out = tmp_path / f"{SAMPLE_NB.stem}.viz.json"
        envelope = json.loads(out.read_text())
        assert "fileUrl" in envelope
        assert "generatedAt" in envelope
        assert "captures" in envelope

    def test_out_dir_stem_matches_notebook_name(self, tmp_path: Path) -> None:
        from ipynb_data_visualization.cli import main

        main([str(SAMPLE_NB), "--out-dir", str(tmp_path), "--quiet"])
        # Only one file should be created; its name must be <notebook-stem>.viz.json
        created = list(tmp_path.glob("*.viz.json"))
        assert len(created) == 1
        expected_name = f"{SAMPLE_NB.stem}.viz.json"
        assert created[0].name == expected_name, f"Expected {expected_name}, got {created[0].name}"

    def test_out_dir_created_if_absent(self, tmp_path: Path) -> None:
        from ipynb_data_visualization.cli import main

        nested = tmp_path / "subdir" / "output"
        rc = main([str(SAMPLE_NB), "--out-dir", str(nested), "--quiet"])
        assert rc == 0
        assert (nested / f"{SAMPLE_NB.stem}.viz.json").exists()


# ---------------------------------------------------------------------------
# Phase 3 — CLI --sidecar output convention
# ---------------------------------------------------------------------------


class TestCliSidecar:
    """Verify ``--sidecar`` writes ``<notebook-dir>/<stem>.viz.json`` next to the .ipynb."""

    def test_sidecar_written_next_to_notebook(self, tmp_path: Path) -> None:
        import shutil

        from ipynb_data_visualization.cli import main

        # Copy the fixture notebook into a temp directory so we don't pollute the fixture dir.
        nb_copy = tmp_path / SAMPLE_NB.name
        shutil.copy(SAMPLE_NB, nb_copy)

        rc = main([str(nb_copy), "--sidecar", "--quiet"])
        assert rc == 0
        sidecar = tmp_path / f"{SAMPLE_NB.stem}.viz.json"
        assert sidecar.exists(), f"Expected sidecar at {sidecar}"

    def test_sidecar_filename_convention(self, tmp_path: Path) -> None:
        import shutil

        from ipynb_data_visualization.cli import main

        nb_copy = tmp_path / SAMPLE_NB.name
        shutil.copy(SAMPLE_NB, nb_copy)
        main([str(nb_copy), "--sidecar", "--quiet"])

        # Filename must be <stem>.viz.json — no extra suffix on top of .ipynb.
        expected_name = f"{SAMPLE_NB.stem}.viz.json"
        assert (tmp_path / expected_name).exists()

    def test_sidecar_content_is_valid_envelope(self, tmp_path: Path) -> None:
        import shutil

        from ipynb_data_visualization.cli import main

        nb_copy = tmp_path / SAMPLE_NB.name
        shutil.copy(SAMPLE_NB, nb_copy)
        main([str(nb_copy), "--sidecar", "--quiet"])

        sidecar = tmp_path / f"{SAMPLE_NB.stem}.viz.json"
        envelope = json.loads(sidecar.read_text())
        assert "fileUrl" in envelope
        assert "captures" in envelope
        # cell indices must be absolute and match expected positions
        cell_indices = {cap["cellIndex"] for cap in envelope["captures"]}
        assert cell_indices.issubset({1, 3, 4})

    def test_sidecar_and_out_dir_both_written(self, tmp_path: Path) -> None:
        """--sidecar and --out-dir can be combined; both outputs are written."""
        import shutil

        from ipynb_data_visualization.cli import main

        nb_copy = tmp_path / "notebooks" / SAMPLE_NB.name
        nb_copy.parent.mkdir(parents=True)
        shutil.copy(SAMPLE_NB, nb_copy)
        out_dir = tmp_path / "out"

        rc = main([str(nb_copy), "--sidecar", "--out-dir", str(out_dir), "--quiet"])
        assert rc == 0
        assert (nb_copy.parent / f"{SAMPLE_NB.stem}.viz.json").exists()
        assert (out_dir / f"{SAMPLE_NB.stem}.viz.json").exists()
