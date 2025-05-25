use std::collections::BTreeMap;

use convert_blog_post_marco::set_field_value;
use serde::{Deserialize, Serialize};

use crate::proto::anytype::SnapshotWithType;
use anyhow::anyhow;

use super::{
    common::{AttributeMap, get_field_value, get_shorten_id, get_snapshot_shorthanded},
    page::Page,
    trait_impl::FromRaw,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    id: String,
    #[serde(rename = "_sid")]
    sid: String,
    name: String,
    description: String,
    cover: String,
    styles: Vec<String>,
    attributes: AttributeMap,
    workspace_id: String,
    article_id_list: Vec<String>,
    articles: BTreeMap<String, Page>,
    // fromAnytype?(raw: SnapshotWithType): Collection;
}
impl<'a> FromRaw<SnapshotWithType<'a>> for Collection {
    fn from_raw(raw_obj: &SnapshotWithType<'a>) -> Result<Collection, anyhow::Error> {
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
        set_field_value!(tmp.name, field_map, "name");
        set_field_value!(tmp.description, field_map, "description");

        set_field_value!(tmp.workspace_id, field_map, "spaceId");
        set_field_value!(tmp.article_id_list, field_map, "links");

        tmp.attributes = field_map;
        return Ok(tmp);
    }
}
