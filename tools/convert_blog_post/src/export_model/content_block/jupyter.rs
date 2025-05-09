use serde::{Deserialize, Serialize};

use crate::export_model::trait_impl::AddFromExternalFile;

use super::mark::Mark;
use super::text::TextStyle;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupyterComponentAttr {
    // "text": "/custom_component:jupyter:(data-preparating.ipynb:10)/",
    // "style": "Code",
    // "marks": {},
    // "fileName": "data-preparating.ipynb",
    // "cellNumber": "10",
    pub text: String,
    pub style: TextStyle,
    pub marks: Vec<Mark>,
    pub file_name: String,
    pub cell_number: usize,
    pub cell: JupyterCell,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupyterCell {}

// impl AddFromExternalFile {}
