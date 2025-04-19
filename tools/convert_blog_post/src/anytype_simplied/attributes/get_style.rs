use std::collections::BTreeMap;

use crate::anytype_proto::anytype_model::{self, block, block::content};

use super::AttributeMap;

pub trait GetStyle {
    fn get_style(&self) -> AttributeMap;
}

impl GetStyle for anytype_model::Block {
    fn get_style(&self) -> AttributeMap {
        let mut style_map = BTreeMap::new();
        style_map.insert(
            "align".to_string(),
            block::Align::from_i32(self.align)
                .unwrap()
                .as_str_name()
                .to_owned(),
        );

        style_map.insert(
            "vertical_align".to_string(),
            block::VerticalAlign::from_i32(self.vertical_align)
                .unwrap()
                .as_str_name()
                .to_owned(),
        );

        style_map.insert(
            "background_color".to_string(),
            self.background_color.to_string(),
        );

        style_map
    }
}

// #[prost(message, tag = "11")]
// Smartblock(content::Smartblock),
impl GetStyle for anytype_model::block::content::Smartblock {
    fn get_style(&self) -> AttributeMap {
        BTreeMap::new()
    }
}

// #[prost(message, tag = "12")]
// Text(content::Text),
impl GetStyle for anytype_model::block::content::Text {
    fn get_style(&self) -> AttributeMap {
        let mut style_map = BTreeMap::new();
        style_map.insert("text_color".to_string(), self.color.to_string());
        style_map
    }
}

// #[prost(message, tag = "15")]
// File(content::File),
impl GetStyle for anytype_model::block::content::File {
    fn get_style(&self) -> AttributeMap {
        BTreeMap::new()
    }
}

// #[prost(message, tag = "16")]
// Layout(content::Layout),
impl GetStyle for anytype_model::block::content::Layout {
    fn get_style(&self) -> AttributeMap {
        let mut style_map = BTreeMap::new();
        style_map.insert(
            "layout".to_string(),
            content::layout::Style::from_i32(self.style)
                .unwrap()
                .as_str_name()
                .to_string(),
        );
        style_map
    }
}

// #[prost(message, tag = "17")]
// Div(content::Div),
impl GetStyle for anytype_model::block::content::Div {
    fn get_style(&self) -> AttributeMap {
        BTreeMap::new()
    }
}
// /**

//         // #[prost(message, tag = "16")]
//         Layout(content::Layout),

//         #[prost(message, tag = "17")]
//         Div(content::Div),

//         #[prost(message, tag = "18")]
//         Bookmark(content::Bookmark),

//         #[prost(message, tag = "19")]
//         Icon(content::Icon),

//         #[prost(message, tag = "20")]
//         Link(content::Link),

//         #[prost(message, tag = "21")]
//         Dataview(content::Dataview),

//         #[prost(message, tag = "22")]
//         Relation(content::Relation),

//         #[prost(message, tag = "23")]
//         FeaturedRelations(content::FeaturedRelations),

//         #[prost(message, tag = "24")]
//         Latex(content::Latex),

//         #[prost(message, tag = "25")]
//         TableOfContents(content::TableOfContents),

//         #[prost(message, tag = "26")]
//         Table(content::Table),

//         #[prost(message, tag = "27")]
//         TableColumn(content::TableColumn),

//         #[prost(message, tag = "28")]
//         TableRow(content::TableRow),
//         #[prost(message, tag = "29")]
//         Widget(content::Widget),
//         #[prost(message, tag = "30")]
//         Chat(content::Chat),
//  */
