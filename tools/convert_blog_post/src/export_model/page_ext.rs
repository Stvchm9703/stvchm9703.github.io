use anyhow::{Error, anyhow};

use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
    pub images: Option<Vec<PageMetaOpenGraphObj>>,
    pub videos: Option<Vec<PageMetaOpenGraphObj>>,
    pub audio: Option<Vec<PageMetaOpenGraphObj>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMetaOpenGraphObj {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure_url: Option<String>,
    #[serde(rename = "type")]
    pub file_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}
