pub mod base_block;

pub mod bookmark;
pub mod file;
pub mod jupyter;
pub mod latex;
pub mod layout;
pub mod link;
pub mod mark;
pub mod text;

use std::borrow::Borrow;

use bookmark::BookmarkComponentAttr;
use file::FileComponentAttr;
use jupyter::JupyterComponentAttr;
use latex::LatexComponentAttr;
use layout::LayoutComponentAttr;
use link::LinkComponentAttr;
use serde::{Deserialize, Serialize};
use serde_json::json;
use text::{TextComponentAttr, TextStyle};

use anyhow::{Error, anyhow};

use super::{
    common::{AttributeMap, header_id_resolver, is_release},
    page::ext::PageExternalLink,
    trait_impl::{FromBlock, FromBlockContent},
};
use crate::proto::anytype::model::{self, mod_Block};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentBlock {
    pub id: String,
    pub order: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<AttributeMap>,
    pub children_ids: Option<Vec<String>>,
    pub style: ComponentStyle,
    pub component_attr: ComponentAttrType,
    #[serde(skip_serializing_if = "is_release")]
    pub debug_type: String,
}

impl FromBlock for ContentBlock {
    fn from_block(raw_blk: &model::Block) -> Result<ContentBlock, anyhow::Error> {
        let children_ids = raw_blk.childrenIds.iter();
        let children_ids = children_ids.map(|x| x.to_string()).collect();

        let mut tmp = ContentBlock {
            id: raw_blk.id.to_string(),
            children_ids: Some(children_ids),
            style: ComponentStyle::from_block(raw_blk)?,
            debug_type: raw_blk.content.to_string(),
            ..ContentBlock::default()
        };
        let attr = ComponentAttrType::from_block_content(&raw_blk.content);
        if let Ok(component_attr) = attr {
            tmp.component_attr = component_attr;
        }

        if let Some(f) = raw_blk.fields.as_ref() {
            if let Ok(y) = serde_json::to_value(f) {
                tmp.fields = Some(y.as_object().unwrap().to_owned());
            }
        }

        Ok(tmp)
    }

    fn from_block_with_idx(raw: &model::Block, idx: usize) -> Result<ContentBlock, Error> {
        let mut blk = Self::from_block(raw)?;
        blk.order = idx;
        Ok(blk)
    }
}

impl<'a> ToString for mod_Block::OneOfcontent<'a> {
    fn to_string(&self) -> String {
        return match self {
            mod_Block::OneOfcontent::smartblock(_) => "smartblock",
            mod_Block::OneOfcontent::text(_) => "text",
            mod_Block::OneOfcontent::file(_) => "file",
            mod_Block::OneOfcontent::layout(_) => "layout",
            mod_Block::OneOfcontent::div(_) => "div",
            mod_Block::OneOfcontent::bookmark(_) => "bookmark",
            mod_Block::OneOfcontent::icon(_) => "icon",
            mod_Block::OneOfcontent::link(_) => "link",
            mod_Block::OneOfcontent::dataview(_) => "dataview",
            mod_Block::OneOfcontent::relation(_) => "relation",
            mod_Block::OneOfcontent::featuredRelations(_) => "featured_relations",
            mod_Block::OneOfcontent::latex(_) => "latex",
            mod_Block::OneOfcontent::tableOfContents(_) => "table_of_contents",
            mod_Block::OneOfcontent::table(_) => "table",
            mod_Block::OneOfcontent::tableColumn(_) => "table_column",
            mod_Block::OneOfcontent::tableRow(_) => "table_row",
            mod_Block::OneOfcontent::widget(_) => "widget",
            mod_Block::OneOfcontent::chat(_) => "chat",
            mod_Block::OneOfcontent::None => "none",
        }
        .to_string();
    }
}

impl ContentBlock {
    pub(crate) fn to_toc_link(&self) -> Option<PageExternalLink> {
        if let ComponentAttrType::Text(attr) = self.component_attr.borrow() {
            let mut level: usize = 0;
            if attr.style == TextStyle::Header1 {
                level = 1;
            } else if attr.style == TextStyle::Header2 {
                level = 2;
            } else if attr.style == TextStyle::Header3 {
                level = 3;
            } else if attr.style == TextStyle::Header4 {
                level = 4;
            }

            let id = header_id_resolver(attr.text.as_str(), &self.id);
            let mut tmp = PageExternalLink {
                id: id.to_string(),
                component_id: Some(self.id.to_string()),
                label: attr.text.to_string(),
                url: format!("#{}", id.as_str()),
                ..Default::default()
            };

            if level > 0 {
                tmp.level = Some(level);
            }

            return Some(tmp);
        }

        return None;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "componentType")]
pub enum ComponentAttrType {
    None,
    Unknown(AttributeMap),
    Bookmark(BookmarkComponentAttr),
    File(FileComponentAttr),
    JupyterComponent(JupyterComponentAttr),
    Layout(LayoutComponentAttr),
    Link(LinkComponentAttr),
    Latex(LatexComponentAttr),
    Table(LayoutComponentAttr),
    Text(TextComponentAttr),
}
impl Default for ComponentAttrType {
    fn default() -> Self {
        Self::Unknown(json!({}).as_object().unwrap().to_owned())
    }
}
impl ComponentAttrType {
    fn from_input(input: serde_json::Value) -> Self {
        Self::Unknown(input.as_object().unwrap().to_owned())
    }
}

impl<'a> FromBlockContent<mod_Block::OneOfcontent<'a>> for ComponentAttrType {
    fn from_block_content(
        raw_obj: &mod_Block::OneOfcontent<'a>,
    ) -> Result<ComponentAttrType, Error> {
        match raw_obj {
            mod_Block::OneOfcontent::smartblock(_) => Ok(ComponentAttrType::from_input(json!({}))),
            mod_Block::OneOfcontent::text(text) => {
                let y = TextComponentAttr::from_block_content(text);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Text(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::file(file) => {
                let y = FileComponentAttr::from_block_content(file);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::File(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::layout(layout) => {
                let y = LayoutComponentAttr::from_block_content(layout);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Layout(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::div(div) => Ok(ComponentAttrType::from_input(json!({
                "style": div.style.to_string(),
                "orginal_type": "div",
            }))),
            mod_Block::OneOfcontent::bookmark(bookmark) => {
                let y = BookmarkComponentAttr::from_block_content(bookmark);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Bookmark(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::icon(_) => todo!(),
            mod_Block::OneOfcontent::link(link) => {
                let y = LinkComponentAttr::from_block_content(link);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Link(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::dataview(_) => Ok(ComponentAttrType::from_input(json!({
                "orginal_type": "dataview",
            }))),
            mod_Block::OneOfcontent::relation(_) => Ok(ComponentAttrType::from_input(json!({
                "orginal_type": "relation",
            }))),
            mod_Block::OneOfcontent::featuredRelations(_) => {
                Ok(ComponentAttrType::from_input(json!({
                    "orginal_type": "featuredRelations",
                })))
            }
            mod_Block::OneOfcontent::latex(latex) => {
                let y = LatexComponentAttr::from_block_content(latex);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Latex(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::tableOfContents(_) => {
                Ok(ComponentAttrType::from_input(json!({
                    "orginal_type": "tableOfContents",
                })))
            }
            mod_Block::OneOfcontent::table(table) => {
                let y = LayoutComponentAttr::from_block_content(table);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Table(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::tableColumn(table_column) => {
                let y = LayoutComponentAttr::from_block_content(table_column);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Table(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::tableRow(table_row) => {
                let y = LayoutComponentAttr::from_block_content(table_row);
                if let Ok(result) = y {
                    Ok(ComponentAttrType::Table(result))
                } else {
                    Err(anyhow!("init error :{:?}", y.err()))
                }
            }
            mod_Block::OneOfcontent::widget(_) => Ok(ComponentAttrType::from_input(json!({
                "orginal_type": "widget",
            }))),
            mod_Block::OneOfcontent::chat(_) => Ok(ComponentAttrType::from_input(json!({
                "orginal_type": "chat",
            }))),
            mod_Block::OneOfcontent::None => Ok(ComponentAttrType::from_input(json!({
                "orginal_type": "none",
            }))),
        }
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

impl FromBlock for ComponentStyle {
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
