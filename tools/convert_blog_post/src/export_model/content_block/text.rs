use serde::{Deserialize, Serialize};

// use super::mark;
use super::{ComponentStyle, mark::Mark};
use crate::export_model::trait_impl::FromRaw;
use crate::proto::anytype::model;
use crate::{
    export_model::trait_impl::{AddFromExternalFile, FromBlockContent},
    proto::anytype::{
        SnapshotWithType,
        model::mod_Block::mod_Content::{Text as RawText, mod_Text},
    },
};

pub type TextStyle = mod_Text::Style;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextComponentAttr {
    pub text: String,
    pub style: TextStyle,
    pub marks: Option<Vec<Mark>>,
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
pub struct TextItem {
    pub id: String,
    pub text: String,
    pub style: ComponentStyle,
    pub marks: Vec<Mark>,
}
