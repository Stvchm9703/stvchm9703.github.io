pub mod attr;
pub mod ext;
pub mod layout_related;
pub mod meta;
pub mod notebook_related;
pub mod util;
use super::{
    common::{
        AttributeMap,
        // DEFAULT_TAG,
        get_field_value,
        get_shorten_id,
        get_snapshot_shorthanded,
        is_release,
        path_resolver,
    },
    content_block::{
        ComponentAttrType, ContentBlock,
        layout::{LayoutComponentAttr, LayoutItem},
        text::{TextComponentAttr, TextStyle},
    },
    // external_link, file_object,
    trait_impl::{FromBlock, FromRaw, FromSnapshotList},
};
use crate::{
    export_model::{
        common::{GLOBAL_RELATION_NAMEMAP, GLOBAL_RELATION_NAMEMAP_STR},
        // content_block::text::TextItem,
        // content_block::text::TextItemComponentAttr,
    },
    proto::anytype::SnapshotWithType,
};

// use layout_related::*;

use std::{
    // borrow::{Borrow, BorrowMut},
    // borrow::Borrow,
    collections::BTreeMap,
};

use anyhow::{Error, anyhow};
use attr::get_page_details;
use convert_blog_post_marco::set_field_value;
use ext::PageExternalLink;
// use lo_::intersection;
use meta::PageMeta;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub title: String,
    // pub description: String,
    pub snippet: String,
    pub collection_id: String,
    pub workspace_id: String,
    // pub attributes: PageAttributes,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_attributes: AttributeMap,
    pub creator: String,
    pub publish_date: i64,
    pub tags: Vec<PageExternalLink>,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_tag_list: Vec<String>,
    pub styles: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_series: Option<Vec<String>>,
    pub serie: Option<PageExternalLink>,
    // page other relation / attr
    pub meta: PageMeta,
    pub table_of_contents: Vec<PageExternalLink>,
    pub related_chapters: Vec<PageExternalLink>,
    pub related_articles: Vec<PageExternalLink>,
    // content-block
    #[serde(skip)]
    pub cache_contents: BTreeMap<String, ContentBlock>,
    #[serde(skip_serializing_if = "is_release")]
    pub check_block_order: Vec<String>,

    pub contents: Vec<ContentBlock>,
}

impl<'a> FromRaw<SnapshotWithType<'a>> for Page {
    fn from_raw(input: &SnapshotWithType) -> Result<Page, anyhow::Error> {
        let mut tmp = Page::default();
        let instance = get_snapshot_shorthanded(input);
        if let Err(e) = instance {
            return Err(anyhow!("fail field map : {:?}", e));
        }
        let (data_wrap, field_map) = instance.unwrap();
        tmp.raw_attributes = field_map.clone();
        // let field_map_ref = field_map;
        let field_try_map = get_page_details(&data_wrap);
        if let Err(e) = field_try_map {
            // eprintln!("{:?}", e);
            return Err(anyhow!("fail page-attributes : {:?}", e));
        }
        let field_attr = field_try_map.unwrap();

        // if let Ok(id) = get_field_value::<String>(field_map_ref, "id") {
        tmp.id = field_attr.id.to_owned();
        tmp.sid = get_shorten_id(&field_attr.id);
        // };
        // if let Ok(title) = get_field_value::<String>(field_map_ref, "name") {
        tmp.title = field_attr.name.to_owned();
        // }
        // tmp.attributes = field_attr.to_owned();
        // if let Ok(backlink) =  get_field_value::<Vec<String>>>(field_map_ref, "backlinks"){
        if field_attr.backlinks.len() > 0 {
            tmp.collection_id = field_attr.backlinks.into_iter().last().unwrap().to_owned();
        }

        // tmp.workspace_id = getFieldValue(detailMap, "spaceId") ?? "";
        tmp.workspace_id = field_attr.space_id;

        // tmp.creator = getFieldValue(detailMap, "creator") ?? "";
        tmp.creator = field_attr.creator;
        // tmp.snippet = getFieldValue(detailMap, "snippet") ?? "";
        tmp.snippet = field_attr.snippet;
        // tmp.raw_tag_list = getFieldValue(detailMap, "tag") ?? [];
        if let Some(tag) = field_attr.tag {
            tmp.raw_tag_list = tag;
        }
        // tmp.publish_date = getFieldValue<number>(detailMap, "publish date");
        if let Ok(publish_date) = get_field_value::<i64>(&field_map, "publish date") {
            tmp.publish_date = publish_date.to_owned();
        }

        for (idx, block) in data_wrap.blocks.iter().enumerate() {
            let block_tmp = ContentBlock::from_block_with_idx(block, idx);
            if let Ok(blk) = block_tmp {
                tmp.cache_contents.insert(blk.id.to_owned(), blk.clone());
            } else {
                println!("error in page-> content-block : {:?}", block_tmp.err());
            }
        }

        return Ok(tmp);
    }
}

impl FromSnapshotList for Page {
    fn from_snapshot_list(raw: Vec<SnapshotWithType>) -> Result<Vec<Page>, anyhow::Error> {
        let mut tmp_ls: Vec<Page> = vec![];

        for y in raw.into_iter() {
            let tmp = Page::from_raw(&y);
            if let Err(e) = tmp {
                return Err(anyhow!("list init error : {:?} , {:?}", e, &y));
            }
            tmp_ls.push(tmp.unwrap());
        }

        return Ok(tmp_ls);
    }
}

impl Page {
    pub fn recheck_fields(&mut self) {
        let mut publish_date: f64 = 0.0;
        set_field_value!(publish_date, self.raw_attributes, "publish date");
        self.publish_date = publish_date as i64;
        let mut series: Vec<String> = Vec::<String>::new();
        set_field_value!(series, self.raw_attributes, "Series");
        self.raw_series = Some(series.to_owned());
        if let Some(rel_key) = GLOBAL_RELATION_NAMEMAP_STR.lock().unwrap().get("Series") {
            if let Some(serie_id) = series.first() {
                if let Some(tag) = GLOBAL_RELATION_NAMEMAP.lock().unwrap().get(rel_key) {
                    self.serie = tag.get_option_page_link(serie_id);
                    if let Some(serie_field) = self.serie.as_mut() {
                        serie_field.url = serie_field.url.replace("/tags/", "/series/");
                    }
                }
            }
        }

        // GLOBAL_RELATION_NAMEMAP.

        // println!("GLOBAL_RELATION_IDMAP {:?}", GLOBAL_RELATION_IDMAP);
        // println!("GLOBAL_RELATION_NAMEMAP {:?}", GLOBAL_RELATION_NAMEMAP);
        // println!( "GLOBAL_RELATION_NAMEMAP_STR {:?}", GLOBAL_RELATION_NAMEMAP_STR );
    }

    pub fn resolve_content_block(&mut self) {
        let root_block = self.cache_contents.get(&self.id);
        if root_block.is_none() {
            return;
        }
        let check_id = self.id.to_owned();
        // println!("page-id: {:?}", check_id);
        let mut order_list_ptr = vec![];
        self.resolve_children_ids_order(&check_id, &mut order_list_ptr);
        self.resolve_nest_children(&order_list_ptr);
        // self.transform_text_list(current_block, text_sebering, &current_layer);

        return;
    }
}
