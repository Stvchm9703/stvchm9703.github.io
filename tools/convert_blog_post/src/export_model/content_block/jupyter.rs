use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::jupyter_notbook::model::JupyterNotebookRoot;

use super::mark::Mark;
use super::text::{TextComponentAttr, TextStyle};
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupyterComponentAttr {
    // "text": "/custom_component:jupyter:(data-preparating.ipynb:10)/",
    // "style": "Code",
    // "marks": {},
    // "fileName": "data-preparating.ipynb",
    // "cellNumber": "10",
    pub text: String,
    pub style: TextStyle,
    pub marks: Option<Vec<Mark>>,
    pub file_name: String,
    pub cell_number: usize,
    pub cell: Option<JupyterCell>,
}
pub type JupyterCell = crate::jupyter_notbook::model::Cell;
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
        }
    }
    pub fn add_notebook_file(&mut self, file: &JupyterNotebookRoot) {
        // self.file_name = file_name;
        self.cell = file.cells.get(self.cell_number).cloned();
    }
}
