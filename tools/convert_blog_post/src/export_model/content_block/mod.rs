pub mod bookmark;
pub mod file;
pub mod jupyter;
pub mod layout;
pub mod link;
pub mod mark;
pub mod text;

use bookmark::BookmarkComponentAttr;
use file::FileComponentAttr;
use jupyter::JupyterComponentAttr;
use layout::LayoutComponentAttr;
use link::LinkComponentAttr;
use serde::{Deserialize, Serialize};
use text::TextComponentAttr;

use anyhow::Error;

use super::trait_impl::{FromBlock, FromBlockContent};
use crate::proto::anytype::model;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentBlock {
    pub id: String,
    pub order: usize,
    pub fields: serde_json::Value,
    #[serde(skip)]
    pub children_ids: Option<Vec<String>>,
    pub style: ComponentStyle,
    pub component_attr: ComponentAttrType,
}

impl FromBlock<ContentBlock> for ContentBlock {
    fn from_block(raw: &model::Block) -> Result<ContentBlock, anyhow::Error> {
        let raw_clone = raw.clone();
        let mut fields = serde_json::Value::Null;
        if let Some(f) = raw_clone.fields.as_ref() {
            if let Ok(y) = serde_json::to_value(f) {
                fields = y;
            }
        }

        let children_ids: Vec<String> = raw_clone
            .childrenIds
            .into_iter()
            .map(String::from)
            .collect();

        let tmp = ContentBlock {
            id: raw_clone.id.into_owned(),
            fields,
            children_ids: Some(children_ids),
            style: ComponentStyle::from_block(raw)?,
            ..ContentBlock::default()
        };

        Ok(tmp)
    }

    fn from_block_with_idx(raw: &model::Block, idx: usize) -> Result<ContentBlock, Error> {
        let mut blk = Self::from_block(raw)?;
        blk.order = idx;
        Ok(blk)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "componentType")]
pub enum ComponentAttrType {
    Unknown,
    Bookmark(BookmarkComponentAttr),
    // Div,
    File(FileComponentAttr),
    JupyterComponent(JupyterComponentAttr),
    Layout(LayoutComponentAttr),
    /// not for export
    // LayoutRow(LayoutComponentAttr),
    // LayoutColumn(LayoutComponentAttr),
    Link(LinkComponentAttr),
    // Relation,
    Table(LayoutComponentAttr),
    Text(TextComponentAttr),
}
impl Default for ComponentAttrType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStyle {
    pub background_color: String,
    pub align: AlignStyle,
    pub vertical_align: VerticalAlignStyle,
}

pub type AlignStyle = model::mod_Block::Align;
pub type VerticalAlignStyle = model::mod_Block::VerticalAlign;

impl FromBlock<ComponentStyle> for ComponentStyle {
    fn from_block(raw: &model::Block) -> Result<ComponentStyle, Error> {
        let raw_clone = raw.clone();
        let tmp = ComponentStyle {
            background_color: raw_clone.backgroundColor.into_owned(),
            align: raw_clone.align.to_owned(),
            vertical_align: raw_clone.verticalAlign.to_owned(),
            ..ComponentStyle::default()
        };
        return Ok(tmp);
    }

    fn from_block_with_idx(raw: &model::Block, _: usize) -> Result<ComponentStyle, Error> {
        Self::from_block(raw)
    }
}
