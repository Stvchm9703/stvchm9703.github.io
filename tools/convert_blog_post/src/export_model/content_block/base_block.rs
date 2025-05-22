// use crate::export_model::common::AttributeMap;

use super::ComponentStyle;

use crate::export_model::{
    common::{AttributeMap, header_id_resolver, is_release},
    page::ext::PageExternalLink,
    trait_impl::{FromBlock, FromBlockContent},
};
use serde::{Deserialize, Serialize};
// use serde_json::json;
// use text::{TextComponentAttr, TextStyle};

// use anyhow::{Error, anyhow};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct BaseContentBlock<AttrContent> {
    pub id: String,

    pub order: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<AttributeMap>,
    // #[serde(skip)]
    pub children_ids: Option<Vec<String>>,
    pub style: ComponentStyle,
    pub contents: AttrContent,
}
