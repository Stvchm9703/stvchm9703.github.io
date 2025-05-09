use super::{
    common::{
        get_field_value, get_shorten_id, get_snapshot_data, get_snapshot_detail_field,
        get_snapshot_shorthanded,
    },
    content_block::ContentBlock,
    trait_impl::{FromRaw, FromSnapshotList},
};
use crate::proto::anytype::mod_Change::Snapshot;
use crate::proto::anytype::{SnapshotWithType, model::SmartBlockSnapshotBase};

use std::collections::BTreeMap;

use anyhow::{Error, anyhow};
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
    pub attributes: PageAttributes,
    pub creator: String,
    pub publish_date: i64,
    pub tags: Vec<PageExternalLink>,
    #[serde(skip)]
    pub raw_tag_list: Vec<String>,
    pub styles: Option<serde_json::Value>,
    pub serie: Option<PageExternalLink>,
    // page other relation / attr
    pub meta: PageMeta,
    pub table_of_contents: Vec<PageExternalLink>,
    pub related_chapters: Vec<PageExternalLink>,
    pub related_posts: Vec<PageExternalLink>,
    pub related_articles: Vec<PageExternalLink>,
    // content-block
    #[serde(skip)]
    pub cache_contents: BTreeMap<String, ContentBlock>,
    pub contents: Vec<ContentBlock>,
}

impl<'a> FromRaw<SnapshotWithType<'a>, Page> for Page {
    fn from_raw(input: &SnapshotWithType) -> Result<Page, anyhow::Error> {
        let mut tmp = Page::default();
        let instance = get_snapshot_shorthanded(input);
        if let Err(e) = instance {
            return Err(anyhow!("fail field map : {:?}", e));
        }
        let (data_wrap, field_map) = instance.unwrap();
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
        tmp.attributes = field_attr.to_owned();
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

        return Ok(tmp);
    }
}

impl FromSnapshotList<Page> for Page {
    fn from_snapshot_list(raw: Vec<SnapshotWithType>) -> Result<Vec<Page>, anyhow::Error> {
        let mut tmp_ls: Vec<Page> = vec![];

        for y in raw.into_iter() {
            // return Page::from_raw(&raw).unwrap();
            let tmp = Page::from_raw(&y);
            if let Err(e) = tmp {
                return Err(anyhow!("list init error : {:?} , {:?}", e, &y));
            }
            tmp_ls.push(tmp.unwrap());
        }

        return Ok(tmp_ls);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageAttributes {
    pub featured_relations: Option<Vec<serde_json::Value>>,
    pub id: String,
    pub snippet: String,
    pub resolved_layout: i64,
    pub space_id: String,
    // #[serde(rename = "67fd2398e212b5271c38aad2")]
    // pub the_67_fd2398_e212_b5271_c38_aad2: Vec<Option<serde_json::Value>>,
    pub last_modified_by: String,
    pub mentions: Vec<String>,
    pub name: String,
    pub sync_date: i64,
    // #[serde(rename = "67ff5cd5e212b569d0ab04d2")]
    // pub the_67_ff5_cd5_e212_b569_d0_ab04_d2: i64,
    pub sync_error: i64,
    pub created_date: i64,
    pub sync_status: i64,
    pub links: Vec<String>,
    pub backlinks: Vec<String>,
    #[serde(rename = "type")]
    pub attributes_type: String,
    pub last_modified_date: i64,
    pub last_opened_date: i64,
    pub creator: String,
    pub tag: Option<Vec<String>>,
    pub description: Option<String>,
    pub source_object: String,
    pub layout: i64,
}

impl PageAttributes {
    fn from_cast(input: &PageAttributesFromCast) -> Self {
        let tmp = PageAttributes {
            featured_relations: input.featured_relations.clone(),
            id: input.id.clone(),
            snippet: input.snippet.clone(),
            resolved_layout: input.resolved_layout.clone() as i64,
            space_id: input.space_id.clone(),
            last_modified_by: input.last_modified_by.clone(),
            mentions: input.mentions.clone(),
            name: input.name.clone(),
            sync_date: input.sync_date.clone() as i64,
            sync_error: input.sync_error.clone() as i64,
            created_date: input.created_date.clone() as i64,
            sync_status: input.sync_status.clone() as i64,
            links: input.links.clone(),
            backlinks: input.backlinks.clone(),
            attributes_type: input.attributes_type.clone(),
            last_modified_date: input.last_modified_date.clone() as i64,
            last_opened_date: input.last_opened_date.clone() as i64,
            creator: input.creator.clone(),
            tag: input.tag.clone(),
            description: input.description.clone(),
            source_object: input.source_object.clone(),
            layout: input.layout.clone() as i64,
        };

        return tmp;
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageAttributesFromCast {
    pub featured_relations: Option<Vec<serde_json::Value>>,
    pub id: String,
    pub snippet: String,
    pub resolved_layout: f64,
    pub space_id: String,
    // #[serde(rename = "67fd2398e212b5271c38aad2")]
    // pub the_67_fd2398_e212_b5271_c38_aad2: Vec<Option<serde_json::Value>>,
    pub last_modified_by: String,
    pub mentions: Vec<String>,
    pub name: String,
    pub sync_date: f64,
    // #[serde(rename = "67ff5cd5e212b569d0ab04d2")]
    // pub the_67_ff5_cd5_e212_b569_d0_ab04_d2: f64,
    pub sync_error: f64,
    pub created_date: f64,
    pub sync_status: f64,
    pub links: Vec<String>,
    pub backlinks: Vec<String>,
    #[serde(rename = "type")]
    pub attributes_type: String,
    pub last_modified_date: f64,
    pub last_opened_date: f64,
    pub creator: String,
    pub tag: Option<Vec<String>>,
    pub description: Option<String>,

    pub source_object: String,
    pub layout: f64,
}
pub(crate) fn get_page_details<'a>(
    input: &SmartBlockSnapshotBase<'a>,
) -> Result<PageAttributes, Error> {
    if input.details.is_none() {
        return Err(anyhow!("details is none"));
    }
    let detail = input.details.as_ref().unwrap();
    let tmp = serde_json::to_value(detail);
    if tmp.is_err() {
        return Err(anyhow!("parsing error ; to-value"));
    }
    // println!("{:#?}", tmp);
    let result = serde_json::from_value::<PageAttributesFromCast>(tmp.unwrap());
    if result.is_err() {
        // println!("{:#?}", result);
        return Err(anyhow!("parsing error ; from-value"));
    }

    return Ok(PageAttributes::from_cast(result.as_ref().unwrap()));
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageExternalLink {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub component_id: Option<String>,
    pub label: String,
    pub url: String,
    pub level: Option<usize>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
    pub images: Option<Vec<PageMetaOpenGraphObj>>,
    pub videos: Option<Vec<PageMetaOpenGraphObj>>,
    pub audio: Option<Vec<PageMetaOpenGraphObj>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMetaOpenGraphObj {
    pub url: String,
    pub secure_url: Option<String>,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub alt: Option<String>,
}
