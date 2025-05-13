use super::{
    common::{
        ObjectTypes, get_field_value, get_shorten_id, get_snapshot_shorthanded,
    },
    trait_impl::{FromRaw, FromSnapshotList},
};
use crate::proto::anytype::SnapshotWithType;
use anyhow::anyhow;
use convert_blog_post_marco::set_field_value;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalBookmarkLink {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub url: String,
    pub title: String,
    pub level: Option<i64>,
}

pub const EXTERNAL_BOOKMARK_LINK_OBJECT_TYPES: ObjectTypes = ObjectTypes::Bookmark;

impl FromSnapshotList<ExternalBookmarkLink> for ExternalBookmarkLink {
    // const FilterKeyword: ObjectTypes = ObjectTypes::Bookmark;
    fn from_snapshot_list(
        list_raw: Vec<SnapshotWithType>,
    ) -> Result<Vec<ExternalBookmarkLink>, anyhow::Error> {
        let mut tmp_list: Vec<ExternalBookmarkLink> = vec![];
        for item in list_raw {
            let tmp = ExternalBookmarkLink::from_raw(&item);
            if let Err(e) = tmp {
                return Err(anyhow!(e));
            }
            tmp_list.push(tmp.unwrap());
        }

        return Ok(tmp_list);
    }
}

impl<'a> FromRaw<SnapshotWithType<'a>, ExternalBookmarkLink> for ExternalBookmarkLink {
    fn from_raw(input: &SnapshotWithType) -> Result<ExternalBookmarkLink, anyhow::Error> {
        let mut tmp = ExternalBookmarkLink::default();
        let instance = get_snapshot_shorthanded(input);
        if let Err(e) = instance {
            return Err(anyhow!("fail field map : {:?}", e));
        }
        let (_, field_map) = instance.unwrap();

        if let Ok(id) = get_field_value::<String>(&field_map, "id") {
            // tmp.publish_date = publish_date.to_owned();
            tmp.id = id.to_owned();
            tmp.sid = get_shorten_id(&id);
        }
        set_field_value!(tmp.url, &field_map, "name");
        set_field_value!(tmp.title, &field_map, "title");
        return Ok(tmp);
    }
}
