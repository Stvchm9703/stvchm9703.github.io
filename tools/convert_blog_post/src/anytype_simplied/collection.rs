use std::collections::BTreeMap;

use anyhow::{Error, Ok};
use serde::{Deserialize, Serialize};

use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{
        anytype::SnapshotWithType,
        trait_impl::{get_field_value, get_field_value_list, to_val_map},
    },
};

use super::{
    attributes::AttributeMap,
    page::{Page, PageId, PageMap},
    workspace::WorkspaceId,
};

// use super::article::Article;

pub type CollectionId = String;
pub type CollectionMap = BTreeMap<CollectionId, Box<Collection>>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub id: CollectionId,
    pub name: String,
    pub description: String,
    pub cover: String,
    pub styles: Vec<String>,
    #[serde(skip_serializing)]
    pub attributes: AttributeMap,
    #[serde(skip_serializing)]
    pub workspace_id: WorkspaceId,

    pub articale_id_list: Vec<PageId>,
    pub articles: PageMap,
}

impl Collection {
    #[deprecated]
    pub fn from_anytype(raw: &AnytypeObject) -> Self {
        let data = raw.snapshot.data.details.clone();

        let mut articale_id_list: Vec<String> = vec![];
        if let Some(collections) = raw.snapshot.data.collections.clone() {
            articale_id_list = collections.objects.unwrap_or_default();
        }
        Self {
            id: data.id.clone(),
            name: data.name.unwrap_or_default().clone(),
            description: data.description.unwrap_or_default().clone(),
            // cover: data.cover.unwrap_or_default().clone(),
            // styles: data.styles.unwrap_or_default().clone(),
            // articale_id_list: raw.snapshot.data.collections.unwrap_or_default().objects
            articale_id_list,
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
            .unwrap();

        tmp.id = get_field_value(data_map, "id")?;
        tmp.name = get_field_value(data_map, "name")?;
        tmp.attributes = to_val_map(data_map)?;
        tmp.articale_id_list = get_field_value_list(data_map, "links")?;
        tmp.workspace_id = get_field_value(data_map, "spaceId")?;
        // tmp.description = get_field_value(data_map, "description")?;
        // tmp.serie = get_field_value(data_map, "Serie")?;

        // let mut articale_id_list: Vec<String> = vec![];
        // if let Some(collections) = raw.snapshot.data.collections.clone() {
        //     articale_id_list = collections.objects.unwrap_or_default();
        // }
        Ok(tmp)
    }

    pub fn insert_articles(&mut self, files: &Vec<Page>) -> Result<(), Error> {
        for key in self.articale_id_list.iter() {
            if let Some(file) = files.iter().find(|f| &f.id == key) {
                self.articles.insert(file.id.to_owned(), file.to_owned());
            }
        }
        Ok(())
    }
}
