use serde::{Deserialize, Serialize};

use super::{ComponentAttrType, ContentBlock};
// use super::mark;
use super::{super::common::is_release, mark::Mark};
use crate::export_model::trait_impl::FromRaw;
// use crate::proto::anytype::model;

use crate::{
    export_model::trait_impl::FromBlockContent,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marks: Option<Vec<Mark>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub is_container: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TextItem>>,
}

impl<'life> FromBlockContent<RawText<'life>> for TextComponentAttr {
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

impl ContentBlock {
    pub fn is_level_text(&self) -> bool {
        match &self.component_attr {
            ComponentAttrType::Text(t) => {
                t.style == TextStyle::Marked || t.style == TextStyle::Numbered
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(untagged)]
#[serde(tag = "_text_item_type")]
pub enum TextItem {
    Block(TextComponentAttr),
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
    // pub fn to_subitem()
    pub fn to_subitem(blk: &ContentBlock) -> TextItem {
        if let ComponentAttrType::Text(t) = &blk.component_attr {
            return match t.style {
                TextStyle::Marked | TextStyle::Numbered | TextStyle::Toggle => {
                    Self::to_level_text(blk)
                }
                _ => Self::to_endpoint(blk),
            };
        }

        return Self::to_endpoint(blk);
    }

    pub fn create_container(subitems: &Vec<TextItem>) -> TextItem {
        let mut cloned: TextComponentAttr = TextComponentAttr::default();
        if let Some(first_item) = subitems.first() {
            // cloned = first_item.clone();
            if let TextItem::LevelText(t) = first_item {
                // cloned = t.clone();
                cloned.id = t.id.to_owned();
                cloned.style = t.style.clone();
                cloned.order = t.order.to_owned();
                // cloned.children_ids = t.children_ids.to_owned();
            }
        }
        cloned.items = Some(subitems.clone());

        return TextItem::Block(cloned);
    }

    fn to_level_text(blk: &ContentBlock) -> TextItem {
        if let ComponentAttrType::Text(t) = &blk.component_attr {
            return match t.style {
                TextStyle::Marked | TextStyle::Numbered | TextStyle::Toggle => {
                    let mut tmp = t.clone();
                    tmp.id = blk.id.to_owned();
                    tmp.order = blk.order.to_owned();
                    if blk.children_ids.as_ref().is_some_and(|f| !f.is_empty()) {
                        tmp.children_ids = blk.children_ids.to_owned();
                    }
                    if t.items.as_ref().is_some_and(|f| !f.is_empty()) {
                        tmp.items = t.items.to_owned();
                    }
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

    pub fn is_level_text(&self) -> bool {
        match self {
            TextItem::LevelText(_) => true,
            _ => false,
        }
    }
}
