// use std::collections::BTreeMap;

// use super::attribute::AttributeMap;
use anyhow::{Error, Ok};
use serde::{Deserialize, Serialize};

use crate::{
    anytype::{
        object::Block as ATBlock,
        trait_impl::{GetBlockComponentStyle, GetLayoutStyle},
    },
    anytype_proto::{
        anytype_model::{Block as PBlock, block},
        trait_impl::to_val_map,
    },
};

use super::component_base::resolve_component_name;

#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ContentBlock {
    pub id: String,
    pub order: usize,
    pub component_name: String,
    pub content: String,
    pub attributes: AttributeMap,
    pub styles: AttributeMap,
    pub raw_content: String,
    pub children_ids: Vec<String>,
    pub children: Vec<ContentBlock>,
}

impl ContentBlock {
    #[deprecated]
    pub fn from_raw_block(idx: &usize, block: &ATBlock) -> Self {
        Self {
            id: block.id.clone(),
            order: *idx,
            component_name: block.get_block_component_style(),
            // content: block.content.clone(),
            // attributes: AttributeMap::from_raw_attributes(&block.attributes),
            // styles: BTreeMap::from_raw_styles(&block.styles),
            // children_ids: block.children_ids.clone(),
            // children: Vec::new(),
            ..Default::default()
        }
    }

    pub fn from_block(idx: &usize, block: &PBlock) -> Result<Self, Error> {
        let mut content_block = Self::default();

        content_block.id = block.id.clone();
        content_block.order = *idx;
        content_block.component_name = resolve_component_name(block.content.as_ref().unwrap());
        // content_block.content = block.content.as_ref().unwrap().
        content_block.attributes = to_val_map(block.fields.as_ref().unwrap())?;

        // content_block.styles = BTreeMap::from_raw_styles(&block.styles);
        content_block.children_ids = block.children_ids.clone();
        // content_block.children = Vec::new();

        content_block.raw_content = serde_json::to_string(block.content.as_ref().unwrap()).unwrap();
        Ok(content_block)
    }
}
