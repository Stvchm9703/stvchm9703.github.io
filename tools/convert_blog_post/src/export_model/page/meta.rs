
use serde::{Deserialize, Serialize};


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
