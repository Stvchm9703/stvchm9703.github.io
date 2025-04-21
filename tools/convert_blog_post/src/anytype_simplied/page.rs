use std::collections::BTreeMap;

use super::{
    attributes::AttributeMap,
    collection::CollectionId,
    content_block::{ContentBlock, ContentBlockList, ContentBlockMap},
    tag::{Tag, TagList},
    user::UserId,
    workspace::WorkspaceId,
};
use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{
        anytype::SnapshotWithType,
        anytype_model::Block as PbBlock,
        trait_impl::{get_field_value, get_field_value_list, to_val_map},
    },
};

use anyhow::{Error, Ok};
use serde::{Deserialize, Serialize};

pub type PageId = String;
pub type PageMap = BTreeMap<String, Page>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Page {
    pub id: PageId,
    pub title: String,
    // pub name: String,
    pub description: String,
    pub snippet: String,
    pub collection_id: CollectionId,

    #[serde(skip_serializing)]
    pub attributes: AttributeMap,
    #[serde(skip_serializing)]
    pub workspace_id: WorkspaceId,

    pub creator: UserId,
    pub publish_date: String,

    pub tags: TagList,
    #[serde(skip_serializing)]
    raw_tag_list: Vec<String>,

    pub styles: AttributeMap,
    pub serie: String,
    pub contents: ContentBlockMap,
    pub reformed_contents: ContentBlockList,

    #[serde(skip_serializing)]
    pub raw_contents: Vec<serde_json::Value>,
}

impl Page {
    #[deprecated]
    pub fn from_anytype(raw: &AnytypeObject) -> Self {
        let data = raw.snapshot.data.details.clone();

        // let mut articale_id_list: Vec<String> = vec![];
        // if let Some(collections) = raw.snapshot.data.collections.clone() {
        //     articale_id_list = collections.objects.unwrap_or_default();
        // }
        Self {
            id: data.id.clone(),
            title: data.name.unwrap_or_default().clone(),
            description: data.description.unwrap_or_default().clone(),
            // cover: data.cover.unwrap_or_default().clone(),
            // styles: data.styles.unwrap_or_default().clone(),
            // articale_id_list: raw.snapshot.data.collections.unwrap_or_default().objects
            // articale_id_list,
            ..Default::default()
        }
    }

    pub fn from_anytype_proto(raw: &SnapshotWithType) -> Result<Self, Error> {
        let mut tmp = Self::default();
        let data = raw.snapshot.as_ref().unwrap().data.as_ref().unwrap();

        let ref detail_map = data.details.as_ref().unwrap();

        // println!("data_map: {:#?}", data.extra_relations);

        tmp.id = get_field_value(detail_map, "id")?;
        tmp.title = get_field_value(detail_map, "name")?;
        // tmp.description = get_field_value(data_map, "description")?;
        tmp.attributes = to_val_map(detail_map)?;
        tmp.collection_id = get_field_value_list(detail_map, "backlinks")
            .unwrap()
            .last()
            .unwrap()
            .clone();
        tmp.workspace_id = get_field_value(detail_map, "spaceId")?;
        tmp.creator = get_field_value(detail_map, "creator")?;
        tmp.snippet = get_field_value(detail_map, "snippet")?;
        // tmp.serie = get_field_value(data_map, "Serie")?;
        // !todo
        tmp.raw_tag_list = get_field_value_list(detail_map, "tag").unwrap();
        // let mut articale_id_list: Vec<String> = vec![];
        // if let Some(collections) = raw.snapshot.data.collections.clone() {
        //     articale_id_list = collections.objects.unwrap_or_default();
        // }
        tmp.resolve_content(&data.blocks)?;

        println!("resolved : {:#?}", tmp);
        Ok(tmp)
    }

    pub fn resolve_relations(&mut self, relations: Vec<Tag>) {
        // self.relations = relations;
    }

    fn resolve_content(&mut self, raw_content: &Vec<PbBlock>) -> Result<(), Error> {
        for (idx, block) in raw_content.iter().enumerate() {
            let cb_resolved = ContentBlock::from_block(&idx, block)?;
            self.contents
                .insert(cb_resolved.id.to_string(), cb_resolved);

            // self.raw_contents.push(serde_json::to_value(block).unwrap());
        }
        Ok(())
        // Err(anyhow!("Not implemented"))
    }
}
