use std::collections::BTreeMap;

use super::{attribute::AttributeMap, content_block::ContentBlock};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub contents: BTreeMap<String, ContentBlock>,
    pub attributes: AttributeMap,
}
