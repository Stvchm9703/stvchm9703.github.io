
use crate::anytype::object::AnytypeObject;
use crate::anytype_proto::trait_impl::to_val_string;
use crate::anytype_proto::anytype::SnapshotWithType;
use anyhow::Error;
use std::collections::BTreeMap;

pub type UserId = String;
pub type UserMap = BTreeMap<UserId, User>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub description: String,
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
