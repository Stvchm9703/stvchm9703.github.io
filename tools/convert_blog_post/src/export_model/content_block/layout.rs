use super::{super::common::is_release, ComponentAttrType, ComponentStyle};
use crate::{
    export_model::trait_impl::{FromBlock, FromBlockContent},
    proto::anytype::model::{
        Block as RawBlock,
        mod_Block::mod_Content::{
            Layout as RawLayout, Table as RawTable, TableColumn as RawTableCol,
            TableRow as RawTableRow, mod_Layout,
        },
    },
};
use anyhow::{Error, anyhow};
use serde::{Deserialize, Serialize};

use super::ContentBlock;

pub type LayoutStyle = mod_Layout::Style;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutComponentAttr {
    #[serde(skip_serializing_if = "is_release")]
    pub id: String,
    pub order: usize,
    // #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_serializing_if = "is_release")]
    pub children_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_header: Option<bool>,
    pub style: ComponentStyle,
    pub layout_style: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LayoutItem>>,
}

impl FromBlock<LayoutComponentAttr> for LayoutComponentAttr {
    fn from_block(raw: &RawBlock) -> Result<LayoutComponentAttr, anyhow::Error> {
        let tmp = LayoutComponentAttr {
            id: raw.id.to_string(),
            children_ids: Some(
                raw.childrenIds
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            ),
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
    fn from_block_content(raw_obj: &RawLayout) -> Result<LayoutComponentAttr, anyhow::Error> {
        // let raw_clone = raw.clone();
        let tmp = LayoutComponentAttr {
            layout_style: raw_obj.style.to_string(),
            ..LayoutComponentAttr::default()
        };
        return Ok(tmp);
    }
}

impl FromBlockContent<RawTable, LayoutComponentAttr> for LayoutComponentAttr {
    fn from_block_content(raw_obj: &RawTable) -> Result<LayoutComponentAttr, anyhow::Error> {
        let tmp = LayoutComponentAttr {
            layout_style: "Table".to_string(),
            ..LayoutComponentAttr::default()
        };
        return Ok(tmp);
    }
}

impl FromBlockContent<RawTableRow, LayoutComponentAttr> for LayoutComponentAttr {
    fn from_block_content(raw_obj: &RawTableRow) -> Result<LayoutComponentAttr, anyhow::Error> {
        let tmp = LayoutComponentAttr {
            layout_style: "TableRow".to_string(),
            is_header: Some(raw_obj.isHeader),
            ..LayoutComponentAttr::default()
        };
        return Ok(tmp);
    }
}

impl FromBlockContent<RawTableCol, LayoutComponentAttr> for LayoutComponentAttr {
    fn from_block_content(raw_obj: &RawTableCol) -> Result<LayoutComponentAttr, anyhow::Error> {
        let tmp = LayoutComponentAttr {
            layout_style: "TableCol".to_string(),
            ..LayoutComponentAttr::default()
        };
        return Ok(tmp);
    }
}

impl LayoutComponentAttr {
    pub fn push_item(&mut self, item: LayoutItem) -> Result<(), Error> {
        if let Some(list) = self.items.as_mut() {
            list.push(item);
            return Ok(());
        } else {
            let tmp: Vec<LayoutItem> = vec![item];
            self.items = Some(tmp.to_owned());
            return Ok(());
        }

        return Err(anyhow!("unresolved case"));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayoutItem {
    Endpoint(ContentBlock),
    SubLayoutBlock(LayoutComponentAttr),
    Unknown,
}

impl Default for LayoutItem {
    fn default() -> Self {
        Self::Unknown
    }
}

impl LayoutItem {
    pub fn from_content_block(blk: &ContentBlock) -> LayoutItem {
        match &blk.component_attr {
            ComponentAttrType::Table(_) | ComponentAttrType::Layout(_) => Self::to_sublayout(blk),
            _ => Self::to_endpoint(blk),
        }
    }
    fn to_sublayout(blk: &ContentBlock) -> LayoutItem {
        if let ComponentAttrType::Layout(attr) = blk.component_attr.to_owned() {
            return LayoutItem::SubLayoutBlock(LayoutComponentAttr {
                id: blk.id.to_owned(),
                is_header: attr.is_header.to_owned(),
                layout_style: attr.layout_style.to_owned(),
                style: blk.style.to_owned(),
                children_ids: blk.children_ids.to_owned(),
                order: blk.order,
                ..attr
            });
        }
        if let ComponentAttrType::Table(attr) = blk.component_attr.to_owned() {
            return LayoutItem::SubLayoutBlock(LayoutComponentAttr {
                id: blk.id.to_owned(),
                is_header: attr.is_header.to_owned(),
                layout_style: attr.layout_style.to_owned(),
                style: blk.style.to_owned(),
                children_ids: blk.children_ids.to_owned(),
                order: blk.order,
                ..attr
            });
        }
        return LayoutItem::Unknown;
    }

    fn to_endpoint(blk: &ContentBlock) -> LayoutItem {
        LayoutItem::Endpoint(blk.to_owned())
    }
}

// impl FromBlockContent<RawLayout, LayoutComponentAttr> for LayoutComponentAttr {
//     fn from_block_content(raw: &RawLayout) -> Result<LayoutComponentAttr, anyhow::Error> {
//         // let raw_clone = raw.clone();
//         let tmp = LayoutComponentAttr {
//             layout_style: raw.style.to_owned(),
//             ..LayoutComponentAttr::default()
//         };
//         return Ok(tmp);
//     }
// }
