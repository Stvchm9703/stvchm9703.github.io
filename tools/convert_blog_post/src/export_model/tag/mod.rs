pub mod option;
pub mod util;

use super::{
    common::{
        AttributeMap, get_field_value, get_shorten_id, get_snapshot_shorthanded,
    },
    // option::TagOption,
    page::ext::{PageExternalLink, ToPageExternalLink},
    trait_impl::{FromRaw, FromSnapshotList},
};
use crate::proto::anytype::SnapshotWithType;
use anyhow::anyhow;
use convert_blog_post_marco::set_field_value;
use option::TagOption;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub name: String,
    pub description: String,
    pub relation_key: String, // st-relationshipKey
    // styles: string[];
    #[serde(skip)]
    pub attributes: AttributeMap, // Define AttributeMap according to your needs
    pub options: Vec<TagOption>,
}

impl FromSnapshotList for Tag {
    fn from_snapshot_list(list_raw: Vec<SnapshotWithType>) -> Result<Vec<Tag>, anyhow::Error> {
        let mut tmp_list: Vec<Tag> = vec![];
        for item in list_raw {
            let tmp = Tag::from_raw(&item);
            if let Err(e) = tmp {
                return Err(anyhow!(e));
            }
            tmp_list.push(tmp.unwrap());
        }

        return Ok(tmp_list);
    }
}

impl<'a> FromRaw<SnapshotWithType<'a>> for Tag {
    fn from_raw(input: &SnapshotWithType) -> Result<Tag, anyhow::Error> {
        let mut tmp = Tag::default();
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
        set_field_value!(tmp.name, &field_map, "name");
        set_field_value!(tmp.description, &field_map, "description");
        set_field_value!(tmp.relation_key, &field_map, "relationKey");
        tmp.attributes = field_map.to_owned();
        return Ok(tmp);
    }
}

impl Tag {
    pub fn get_option_page_link(&self, opt_id: &str) -> Option<PageExternalLink> {
        let opt = self.options.iter().find(|o| o.id.as_str() == opt_id);
        if opt.is_none() {
            return None;
        }
        let opt_t = opt.unwrap();
        Some(opt_t.to_page_ext_link())
    }
}
