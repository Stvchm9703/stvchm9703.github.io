use serde::{Deserialize, Serialize};

use crate::{
    export_model::{
        common::{
            AttributeMap, get_field_value, get_shorten_id, get_snapshot_shorthanded,
            header_id_resolver,
        },
        page::ext::{PageExternalLink, ToPageExternalLink},
        trait_impl::{FromRaw, FromSnapshotList},
    },
    proto::anytype::SnapshotWithType,
};
use anyhow::anyhow;
use convert_blog_post_marco::set_field_value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagOption {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub name: String,
    pub description: String,
    pub relation_key: String, // st-relationshipKey
    // styles: string[];
    #[serde(skip)]
    pub attributes: AttributeMap, // Define AttributeMap according to your needs
}

impl FromSnapshotList for TagOption {
    fn from_snapshot_list(
        list_raw: Vec<SnapshotWithType>,
    ) -> Result<Vec<TagOption>, anyhow::Error> {
        let mut tmp_list: Vec<TagOption> = vec![];
        for item in list_raw {
            let tmp = TagOption::from_raw(&item);
            if let Err(e) = tmp {
                return Err(anyhow!(e));
            }
            tmp_list.push(tmp.unwrap());
        }

        return Ok(tmp_list);
    }
}

impl<'a> FromRaw<SnapshotWithType<'a>> for TagOption {
    fn from_raw(input: &SnapshotWithType) -> Result<TagOption, anyhow::Error> {
        let mut tmp = TagOption::default();
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

impl ToPageExternalLink for TagOption {
    fn to_page_external_link(&self) -> PageExternalLink {
        return PageExternalLink {
            id: self.id.to_string(),
            sid: get_shorten_id(self.id.as_str()),
            label: self.name.to_string(),
            url: format!(
                "/posts/tags/{}",
                header_id_resolver(self.name.as_str(), self.id.as_str())
            ),
            ..Default::default()
        };
    }

    /// alias to to_page_external_link
    fn to_page_ext_link(&self) -> PageExternalLink {
        self.to_page_external_link()
    }
}
