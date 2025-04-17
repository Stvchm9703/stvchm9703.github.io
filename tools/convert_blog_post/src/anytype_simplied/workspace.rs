use std::collections::BTreeMap;

use crate::anytype::object::AnytypeObject;

use super::{collection::Collection, tag::Tag, user::User};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub collections: BTreeMap<String, Collection>,
    pub users: BTreeMap<String, User>,
    pub tags: BTreeMap<String, Tag>,
    // pub pages: HashMap<String, Page>,
    // pub relations: HashMap<String, Relation>,
}

impl Workspace {
    pub fn from_anytype(raw: &AnytypeObject) -> Self {
        let data = raw.snapshot.data.details.clone();

        Self {
            id: data.id.clone(),
            name: data.name.unwrap_or_default().clone(),
            ..Default::default()
        }
    }
}
