use crate::{
    export_model::trait_impl::FromBlockContent,
    proto::anytype::model::mod_Block::mod_Content::{Latex as RawLatex, mod_Latex},
};
use serde::{Deserialize, Serialize};

pub type ProcessorType = mod_Latex::Processor;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatexComponentAttr {
    pub text: String,
    pub processor: ProcessorType,
}

impl<'life> FromBlockContent<RawLatex<'life>> for LatexComponentAttr {
    fn from_block_content(raw: &RawLatex<'life>) -> Result<LatexComponentAttr, anyhow::Error> {
        let tmp = LatexComponentAttr {
            text: raw.text.to_string(),
            processor: raw.processor.to_owned(),
            ..LatexComponentAttr::default()
        };
        return Ok(tmp);
    }
}
