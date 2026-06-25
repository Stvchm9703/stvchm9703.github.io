use std::path::Path;

use super::Page;
use crate::{
    export_model::content_block::{
        ComponentAttrType, ContentBlock,
        jupyter::JupyterComponentAttr,
        text::TextStyle,
    },
    jupyter_notebook::model::JupyterNotebookRoot,
};

// use regex_static;

struct NotbookStore<'a>(&'a ContentBlock, &'a ContentBlock);

// let CUSTOM_COMPONENT_JUPYTER = regex_static::static_regex!("^\/custom_component\:jupyter\:\((\w+)\:(\d+)\)\/$");

impl Page {
    pub(crate) fn resolve_notebook(&mut self, notebook_list: &Vec<JupyterNotebookRoot>) {
        //
        // let mut store: Vec<NotbookStore> = vec![];
        // let mut tmp_store: &mut ContentBlock;
        let mut tmp_order: usize = 0;
        let mut tmp_notebook: JupyterComponentAttr = JupyterComponentAttr::default();
        let mut to_delete = vec![];
        let to_update: Vec<ContentBlock> = vec![];

        let values = self.cache_contents.values_mut();
        for value in values {
            if let ComponentAttrType::Text(t) = &value.component_attr {
                // store.push(NotbookStore(notebook, value));
                if t.style == TextStyle::Code && JupyterComponentAttr::is_notebook_command(&t.text)
                {
                    tmp_notebook = JupyterComponentAttr::from_text_attr(t);
                    tmp_order = value.order;
                    to_delete.push(value.id.to_owned());
                }
            }
            if let ComponentAttrType::File(f) = &value.component_attr {
                // store.push(NotbookStore(notebook, value));

                if f.file_url.contains(&tmp_notebook.file_name) && value.order == tmp_order + 1 {
                    let notebook = notebook_list.iter().find(|p| {
                        p.file_url
                            .as_ref()
                            .is_some_and(|f| f == &tmp_notebook.file_name)
                    });
                    if let Some(nb) = notebook {
                        tmp_notebook.add_notebook_file(nb);
                        // Load sidecar data tables if the notebook's filesystem path is known.
                        if let Some(ref sp) = nb.source_path {
                            tmp_notebook.load_sidecar(Path::new(sp));
                        }
                        // to_delete.push(value.id.to_owned());
                        let mut new_block = value.clone();
                        new_block.component_attr =
                            ComponentAttrType::JupyterComponent(tmp_notebook.to_owned());
                        // println!("new_block  : {:#?}", &new_block);

                        *value = new_block;
                        tmp_notebook = JupyterComponentAttr::default();
                    }
                }
            }
        }

        for delete_id in to_delete {
            self.cache_contents.remove(&delete_id);
        }
        for update in to_update {
            self.cache_contents.insert(update.id.to_owned(), update);
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests — end-to-end wire: resolve_notebook calls load_sidecar
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, fs, path::Path};

    use crate::export_model::content_block::{
        ComponentAttrType, ComponentStyle, ContentBlock,
        file::FileComponentAttr,
        text::{TextComponentAttr, TextStyle},
    };
    use crate::jupyter_notebook::model::{
        Cell, CodeCell, JupyterNotebookRoot, LanguageInfo, TopLevelMetadata,
        CodemirrorMode, Kernelspec,
    };

    use super::*;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_sidecar(dir: &Path, stem: &str, cell_index: usize) {
        let sidecar = dir.join(format!("{stem}.viz.json"));
        let content = serde_json::json!({
            "fileUrl": format!("{stem}.ipynb"),
            "generatedAt": "2026-01-01T00:00:00Z",
            "captures": [{
                "cellIndex": cell_index,
                "executionOrder": 0,
                "isStopPoint": false,
                "dataframes": [{
                    "name": "wire_df",
                    "shape": [2, 3],
                    "table": {
                        "schema": {
                            "fields": [{"name": "x", "type": "integer"}],
                            "primaryKey": ["x"],
                            "pandas_version": "2.0.0"
                        },
                        "data": [{"x": 1}, {"x": 2}]
                    }
                }]
            }]
        });
        fs::write(&sidecar, content.to_string()).unwrap();
    }

    fn make_tmp_dir(suffix: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir()
            .join(format!("cbp_wire_{suffix}_{}", std::process::id()));
        fs::create_dir_all(&p).unwrap();
        p
    }

    fn dummy_style() -> ComponentStyle {
        ComponentStyle::default()
    }

    fn code_text_block(id: &str, order: usize, text: &str) -> ContentBlock {
        ContentBlock {
            id: id.to_string(),
            order,
            component_attr: ComponentAttrType::Text(TextComponentAttr {
                text: text.to_string(),
                style: TextStyle::Code,
                ..Default::default()
            }),
            style: dummy_style(),
            ..Default::default()
        }
    }

    fn file_block(id: &str, order: usize, file_url: &str) -> ContentBlock {
        ContentBlock {
            id: id.to_string(),
            order,
            component_attr: ComponentAttrType::File(FileComponentAttr {
                file_url: file_url.to_string(),
                ..Default::default()
            }),
            style: dummy_style(),
            ..Default::default()
        }
    }

    fn minimal_notebook_root(
        stem: &str,
        cell_count: usize,
        source_path: Option<String>,
    ) -> JupyterNotebookRoot {
        let cells: Vec<Cell> = (0..cell_count)
            .map(|_| Cell::Code(CodeCell::default()))
            .collect();
        JupyterNotebookRoot {
            cells,
            metadata: TopLevelMetadata {
                kernelspec: Kernelspec {
                    display_name: "Python 3".to_string(),
                    language: "python".to_string(),
                    name: "python3".to_string(),
                },
                language_info: LanguageInfo {
                    codemirror_mode: CodemirrorMode {
                        name: "ipython".to_string(),
                        version: 3,
                    },
                    file_extension: ".py".to_string(),
                    mimetype: "text/x-python".to_string(),
                    name: "python".to_string(),
                    nbconvert_exporter: "python".to_string(),
                    pygments_lexer: "ipython3".to_string(),
                    version: "3.11.0".to_string(),
                },
                orig_nbformat: 4,
            },
            nbformat: 4,
            nbformat_minor: 5,
            file_url: Some(format!("{stem}.ipynb")),
            source_path,
        }
    }

    // -----------------------------------------------------------------------
    // Tests
    // -----------------------------------------------------------------------

    /// Full wire test: resolve_notebook calls load_sidecar via source_path,
    /// and the resulting JupyterComponent block carries data_tables.
    #[test]
    fn test_resolve_notebook_wires_sidecar_data_tables() {
        let tmp = make_tmp_dir("wire_sidecar");
        let stem = "wire_nb";
        let cell_num = 2usize;

        // Write the sidecar next to a dummy .ipynb path
        make_sidecar(&tmp, stem, cell_num);
        let nb_path = tmp.join(format!("{stem}.ipynb"));
        fs::write(&nb_path, "{}").unwrap(); // dummy file — path is all we need

        let file_name = format!("{stem}.ipynb");
        let cmd = format!("/custom_component:jupyter:({file_name}:{cell_num})/");

        // Build a Page with: Code command block (order 0) + File block (order 1).
        let mut cache: BTreeMap<String, ContentBlock> = BTreeMap::new();
        cache.insert("cmd".to_string(), code_text_block("cmd", 0, &cmd));
        cache.insert("file".to_string(), file_block("file", 1, &file_name));

        let mut page = Page {
            cache_contents: cache,
            ..Default::default()
        };

        // The notebook list has source_path set (as main.rs would do).
        let nb = minimal_notebook_root(
            stem,
            cell_num + 1, // need at least cell_num cells
            Some(nb_path.to_string_lossy().into_owned()),
        );
        let notebook_list = vec![nb];

        page.resolve_notebook(&notebook_list);

        // The File block should now be a JupyterComponent with data_tables populated.
        let file_block = page.cache_contents.get("file").expect("file block must remain");
        if let ComponentAttrType::JupyterComponent(attr) = &file_block.component_attr {
            let tables = attr
                .data_tables
                .as_ref()
                .expect("data_tables must be Some when sidecar matches");
            assert_eq!(tables.len(), 1);
            assert_eq!(tables[0].name, "wire_df");
            assert_eq!(tables[0].shape, vec![2, 3]);
        } else {
            panic!("Expected JupyterComponent, got {:?}", file_block.component_attr);
        }
    }

    /// Without source_path set, resolve_notebook still works but data_tables stays None.
    #[test]
    fn test_resolve_notebook_without_source_path_leaves_data_tables_none() {
        let stem = "no_path_nb";
        let cell_num = 1usize;
        let file_name = format!("{stem}.ipynb");
        let cmd = format!("/custom_component:jupyter:({file_name}:{cell_num})/");

        let mut cache: BTreeMap<String, ContentBlock> = BTreeMap::new();
        cache.insert("cmd".to_string(), code_text_block("cmd", 0, &cmd));
        cache.insert("file".to_string(), file_block("file", 1, &file_name));

        let mut page = Page {
            cache_contents: cache,
            ..Default::default()
        };

        // source_path is None — no sidecar will be attempted.
        let nb = minimal_notebook_root(stem, cell_num + 1, None);
        page.resolve_notebook(&vec![nb]);

        let file_block = page.cache_contents.get("file").expect("file block must remain");
        if let ComponentAttrType::JupyterComponent(attr) = &file_block.component_attr {
            assert!(
                attr.data_tables.is_none(),
                "data_tables must be None when source_path is absent"
            );
        } else {
            panic!("Expected JupyterComponent, got {:?}", file_block.component_attr);
        }
    }

}
