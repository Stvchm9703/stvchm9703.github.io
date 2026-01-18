use crate::{
    export_model::trait_impl::FromBlockContent,
    proto::anytype::model::mod_Block::mod_Content::{File as RawFile, mod_File},
};
use serde::{Deserialize, Serialize};

pub type FileType = mod_File::Type;
pub type FileStyle = mod_File::Style;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileComponentAttr {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub mime: String,
    pub size: i64,
    pub target_object_id: String,
    pub state: String,
    pub style: FileStyle,
    pub file_url: String,
}

impl<'life> FromBlockContent<RawFile<'life>> for FileComponentAttr {
    fn from_block_content(raw: &RawFile<'life>) -> Result<FileComponentAttr, anyhow::Error> {
        let tmp = FileComponentAttr {
            name: raw.name.to_string(),
            file_type: raw.type_pb.to_owned(),
            mime: raw.mime.to_string(),
            size: raw.size.to_owned(),
            target_object_id: raw.targetObjectId.to_string(),
            style: raw.style.to_owned(),
            ..FileComponentAttr::default()
        };
        return Ok(tmp);
    }
}
