use std::collections::BTreeMap;

use super::attribute::AttributeMap;
use serde::{Deserialize, Serialize};

use crate::anytype::object::{Block as ATBlock, LayoutStyle, LayoutStyleValue};

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ContentBlock {
    pub id: String,
    pub order: usize,
    pub base_type: String,
    pub content: String,
    pub attributes: AttributeMap,
    pub styles: BTreeMap<String, String>,
    pub children_ids: Vec<String>,
    pub children: Vec<ContentBlock>,
}

impl ContentBlock {
    pub fn from_raw_block(idx: &usize, block: &ATBlock) -> Self {
        Self {
            id: block.id.clone(),
            order: *idx,
            base_type: block.layout.get_style().to_owned(),
            // content: block.content.clone(),
            // attributes: AttributeMap::from_raw_attributes(&block.attributes),
            // styles: BTreeMap::from_raw_styles(&block.styles),
            // children_ids: block.children_ids.clone(),
            // children: Vec::new(),
            ..Default::default()
        }
    }
}
