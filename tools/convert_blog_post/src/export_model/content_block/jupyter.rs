use std::path::Path;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::jupyter_notebook::model::JupyterNotebookRoot;
use crate::jupyter_notebook::util::resolve_jupyter_cell_output;

use super::mark::Mark;
use super::text::{TextComponentAttr, TextStyle};

// ---------------------------------------------------------------------------
// Sidecar (.viz.json) structs — mirrors the Python envelope shape.
// `table` is kept as an opaque JSON value so we don't have to enumerate
// every Table-Schema field here; the blog consumer handles it.
// ---------------------------------------------------------------------------

/// One DataFrame entry inside a sidecar capture.
///
/// JSON shape (camelCase via serde rename_all):
/// ```json
/// { "name": "df", "shape": [120, 5], "table": { "schema": {...}, "data": [...] } }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataTableEntry {
    pub name: String,
    pub shape: Vec<usize>,
    pub table: serde_json::Value,
}

/// One element of the `captures` array in the `.viz.json` envelope.
/// Only the fields we need are parsed; extras are ignored.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VizCapture {
    pub cell_index: usize,
    pub dataframes: Vec<DataTableEntry>,
}

/// Top-level structure of a `<stem>.viz.json` sidecar produced by the
/// Python `ipynb_data_visualization` tool.
#[derive(Debug, Clone, Deserialize)]
struct VizEnvelope {
    pub captures: Vec<VizCapture>,
}

// ---------------------------------------------------------------------------
// JupyterComponentAttr
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupyterComponentAttr {
    pub text: String,
    pub style: TextStyle,
    pub marks: Option<Vec<Mark>>,
    pub file_name: String,
    pub cell_number: usize,
    pub cell: Option<JupyterCell>,
    /// DataFrames attached from the sidecar `.viz.json`, matched by
    /// `cellIndex == cell_number`.  Absent (`null`) when no sidecar exists or
    /// this cell has no captured frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tables: Option<Vec<DataTableEntry>>,
}
pub type JupyterCell = crate::jupyter_notebook::model::Cell;
static CUSTOM_COMPONENT_JUPYTER: lazy_regex::Lazy<Regex> = lazy_regex::lazy_regex!(
    r"^/custom_component:jupyter:\((?P<file_name>[\w\-/\\\.]+\.[\w]+):(?P<cell_num>\d+)\)/$"i
);

impl JupyterComponentAttr {
    pub fn is_notebook_command(text: &str) -> bool {
        CUSTOM_COMPONENT_JUPYTER.is_match(text)
    }
    pub fn from_text_attr(attr: &TextComponentAttr) -> Self {
        let d = CUSTOM_COMPONENT_JUPYTER.captures(&attr.text).unwrap();
        let file_name = d.name("file_name").unwrap().as_str().to_string();
        let cell_number = d.name("cell_num").unwrap().as_str().parse().unwrap();

        Self {
            text: attr.text.clone(),
            style: attr.style.clone(),
            marks: attr.marks.clone(),
            file_name,
            cell_number,
            cell: None,
            data_tables: None,
        }
    }

    pub fn add_notebook_file(&mut self, file: &JupyterNotebookRoot) {
        let mut cloned_nb = file.to_owned();
        let cell_s = cloned_nb.cells.get_mut(self.cell_number);
        if let Some(cell) = cell_s {
            if let JupyterCell::Code(code) = cell {
                if let Some(outputs) = code.outputs.as_mut() {
                    for output in outputs.iter_mut() {
                        if let Some(data) = output.data.as_mut() {
                            if let Some(text_html) = data.text_html.as_mut() {
                                if let Ok(html) = resolve_jupyter_cell_output(text_html) {
                                    *text_html = html;
                                }
                            }
                        }
                    }
                }
            }
            self.cell = Some(cell.to_owned());
        }
    }

    /// Try to load data tables from a sidecar `<notebook-stem>.viz.json` file.
    ///
    /// `notebook_file_path` is the resolved path to the `.ipynb` asset (as
    /// stored in `JupyterNotebookRoot::file_url` or the path that was used to
    /// open the notebook).  The sidecar is expected at
    /// `<parent>/<stem>.viz.json`.
    ///
    /// If the sidecar does not exist, cannot be parsed, or contains no capture
    /// matching `self.cell_number`, `data_tables` is left as `None` — existing
    /// output is unaffected.
    pub fn load_sidecar(&mut self, notebook_file_path: &Path) {
        let stem = match notebook_file_path.file_stem() {
            Some(s) => s.to_string_lossy().into_owned(),
            None => return,
        };
        let sidecar_path = notebook_file_path.with_file_name(format!("{stem}.viz.json"));

        let Ok(content) = std::fs::read_to_string(&sidecar_path) else {
            return;
        };

        let Ok(envelope) = serde_json::from_str::<VizEnvelope>(&content) else {
            return;
        };

        // Find the capture whose cellIndex matches this block's cell_number.
        // cellIndex in the sidecar is the ABSOLUTE notebook cell index, which
        // is the same value stored in cell_number (see implementation_plan.md).
        let tables: Vec<DataTableEntry> = envelope
            .captures
            .into_iter()
            .filter(|cap| cap.cell_index == self.cell_number)
            .flat_map(|cap| cap.dataframes)
            .collect();

        if !tables.is_empty() {
            self.data_tables = Some(tables);
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Create a temporary directory that is cleaned up when the guard is dropped.
    struct TmpDir(std::path::PathBuf);
    impl TmpDir {
        fn new(suffix: &str) -> Self {
            let p = std::env::temp_dir().join(format!("cbp_test_{suffix}_{}", std::process::id()));
            fs::create_dir_all(&p).unwrap();
            Self(p)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for TmpDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn make_sidecar(dir: &Path, stem: &str, cell_index: usize) -> std::path::PathBuf {
        let sidecar = dir.join(format!("{stem}.viz.json"));
        let content = serde_json::json!({
            "fileUrl": format!("{stem}.ipynb"),
            "generatedAt": "2026-01-01T00:00:00Z",
            "captures": [
                {
                    "cellIndex": cell_index,
                    "executionOrder": 0,
                    "isStopPoint": false,
                    "dataframes": [
                        {
                            "name": "df",
                            "shape": [3, 2],
                            "table": {
                                "schema": {
                                    "fields": [
                                        {"name": "index", "type": "integer"},
                                        {"name": "col_a", "type": "string"}
                                    ],
                                    "primaryKey": ["index"],
                                    "pandas_version": "2.0.0"
                                },
                                "data": [
                                    {"index": 0, "col_a": "alpha"},
                                    {"index": 1, "col_a": "beta"},
                                    {"index": 2, "col_a": "gamma"}
                                ]
                            }
                        }
                    ]
                }
            ]
        });
        fs::write(&sidecar, content.to_string()).unwrap();
        sidecar
    }

    fn make_notebook_path(dir: &Path, stem: &str) -> std::path::PathBuf {
        let nb = dir.join(format!("{stem}.ipynb"));
        // The file doesn't need real content — we only need the path for sidecar resolution.
        fs::write(&nb, "{}").unwrap();
        nb
    }

    #[test]
    fn test_load_sidecar_present_matches_cell() {
        let tmp = TmpDir::new("sidecar_match");
        let stem = "test_notebook";
        let cell_num = 3usize;
        make_sidecar(tmp.path(), stem, cell_num);
        let nb_path = make_notebook_path(tmp.path(), stem);

        let mut attr = JupyterComponentAttr {
            file_name: format!("{stem}.ipynb"),
            cell_number: cell_num,
            ..Default::default()
        };
        attr.load_sidecar(&nb_path);

        let tables = attr.data_tables.expect("data_tables should be Some when sidecar matches");
        assert_eq!(tables.len(), 1);
        assert_eq!(tables[0].name, "df");
        assert_eq!(tables[0].shape, vec![3, 2]);
        assert!(tables[0].table.get("schema").is_some());
        assert!(tables[0].table.get("data").is_some());
    }

    #[test]
    fn test_load_sidecar_absent_leaves_data_tables_none() {
        let tmp = TmpDir::new("sidecar_absent");
        let nb_path = tmp.path().join("no_sidecar.ipynb");
        fs::write(&nb_path, "{}").unwrap();

        let mut attr = JupyterComponentAttr {
            file_name: "no_sidecar.ipynb".to_string(),
            cell_number: 1,
            ..Default::default()
        };
        attr.load_sidecar(&nb_path);

        assert!(attr.data_tables.is_none(), "data_tables must be None when no sidecar exists");
    }

    #[test]
    fn test_load_sidecar_cell_index_mismatch_leaves_data_tables_none() {
        let tmp = TmpDir::new("sidecar_mismatch");
        let stem = "mismatch_nb";
        // Sidecar has capture for cellIndex=5, but block has cell_number=2
        make_sidecar(tmp.path(), stem, 5);
        let nb_path = make_notebook_path(tmp.path(), stem);

        let mut attr = JupyterComponentAttr {
            file_name: format!("{stem}.ipynb"),
            cell_number: 2,
            ..Default::default()
        };
        attr.load_sidecar(&nb_path);

        assert!(attr.data_tables.is_none(), "data_tables must be None when cellIndex does not match");
    }

    #[test]
    fn test_data_table_entry_serializes_camel_case() {
        let entry = DataTableEntry {
            name: "result_df".to_string(),
            shape: vec![10, 3],
            table: serde_json::json!({"schema": {}, "data": []}),
        };
        let json = serde_json::to_string(&entry).unwrap();
        // field names must be camelCase in the JSON output
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"shape\""));
        assert!(json.contains("\"table\""));
    }

    #[test]
    fn test_jupyter_component_attr_data_tables_omitted_when_none() {
        let attr = JupyterComponentAttr::default();
        let json = serde_json::to_string(&attr).unwrap();
        // data_tables must be absent (not null) when None
        assert!(!json.contains("dataTables"), "dataTables must be omitted when None, got: {json}");
    }

    #[test]
    fn test_jupyter_component_attr_data_tables_present_when_some() {
        let mut attr = JupyterComponentAttr::default();
        attr.data_tables = Some(vec![DataTableEntry {
            name: "df".to_string(),
            shape: vec![5, 2],
            table: serde_json::json!({}),
        }]);
        let json = serde_json::to_string(&attr).unwrap();
        assert!(json.contains("dataTables"), "dataTables must appear in JSON when Some");
    }
}
