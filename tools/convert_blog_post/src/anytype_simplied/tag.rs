use std::collections::BTreeMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{
        anytype::SnapshotWithType,
        trait_impl::{get_field_value, to_val_map},
    },
};

use super::attributes::AttributeMap;

pub type TagId = String;
pub type TagList = Vec<Tag>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub description: String,
    pub tag_type: String,
    pub styles: Vec<String>,

    #[serde(skip_serializing)]
    pub attributes: AttributeMap,
}

impl Tag {
    #[deprecated]
    pub fn from_anytype_list(raw_list: &Vec<&AnytypeObject>) -> BTreeMap<String, Self> {
        let mut items: BTreeMap<String, Self> = BTreeMap::new();

        for obj in raw_list {
            let data = obj.snapshot.data.details.clone();
            items.insert(
                data.id.clone(),
                Tag {
                    id: data.id.clone(),
                    name: data.name.unwrap_or_default().clone(),
                    description: data.description.unwrap_or_default().clone(),
                    // tag_type: data
                    ..Default::default()
                },
            );
        }
        items
    }
    #[deprecated]
    pub fn from_anytype(obj: &AnytypeObject) -> Self {
        let data = obj.snapshot.data.details.clone();
        Tag {
            id: data.id.clone(),
            name: data.name.unwrap_or_default().clone(),
            description: data.description.unwrap_or_default().clone(),
            // tag_type: data
            ..Default::default()
        }
    }

    pub fn from_anytype_proto(raw: &SnapshotWithType) -> Result<Self, Error> {
        let mut tmp = Self::default();

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
        // let data_map = snapshot.data.as_ref().unwrap().details.unwrap().fields;
        // // prost::Message::

        tmp.id = get_field_value(data_map, "id")?;
        tmp.name = get_field_value(data_map, "name")?;
        // tmp.description = get_field_value(data_map, "description")?;
        tmp.attributes = to_val_map(data_map)?;

        Ok(tmp)
    }
}
