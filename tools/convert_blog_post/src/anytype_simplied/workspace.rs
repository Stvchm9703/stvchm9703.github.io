use std::collections::BTreeMap;

use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{
        anytype::SnapshotWithType,
        trait_impl::{get_field_value, to_val_map},
    },
};

use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::{
    attributes::AttributeMap,
    collection::CollectionMap,
    page::PageMap,
    user::{User, UserMap},
};

pub type WorkspaceId = String;
pub type WorkspaceMap = BTreeMap<WorkspaceId, Workspace>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub description: String,
    pub projects: PageMap,
    pub collections: CollectionMap,
    pub users: UserMap,
    // pub tags: BTreeMap<&'static str, Tag>,
    #[serde(skip_serializing)]
    pub attributes: AttributeMap,
    // pub pages: HashMap<String, Page>,
    // pub relations: HashMap<String, Relation>,
}

impl Workspace {
    #[deprecated]
    pub fn from_anytype(raw: &AnytypeObject) -> Self {
        let data = raw.snapshot.data.details.clone();

        Self {
            id: data.id.clone(),
            name: data.name.unwrap_or_default().clone(),
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

    pub fn add_user(&mut self, user: &User) {
        self.users.insert(user.id.clone(), user.clone());
    }
}
