use crate::anytype::object::AnytypeObject;
use crate::anytype_proto::anytype::SnapshotWithType;
use crate::anytype_proto::trait_impl::{get_field_value, to_val_map};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use super::attributes::AttributeMap;

pub type UserId = String;
pub type UserMap = BTreeMap<UserId, User>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub description: String,

    #[serde(skip_serializing)]
    pub attributes: AttributeMap,
}

impl User {
    #[deprecated]
    pub fn from_anytype(raw: &AnytypeObject) -> Self {
        let data = raw.snapshot.data.details.clone();
        User {
            id: data.id.clone(),
            name: data.name.unwrap_or_default().clone(),
            // email: data.details.email.clone(),
            description: data.description.unwrap_or_default().clone(),
            ..Default::default()
        }
    }
    #[deprecated]
    pub fn from_anytype_list(raw_list: &Vec<&AnytypeObject>) -> BTreeMap<String, Self> {
        let mut users: BTreeMap<String, User> = BTreeMap::new();

        for obj in raw_list {
            let data = obj.snapshot.data.details.clone();
            users.insert(
                data.id.clone(),
                User {
                    id: data.id.clone(),
                    name: data.name.unwrap_or_default().clone(),
                    // email: data.details.email.clone(),
                    description: data.description.unwrap_or_default().clone(),
                    ..Default::default()
                },
            );
        }

        users
    }

    pub fn from_anytype_proto(raw: &SnapshotWithType) -> Result<Self, Error> {
        let mut tmp = Self {
            ..Default::default()
        };

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
        tmp.description = get_field_value(data_map, "description")?;
        tmp.attributes = to_val_map(data_map)?;

        Ok(tmp)
    }
}
