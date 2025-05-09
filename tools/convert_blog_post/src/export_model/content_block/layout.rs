use super::{super::common::is_release, ComponentStyle};
use crate::{
    export_model::trait_impl::{FromBlock, FromBlockContent},
    proto::anytype::model::{
        Block as RawBlock,
        mod_Block::mod_Content::{Layout as RawLayout, mod_Layout},
    },
};
use serde::{Deserialize, Serialize};

use super::ContentBlock;

pub type LayoutStyle = mod_Layout::Style;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutComponentAttr {
    #[serde(skip_serializing_if = "is_release")]
    pub id: String,
    pub order: usize,
    pub is_header: Option<bool>,
    pub style: ComponentStyle,
    pub layout_style: LayoutStyle,
    pub items: Vec<ContentBlock>,
}

impl FromBlock<LayoutComponentAttr> for LayoutComponentAttr {
    fn from_block(raw: &RawBlock) -> Result<LayoutComponentAttr, anyhow::Error> {
        let tmp = LayoutComponentAttr {
            id: raw.id.to_string(),
            style: ComponentStyle::from_block(raw)?,
            ..LayoutComponentAttr::default()
        };

        return Ok(tmp);
    }
    fn from_block_with_idx(
        raw: &RawBlock,
        idx: usize,
    ) -> Result<LayoutComponentAttr, anyhow::Error> {
        let mut tmp = LayoutComponentAttr::from_block(raw)?;
        tmp.order = idx;
        return Ok(tmp);
    }
}

impl FromBlockContent<RawLayout, LayoutComponentAttr> for LayoutComponentAttr {
    fn from_block_content(raw: &RawLayout) -> Result<LayoutComponentAttr, anyhow::Error> {
        // let raw_clone = raw.clone();
        let tmp = LayoutComponentAttr {
            layout_style: raw.style.to_owned(),
            ..LayoutComponentAttr::default()
        };

        return Ok(tmp);
    }
}
