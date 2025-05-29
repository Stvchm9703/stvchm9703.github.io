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
    pub tags: Option<Vec<PageMiniExternalLink>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serie: Option<PageMiniExternalLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMiniExternalLink {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub label: String,
    pub url: String,
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
            publish_date: Some(self.publish_date),

            ..Default::default()
        };
    }
    fn to_page_external_link(&self) -> PageExternalLink {
        self.to_page_ext_link()
    }
}

pub trait ToPageMiniExternalLink {
    fn to_page_mini_external_link(&self) -> PageMiniExternalLink;
}

impl ToPageMiniExternalLink for PageExternalLink {
    fn to_page_mini_external_link(&self) -> PageMiniExternalLink {
        PageMiniExternalLink {
            id: self.id.to_string(),
            label: self.label.to_string(),
            url: self.url.to_string(),
            sid: self.sid.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageIndexReference {
    pub id: String,
    pub post_id: String,
    pub label: String,
    pub url: String,
    pub res: String,
    pub page_content_path: String,
    pub publish_date: i64,
}

impl Page {
    pub fn to_index_reference(&self, base_path: &str) -> PageIndexReference {
        return PageIndexReference {
            id: self.id.to_string(),
            post_id: self.sid.to_string(),
            label: self.title.to_string(),
            url: format!(
                "/posts/{}_{}",
                get_shorten_id(self.id.as_str()),
                path_resolver(self.title.as_str())
            ),
            res: path_resolver(&self.title),
            page_content_path: format!("{}/{}.json", base_path, self.id),
            publish_date: self.publish_date,
        };
    }
}
