use serde::{Deserialize, Serialize};

use crate::proto::anytype::model;

use super::{ComponentStyle, mark::Mark};

pub type TextStyle = model::mod_Block::mod_Content::mod_Text::Style;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponentAttr {
    pub text: String,
    pub style: TextStyle,
    pub marks: Option<Vec<Mark>>,
    pub items: Option<Vec<TextItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextItem {
    pub id: String,
    pub text: String,
    pub style: ComponentStyle,
    pub marks: Vec<Mark>,
}
