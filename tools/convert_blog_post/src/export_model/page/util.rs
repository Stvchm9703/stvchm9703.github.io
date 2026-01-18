use crate::{
    export_model::{
        common::DEFAULT_TAG,
        content_block::{ComponentAttrType, text::TextStyle},
        external_link::{self, ExternalBookmarkLink},
        file_object::{self, FileObject},
        tag::Tag,
        // trait_impl::{FromBlock, FromRaw, FromSnapshotList},
    },
    jupyter_notebook::model::JupyterNotebookRoot,
};

use std::{
    borrow::BorrowMut,
    // collections::BTreeMap,
};

use super::{Page, ext::ToPageMiniExternalLink};
// use anyhow::{Error, anyhow};
// use super::attr::get_page_details;
// use convert_blog_post_marco::set_field_value;
use super::ext::{PageExternalLink, ToPageExternalLink};
use super::meta::PageMetaOpenGraphObj;
use lo_::intersection;
// use serde::{Deserialize, Serialize};
// use smallvec::{SmallVec, smallvec};

// use super::{
//     common::DEFAULT_TAG, external_link::ExternalBookmarkLink, file_object::FileObject, page::Page,
//     tag::Tag,
// };

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
    page.resolve_notebook(nb_list);

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

impl Page {
    /// external layer
    pub(crate) fn resolve_external_link(
        &mut self,
        external_link: &Vec<external_link::ExternalBookmarkLink>,
    ) {
        let list = self.cache_contents.values_mut();
        for item in list {
            if let ComponentAttrType::Link(inside) = item.component_attr.borrow_mut() {
                let target_link = external_link
                    .iter()
                    .find(|p| p.id == inside.target_block_id);
                if let Some(tar) = target_link {
                    if tar.title.is_empty() == false {
                        inside.title = tar.title.to_string();
                    }
                    inside.url = tar.url.to_string();
                }
            }
        }
    }

    pub(crate) fn resolve_file_link(&mut self, file_link: &Vec<file_object::FileObject>) {
        let list = self.cache_contents.values_mut();
        let mut file_list = vec![];
        for item in list {
            if let ComponentAttrType::File(inside) = item.component_attr.borrow_mut() {
                let target_link = file_link.iter().find(|p| p.id == inside.target_object_id);
                if let Some(tar) = target_link {
                    inside.file_url = tar.file_url.to_string();
                    file_list.push(tar);
                }
            }
        }
        self.add_meta_data(&file_list);
    }

    fn add_meta_data(&mut self, file_list: &Vec<&file_object::FileObject>) {
        let images = file_list
            .into_iter()
            .filter(|p| p.file_type == "images")
            .map(|p| p.to_page_meta_og())
            .collect::<Vec<PageMetaOpenGraphObj>>();

        let videos = file_list
            .into_iter()
            .filter(|p| p.file_type == "videos")
            .map(|p| p.to_page_meta_og())
            .collect::<Vec<PageMetaOpenGraphObj>>();

        let audio = file_list
            .into_iter()
            .filter(|p| p.file_type == "audios")
            .map(|p| p.to_page_meta_og())
            .collect::<Vec<PageMetaOpenGraphObj>>();

        if images.len() > 0 {
            self.meta.images = Some(images);
        }
        if videos.len() > 0 {
            self.meta.videos = Some(videos);
        }
        if audio.len() > 0 {
            self.meta.audio = Some(audio);
        }
    }

    pub(crate) fn resolve_toc_component(&mut self) {
        let mut toc_blocks: Vec<_> = self
            .cache_contents
            .values()
            .filter(|p| {
                if let ComponentAttrType::Text(attr) = &p.component_attr {
                    return attr.style == TextStyle::Header1
                        || attr.style == TextStyle::Header2
                        || attr.style == TextStyle::Header3
                        || attr.style == TextStyle::Header4;
                }
                return false;
            })
            .collect();

        // Sort by the block's order field
        toc_blocks.sort_by_key(|b| b.order);

        self.table_of_contents = toc_blocks
            .into_iter()
            .filter_map(|p| p.to_toc_link())
            .collect();
    }

    pub(crate) fn resolve_tag_link(&mut self) {
        // println!("tag-list :{:?}", tag_list);
        // self.raw_tag_list.iter().map(f)
        let included_tag = DEFAULT_TAG
            .lock()
            .unwrap()
            .options
            .iter()
            .filter(|p| self.raw_tag_list.contains(&p.id))
            .map(|f| f.to_page_ext_link())
            .collect::<Vec<_>>();

        self.tags = included_tag;
    }

    pub(crate) fn resolve_related_chapters(&mut self, page_list: &Vec<Page>) {
        if self.serie.is_none() {
            return;
        }
        let serie_tag = self.serie.as_ref().unwrap();
        let mut d = page_list
            .iter()
            .filter(|p| {
                if p.serie.is_none() {
                    return false;
                }
                return p.serie.as_ref().unwrap().id == serie_tag.id;
            })
            .collect::<Vec<_>>();
        d.sort_by(|a, b| b.publish_date.cmp(&a.publish_date));

        self.related_chapters = d
            .iter()
            .take(5)
            .map(|p| p.to_page_ext_link())
            .collect::<Vec<_>>();
    }

    pub(crate) fn resolve_related_articles(&mut self, page_list: &Vec<Page>) {
        if self.raw_tag_list.len() == 0 {
            return;
        }
        let mut d = page_list
            .iter()
            .filter(|p| {
                let ttt = intersection(&self.raw_tag_list, &p.raw_tag_list);
                return ttt.len() > 0;
            })
            .collect::<Vec<_>>();
        d.sort_by(|a, b| b.publish_date.cmp(&a.publish_date));

        self.related_articles = d
            .iter()
            .take(5)
            .map(|p| p.to_page_ext_link())
            .collect::<Vec<_>>();
    }

    pub(crate) fn to_post_card_link(&self) -> PageExternalLink {
        let mut tmp = self.to_page_ext_link();
        tmp.snippet = Some(self.snippet.to_string());
        tmp.tags = Some(
            self.tags
                .to_owned()
                .iter()
                .map(|f| f.to_page_mini_external_link())
                .collect(),
        );
        if let Some(d) = self.serie.as_ref() {
            let mut link = d.to_page_mini_external_link();
            link.url = link.url.replace("/tags/", "/series/");
            tmp.serie = Some(link);
        }
        return tmp;
    }
}
