use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::jupyter_notebook::model::JupyterNotebookRoot;
use crate::jupyter_notebook::util::resolve_jupyter_cell_output;

use super::mark::Mark;
use super::text::{TextComponentAttr, TextStyle};
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupyterComponentAttr {
    pub text: String,
    pub style: TextStyle,
    pub marks: Option<Vec<Mark>>,
    pub file_name: String,
    pub cell_number: usize,
    pub cell: Option<JupyterCell>,
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
}
