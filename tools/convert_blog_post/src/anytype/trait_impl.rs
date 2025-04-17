use super::{Block, LayoutStyle};

use serde_variant::to_variant_name;

pub trait LayoutStyleValue {
    fn get_style(&self) -> String;
}
impl LayoutStyleValue for LayoutStyle {
    fn get_style(&self) -> String {
        self.style.clone()
    }
}
impl LayoutStyleValue for Option<LayoutStyle> {
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
        let mut return_str = String::new();

        if self.table.is_some() {
            return_str = "table".to_owned();
        }
        if self.table_column.is_some() {
            return_str = "table-column".to_owned();
        }

        if self.table_row.is_some() {
            return_str = "table-row".to_owned();
        }

        if self.table_of_contents.is_some() {
            return_str = "toc".to_owned();
        }

        if self.dataview.is_some() {
            return_str = "dataview".to_owned();
        }

        if self.bookmark.is_some() {
            return_str = "bookmark".to_owned();
        }

        if self.relation.is_some() {
            return_str = "relation".to_owned();
        }

        if let Some(text) = self.text.as_ref() {
            // let text_style = self.text.as_ref().unwrap();
            // text_style.
            return_str = to_variant_name(text.style.as_ref().unwrap())
                .unwrap_or_default()
                .to_lowercase()
                .to_owned();
        }

        if let Some(link) = self.link.as_ref() {
            if let Some(card_style) = link.card_style.as_ref() {
                return_str = to_variant_name(card_style)
                    .unwrap_or_default()
                    .to_lowercase()
                    .to_owned();
                return_str = format!("link-{}", return_str);
            } else {
                return_str = "link".to_owned();
            }
        }

        if self.div.is_some() {
            return_str = self.div.get_style();
        }

        return return_str;
    }
}
