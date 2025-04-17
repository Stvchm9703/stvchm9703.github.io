use std::collections::BTreeMap;

use crate::anytype::object::AnytypeObject;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub description: String,
    pub tag_type: String,
    pub styles: Vec<String>,
}

impl Tag {
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
}
