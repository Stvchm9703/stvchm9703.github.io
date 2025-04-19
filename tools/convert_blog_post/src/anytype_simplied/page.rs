use std::{cell::Ref, collections::BTreeMap};

use super::{
    attributes::AttributeMap,
    collection::{Collection, CollectionId},
    content_block::ContentBlock,
};
use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{anytype::SnapshotWithType, trait_impl::to_val_string},
};

use anyhow::Error;
use serde::{Deserialize, Serialize};

pub type PageId = String;
pub type PageMap = BTreeMap<String, Box<Page>>;

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Page {
    pub id: PageId,
    pub title: String,
    // pub name: String,
    pub description: String,
    pub collection_id: CollectionId,
    pub attributes: AttributeMap,
    pub tags: Vec<String>,
    pub styles: AttributeMap,

    pub contents: BTreeMap<String, ContentBlock>,
    pub reformed_contents: BTreeMap<String, ContentBlock>,
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
        // let data = raw.snapshot.data.details.clone();
        let ref data_map = raw
            .snapshot
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .details
            .as_ref()
            .unwrap()
            .fields;

        if let Some(id) = data_map.get("id") {
            tmp.id = to_val_string(&id)?;
        }
        if let Some(name) = data_map.get("name") {
            tmp.title = to_val_string(&name)?;
        }
        if let Some(description) = data_map.get("description") {
            tmp.description = to_val_string(&description)?;
        }

        // let mut articale_id_list: Vec<String> = vec![];
        // if let Some(collections) = raw.snapshot.data.collections.clone() {
        //     articale_id_list = collections.objects.unwrap_or_default();
        // }
        Ok(tmp)
    }
}
