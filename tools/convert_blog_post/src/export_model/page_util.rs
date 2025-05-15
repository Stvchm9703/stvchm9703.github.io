use crate::jupyter_notbook::model::JupyterNotebookRoot;

use super::{
    common::DEFAULT_TAG, external_link::ExternalBookmarkLink, file_object::FileObject, page::Page,
    tag::Tag,
};

pub fn resolve_page_external<'a>(
    page: &'a mut Page,
    file_list: &'a Vec<FileObject>,
    bookmark_list: &'a Vec<ExternalBookmarkLink>,
    tag_list: &'a Vec<Tag>,
    nb_list: &'a Vec<JupyterNotebookRoot>,
) {
    //
    page.recheck_fields();
    // resolve the external data inject
    page.resolve_external_link(bookmark_list);
    page.resolve_file_link(file_list);

    if let Ok(mut t) = DEFAULT_TAG.lock() {
        if t.id.is_empty() {
            if let Some(default_tag) = tag_list.into_iter().find(|p| p.name == "Tag") {
                *t = default_tag.to_owned();
            }
        }
        drop(t);
    }

    page.resolve_tag_link();
    page.resolve_toc_component();

    // restruct
    page.resolve_content_block();
}

pub fn resolve_page_related_posts<'a>(page: &'a mut Page, page_list: &'a Vec<Page>) {
    page.resolve_related_articles(page_list);
    page.resolve_related_chapters(page_list);
}
