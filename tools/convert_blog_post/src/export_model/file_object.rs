use convert_blog_post_marco::set_field_value;
use serde::{Deserialize, Serialize};

use crate::proto::anytype::SnapshotWithType;
use anyhow::anyhow;

use super::{
    common::{
        AttributeMap, get_field_value, get_shorten_id, get_snapshot_data,
        get_snapshot_detail_field, get_snapshot_shorthanded,
    },
    trait_impl::{FromRaw, FromSnapshotList},
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileObject {
    pub id: String,
    pub sid: String,
    pub title: String,
    pub file_url: String,
    pub file_ext: String,
    pub file_type: String,
    pub attributes: AttributeMap,
}

impl<'a> FromRaw<SnapshotWithType<'a>, FileObject> for FileObject {
    fn from_raw(raw_obj: &SnapshotWithType<'a>) -> Result<FileObject, anyhow::Error> {
        let mut tmp = Self::default();
        let instance = get_snapshot_shorthanded(raw_obj);
        if let Err(e) = instance {
            return Err(anyhow!("fail field map : {:?}", e));
        }
        let (_, field_map) = instance.unwrap();

        let mut id = String::new();
        set_field_value!(id, field_map, "id");
        tmp.id = id.to_owned();
        tmp.sid = get_shorten_id(&id);
        set_field_value!(tmp.title, field_map, "name");

        set_field_value!(tmp.file_url, field_map, "source");
        let mut file_mime_type = String::new();
        set_field_value!(file_mime_type, field_map, "fileMimeType");
        let mut file_ext = String::new();
        set_field_value!(file_ext, field_map, "fileExt");

        tmp.file_type = resolve_file_type(&file_ext, &file_mime_type).to_owned();
        tmp.file_ext = file_ext;
        tmp.attributes = field_map;
        return Ok(tmp);
    }
}

impl FromSnapshotList<FileObject> for FileObject {
    fn from_snapshot_list(
        list_raw: Vec<crate::proto::anytype::SnapshotWithType>,
    ) -> Result<Vec<FileObject>, anyhow::Error> {
        let mut tmp_list: Vec<FileObject> = vec![];
        for item in list_raw {
            let tmp = FileObject::from_raw(&item);
            if let Err(e) = tmp {
                return Err(anyhow!(e));
            }
            tmp_list.push(tmp.unwrap());
        }

        return Ok(tmp_list);
    }
}

fn resolve_file_type(file_ext: &str, file_mime_type: &str) -> &'static str {
    if (file_mime_type.starts_with("image")) {
        return "images";
    }
    if (file_mime_type.starts_with("video")) {
        return "videos";
    }
    if (file_mime_type.starts_with("audio")) {
        return "audios";
    }
    // if (fileMimeType.startsWith('document')) return "Document";
    if (file_ext == "pdf") {
        return "pdf";
    }
    if (file_ext == "doc"
        || file_ext == "docx"
        || file_ext == "xls"
        || file_ext == "xlsx"
        || file_ext == "ppt"
        || file_ext == "pptx")
    {
        return "documents";
    }

    if (file_ext == "txt") {
        return "text";
    }

    if (file_ext == "csv" || file_ext == "json" || file_ext == "xml") {
        return "data";
    }

    if (file_ext == "ipynb") {
        return "notebook";
    }

    return "raw";
}
