use crate::anytype::object::AnytypeObject;
use std::collections::BTreeMap;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub description: String,
}

impl User {
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
}
