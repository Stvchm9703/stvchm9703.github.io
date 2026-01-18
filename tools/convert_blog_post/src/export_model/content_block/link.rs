use serde::{Deserialize, Serialize};

use crate::{
    export_model::trait_impl::FromBlockContent,
    proto::anytype::model::mod_Block::mod_Content::{Link as RawLink, mod_Link},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkComponentAttr {
    pub title: String,
    pub description: String,
    pub url: String,
    pub target_block_id: String,
    pub icon_size: LinkIconSize,
    pub card_style: LinkCardStyle,
}

pub type LinkIconSize = mod_Link::IconSize;
pub type LinkCardStyle = mod_Link::CardStyle;

impl<'life> FromBlockContent<RawLink<'life>> for LinkComponentAttr {
    fn from_block_content(raw: &RawLink<'life>) -> Result<LinkComponentAttr, anyhow::Error> {
        let tmp = LinkComponentAttr {
            target_block_id: raw.targetBlockId.to_string(),
            icon_size: raw.iconSize.to_owned(),
            card_style: raw.cardStyle.to_owned(),
            ..LinkComponentAttr::default()
        };
        return Ok(tmp);
    }
}
