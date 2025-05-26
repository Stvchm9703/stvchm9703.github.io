use super::Page;
use crate::{
    export_model::content_block::{
        ComponentAttrType, ContentBlock,
        file::FileComponentAttr,
        jupyter::JupyterComponentAttr,
        text::{TextComponentAttr, TextStyle},
    },
    jupyter_notbook::{self, model::JupyterNotebookRoot},
};

use regex::Regex;
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
        let mut to_update: Vec<ContentBlock> = vec![];

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
