use anyhow::{Error, anyhow};
use serde::{Deserialize, Serialize};

use super::{ComponentAttrType, ContentBlock};
// use super::mark;
use super::{super::common::is_release, ComponentStyle, mark::Mark};
use crate::export_model::trait_impl::FromRaw;
// use crate::proto::anytype::model;

use crate::{
    export_model::trait_impl::{AddFromExternalFile, FromBlockContent},
    proto::anytype::model::mod_Block::mod_Content::{Text as RawText, mod_Text},
};

pub type TextStyle = mod_Text::Style;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponentAttr {
    #[serde(skip_serializing_if = "is_release")]
    pub id: String,
    pub order: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children_ids: Option<Vec<String>>,

    pub text: String,
    pub style: TextStyle,
    pub marks: Option<Vec<Mark>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub is_container: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TextItem>>,
}

impl<'life> FromBlockContent<RawText<'life>, TextComponentAttr> for TextComponentAttr {
    fn from_block_content(raw: &RawText<'life>) -> Result<TextComponentAttr, anyhow::Error> {
        let marks: Vec<Mark> = raw
            .marks
            .as_ref()
            .unwrap()
            .marks
            .iter()
            .map(Mark::from_raw)
            .filter(|p| p.is_ok())
            .map(|p| p.ok().unwrap())
            .collect();

        let tmp = TextComponentAttr {
            text: raw.text.to_string(),
            style: raw.style.to_owned(),
            marks: Some(marks),
            // url : raw_clone.
            ..TextComponentAttr::default()
        };
        return Ok(tmp);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextItem {
    LevelText(TextComponentAttr),
    Other(ContentBlock),
    Unknown,
}
impl Default for TextItem {
    fn default() -> Self {
        Self::Unknown
    }
}

impl TextItem {
    pub fn from_content_block(blk: &ContentBlock) -> TextItem {
        if let ComponentAttrType::Text(t) = &blk.component_attr {
            return match t.style {
                TextStyle::Marked | TextStyle::Numbered | TextStyle::Toggle => {
                    Self::to_level_text(blk)
                }
                _ => Self::to_endpoint(blk),
                // mod_Text::Style::Paragraph => todo!(),
                // mod_Text::Style::Header1 => todo!(),
                // mod_Text::Style::Header2 => todo!(),
                // mod_Text::Style::Header3 => todo!(),
                // mod_Text::Style::Header4 => todo!(),
                // mod_Text::Style::Quote => todo!(),
                // mod_Text::Style::Code => todo!(),
                // mod_Text::Style::Title => todo!(),
                // mod_Text::Style::Checkbox => todo!(),

                // mod_Text::Style::Description => todo!(),
                // mod_Text::Style::Callout => todo!(),
            };
        }

        return Self::Unknown;
    }
    fn to_level_text(blk: &ContentBlock) -> TextItem {
        if let ComponentAttrType::Text(t) = &blk.component_attr {
            return match t.style {
                TextStyle::Marked | TextStyle::Numbered | TextStyle::Toggle => {
                    let mut tmp = t.clone();
                    tmp.id = blk.id.to_owned();
                    tmp.children_ids = blk.children_ids.to_owned();
                    tmp.order = blk.order.to_owned();
                    return TextItem::LevelText(tmp);
                }
                _ => Self::to_endpoint(blk),
            };
        }
        return TextItem::Unknown;
    }

    fn to_endpoint(blk: &ContentBlock) -> TextItem {
        TextItem::Other(blk.to_owned())
    }
}
