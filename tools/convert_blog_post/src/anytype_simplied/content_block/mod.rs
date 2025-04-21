// use std::collections::BTreeMap;
mod component_base;
mod content_resolver;
use std::collections::BTreeMap;
// use super::attribute::AttributeMap;
use anyhow::{Error, Ok};
use component_base::resolve_component_name;
use serde::{Deserialize, Serialize};

use crate::{
    anytype::{object::Block as ATBlock, trait_impl::GetBlockComponentStyle},
    anytype_proto::{
        anytype_model::Block as PBlock,
        trait_impl::{ToTokenString, to_val_map},
    },
};

use super::attributes::{AttributeMap, get_style::GetStyle};

pub type ContentBlockId = String;
pub type ContentBlockMap = BTreeMap<ContentBlockId, ContentBlock>;
pub type ContentBlockList = Vec<ContentBlock>;
#[derive(Default, Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ContentBlock {
    pub id: ContentBlockId,
    pub order: usize,
    // #[serde(skip_serializing]
    // pub base_component: i32,
    pub base_component: String,
    pub component_name: String,

    pub content: String,
    pub attributes: AttributeMap,
    pub styles: AttributeMap,
    pub raw_content: serde_json::Value,
    pub children_ids: Vec<ContentBlockId>,
    pub children: ContentBlockList,
}

impl ContentBlock {
    #[deprecated]
    pub fn from_raw_block(idx: &usize, block: &ATBlock) -> Self {
        Self {
            id: block.id.clone(),
            order: *idx,
            // base_component: block.get_block_component_style(),
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
        content_block.base_component = block.content.as_ref().unwrap().to_token_string();
        content_block.component_name = resolve_component_name(block.content.as_ref().unwrap());
        // content_block.content = block.content.as_ref().unwrap().
        if let Some(fields) = block.fields.as_ref() {
            content_block.attributes = to_val_map(fields)?;
        }

        content_block.styles = block.get_style();
        content_block.children_ids = block.children_ids.clone();
        // content_block.children = Vec::new();

        content_block.raw_content = serde_json::to_value(block.content.as_ref().unwrap()).unwrap();
        Ok(content_block)
    }
}
