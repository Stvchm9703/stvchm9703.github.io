use anyhow::{Error, anyhow};
use serde::{Deserialize, Serialize};

use crate::proto::anytype::model::SmartBlockSnapshotBase;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageAttributesFromCast {
    pub featured_relations: Option<Vec<serde_json::Value>>,
    pub id: String,
    pub snippet: String,
    pub resolved_layout: f64,
    pub space_id: String,
    // #[serde(rename = "67fd2398e212b5271c38aad2")]
    // pub the_67_fd2398_e212_b5271_c38_aad2: Vec<Option<serde_json::Value>>,
    pub last_modified_by: String,
    pub mentions: Vec<String>,
    pub name: String,
    pub sync_date: f64,
    // #[serde(rename = "67ff5cd5e212b569d0ab04d2")]
    // pub the_67_ff5_cd5_e212_b569_d0_ab04_d2: f64,
    pub sync_error: f64,
    pub created_date: f64,
    pub sync_status: f64,
    pub links: Vec<String>,
    pub backlinks: Vec<String>,
    #[serde(rename = "type")]
    pub attributes_type: String,
    pub last_modified_date: f64,
    pub last_opened_date: f64,
    pub creator: String,
    pub tag: Option<Vec<String>>,
    pub description: Option<String>,

    pub source_object: String,
    pub layout: f64,
}
pub(crate) fn get_page_details<'a>(
    input: &SmartBlockSnapshotBase<'a>,
) -> Result<PageAttributesFromCast, Error> {
    if input.details.is_none() {
        return Err(anyhow!("details is none"));
    }
    let detail = input.details.as_ref().unwrap();
    let tmp = serde_json::to_value(detail);
    if tmp.is_err() {
        return Err(anyhow!("parsing error ; to-value"));
    }
    let result = serde_json::from_value::<PageAttributesFromCast>(tmp.unwrap());
    if result.is_err() {
        return Err(anyhow!("parsing error ; from-value"));
    }

    return Ok(result.unwrap());
}
