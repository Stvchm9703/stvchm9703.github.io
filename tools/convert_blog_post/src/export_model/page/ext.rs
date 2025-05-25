
use serde::{Deserialize, Serialize};

use super::{Page, get_shorten_id, path_resolver};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageExternalLink {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    pub label: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PageExternalLink>>,
}

pub trait ToPageExternalLink {
    fn to_page_external_link(&self) -> PageExternalLink;
    fn to_page_ext_link(&self) -> PageExternalLink;
}

impl ToPageExternalLink for Page {
    fn to_page_ext_link(&self) -> PageExternalLink {
        return PageExternalLink {
            id: self.id.to_string(),
            sid: get_shorten_id(self.id.as_str()),
            label: self.title.to_string(),
            url: format!(
                "/posts/{}_{}",
                get_shorten_id(self.id.as_str()),
                path_resolver(self.title.as_str())
            ),
            ..Default::default()
        };
    }
    fn to_page_external_link(&self) -> PageExternalLink {
        self.to_page_ext_link()
    }
}
