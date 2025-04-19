use std::collections::BTreeMap;

use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{anytype::SnapshotWithType, trait_impl::to_val_string},
};

use anyhow::Error;

use super::{collection::Collection, tag::Tag, user::User};

pub type WorkspaceId = String;
pub type WorkspaceMap = BTreeMap<WorkspaceId, Workspace>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub id: WorkspaceId,
    pub name: String,
    pub description: String,
    pub projects: BTreeMap<&'static str, String>,
    pub collections: BTreeMap<&'static str, Collection>,
    pub users: BTreeMap<&'static str, User>,
    pub tags: BTreeMap<&'static str, Tag>,
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
            .unwrap()
            .fields;
        // let data_map = snapshot.data.as_ref().unwrap().details.unwrap().fields;
        // // prost::Message::

        if let Some(id) = data_map.get("id") {
            tmp.id = to_val_string(&id)?;
        }
        if let Some(name) = data_map.get("name") {
            tmp.name = to_val_string(&name)?;
        }
        if let Some(description) = data_map.get("description") {
            tmp.description = to_val_string(&description)?;
        }

        Ok(tmp)
    }
}
