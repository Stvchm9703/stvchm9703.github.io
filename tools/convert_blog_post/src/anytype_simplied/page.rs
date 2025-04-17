use std::collections::BTreeMap;

use super::content_block::ContentBlock;
use crate::anytype::object::AnytypeObject;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    // pub name: String,
    pub description: String,
    pub collection_id: String,
    pub contents: BTreeMap<String, ContentBlock>,
    pub reformed_contents: BTreeMap<String, ContentBlock>,
}

impl Page {
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
}
