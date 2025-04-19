use std::collections::BTreeMap;

use super::{Align, Block, LayoutStyle};

use serde::Serialize;
use serde_variant::to_variant_name;

fn to_var<T>(value: &T) -> String
where
    T: Serialize,
{
    to_variant_name(value)
        .unwrap_or_default()
        .to_lowercase()
        .to_owned()
}

pub trait GetLayoutStyle {
    fn get_style(&self) -> String;
}
impl GetLayoutStyle for LayoutStyle {
    fn get_style(&self) -> String {
        self.style.clone()
    }
}
impl GetLayoutStyle for Option<LayoutStyle> {
    fn get_style(&self) -> String {
        match self {
            Some(style) => style.get_style(),
            None => String::new(),
        }
    }
}

pub trait GetBlockComponentStyle {
    fn get_block_component_style(&self) -> String;
}

impl GetBlockComponentStyle for Block {
    fn get_block_component_style(&self) -> String {
        // let mut return_str = String::new();

        if self.table.is_some() {
            return "table".to_owned();
        }
        if self.table_column.is_some() {
            return "table-column".to_owned();
        }

        if self.table_row.is_some() {
            return "table-row".to_owned();
        }

        if self.table_of_contents.is_some() {
            return "toc".to_owned();
        }

        if self.dataview.is_some() {
            return "dataview".to_owned();
        }

        if self.bookmark.is_some() {
            return "bookmark".to_owned();
        }

        if self.relation.is_some() {
            return "relation".to_owned();
        }

        if let Some(text) = self.text.as_ref() {
            // let text_style = self.text.as_ref().unwrap();
            // text_style.
            return to_var(text.style.as_ref().unwrap());
        }

        if let Some(link) = self.link.as_ref() {
            let mut display_style = String::from("");
            let mut navigate_style = String::from("");
            if let Some(card_style) = link.card_style.as_ref() {
                display_style = to_var(card_style);
            }
            if let Some(style) = link.style.as_ref() {
                navigate_style = to_var(style);
            }
            return format!("link-{display_style}-{navigate_style}");
        }

        if self.div.is_some() {
            return self.div.get_style();
        }

        return "".to_owned();
    }
}

pub trait ToTokenString {
    fn to_token_string(&self) -> String;
}

impl ToTokenString for Align {
    fn to_token_string(&self) -> String {
        match self {
            Align::Left => "items-start".to_string(),
            Align::Center => "items-center".to_string(),
            Align::Right => "items-end".to_string(),
        }
    }
}

pub trait GetStyleMap<T> {
    fn get_style_map(&self) -> BTreeMap<String, String>;
}

impl GetStyleMap<Block> for Block {
    fn get_style_map(&self) -> BTreeMap<String, String> {
        let mut style_map: BTreeMap<String, String> = BTreeMap::new();
        style_map.insert("align".to_string(), to_var(&self.align));
        style_map.insert("vertical_align".to_string(), to_var(&self.vertical_align));
        style_map.insert(
            "background_color".to_string(),
            self.background_color.to_owned(),
        );

        // style_map.append(other);

        style_map
    }
}
