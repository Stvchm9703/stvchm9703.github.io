use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::enum_set::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnytypeObject {
    pub sb_type: SbType,
    pub snapshot: Snapshot,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub blocks: Vec<Block>,
    pub details: Details,
    pub file_keys: Option<serde_json::Value>,
    pub extra_relations: Option<Vec<Option<serde_json::Value>>>,
    pub object_types: Vec<ObjectType>,
    pub collections: Option<Collections>,
    pub removed_collection_keys: Option<Vec<Option<serde_json::Value>>>,
    pub relation_links: Option<Vec<RelationLink>>,
    pub key: Option<String>,
    pub original_created_timestamp: Option<String>,
    pub file_info: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub fields: Option<Fields>,
    pub restrictions: Option<Restrictions>,
    pub children_ids: Vec<String>,
    pub background_color: String,
    pub align: Align,
    pub vertical_align: VerticalAlign,
    // extra
    pub smartblock: Option<LogHeads>,
    pub featured_relations: Option<LogHeads>,
    pub layout: Option<LayoutStyle>,
    pub text: Option<Text>,
    pub relation: Option<BlockRelation>,
    pub table_of_contents: Option<LogHeads>,
    pub bookmark: Option<Bookmark>,
    pub div: Option<LayoutStyle>,
    pub table: Option<LogHeads>,
    pub link: Option<Link>,
    pub table_column: Option<LogHeads>,
    pub table_row: Option<TableRow>,
    pub widget: Option<Widget>,
    pub dataview: Option<Dataview>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fields {
    pub analytics_original_id: Option<String>,
    #[serde(rename = "_detailsKey")]
    pub details_key: Option<DetailsKeyUnion>,
    pub width: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collections {
    pub objects: Option<Vec<String>>,
    pub settings: Option<Settings>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub id: String,
    pub backlinks: Option<Vec<String>>,
    pub created_date: Option<u32>,
    pub creator: Option<String>,
    pub description: Option<String>,
    pub featured_relations: Option<Vec<String>>,
    pub last_modified_by: Option<String>,
    pub last_modified_date: Option<u32>,
    pub last_opened_date: Option<u32>,
    pub layout: u32,
    pub links: Option<Vec<String>>,
    pub mentions: Option<Vec<String>>,
    pub name: Option<String>,
    pub snippet: Option<String>,
    pub source_object: Option<String>,
    pub space_id: Option<String>,
    pub sync_date: Option<u32>,
    pub sync_error: Option<u32>,
    pub sync_status: Option<u32>,
    pub tag: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub details_type: String,
    pub restrictions: Option<Vec<u32>>,
    pub icon_option: Option<u32>,
    pub is_hidden: Option<bool>,
    pub space_dashboard_id: Option<String>,
    // #[serde(rename = "67fd138ae212b5271c38aaab")]
    // pub the_67_fd138_ae212_b5271_c38_aaab: Option<String>,
    // #[serde(rename = "67fd2398e212b5271c38aad2")]
    // pub the_67_fd2398_e212_b5271_c38_aad2: Option<Vec<String>>,
    pub icon_image: Option<String>,
    pub origin: Option<u32>,
    pub picture: Option<String>,
    pub source: Option<String>,

    // from relation-option
    pub added_date: Option<u32>,
    pub internal_flags: Option<Vec<Option<serde_json::Value>>>,
    pub relation_key: Option<String>,
    pub relation_option_color: Option<String>,
    pub unique_key: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub log_heads: LogHeads,
    pub data: Data,
    pub file_keys: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub url: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_hash: Option<String>,
    pub favicon_hash: Option<String>,
    #[serde(rename = "type")]
    pub bookmark_type: Option<SbType>,
    pub target_object_id: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutStyle {
    pub style: String,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct LogHeads {}

pub type LogHeads = BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub target_block_id: Option<String>,
    pub style: Option<SbType>,
    pub fields: Option<serde_json::Value>,
    pub icon_size: Option<IconSize>,
    pub card_style: Option<CardStyle>,
    pub description: Option<String>,
    pub relations: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockRelation {
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restrictions {
    pub read: Option<bool>,
    pub edit: Option<bool>,
    pub remove: Option<bool>,
    pub drag: Option<bool>,
    pub drop_on: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow {
    pub is_header: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: Option<String>,
    pub style: Option<TextStyle>,
    pub marks: Option<Marks>,
    pub checked: Option<bool>,
    pub color: Option<String>,
    pub icon_emoji: Option<String>,
    pub icon_image: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marks {
    pub marks: Option<Vec<Mark>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mark {
    pub range: Option<Range>,
    #[serde(rename = "type")]
    pub mark_type: Option<MarkType>,
    pub param: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub from: i64,
    pub to: i64,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum Backlink {
//     Bafyreiaaezcucbcokt5Vw4K4Lp6Qsf6R6Hkqufx4Hha27X5Z4G5K5Ggpfa,
//     Bafyreib22R3Vr6N4Ezcpcvy5Jxnddgsdhtp2Szlnxe5Bojmj4Owc2Ak24Y,
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationLink {
    pub key: String,
    pub format: Format,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DetailsKeyUnion {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dataview {
    pub source: Option<Vec<String>>,
    pub views: Option<Vec<View>>,
    pub active_view: Option<String>,
    pub relations: Option<Vec<String>>,
    pub group_orders: Option<Vec<String>>,
    pub object_orders: Option<Vec<String>>,
    pub relation_links: Option<Vec<RelationLink>>,
    #[serde(rename = "TargetObjectId")]
    pub target_object_id: Option<String>,
    pub is_collection: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub view_type: Option<String>,
    pub name: Option<String>,
    pub sorts: Option<Vec<Sort>>,
    pub filters: Option<Vec<Filter>>,
    pub relations: Option<Vec<RelationElement>>,
    pub cover_relation_key: Option<String>,
    pub hide_icon: Option<bool>,
    pub card_size: Option<String>,
    pub cover_fit: Option<bool>,
    pub group_relation_key: Option<String>,
    pub group_background_colors: Option<bool>,
    pub page_limit: Option<i64>,
    pub default_template_id: Option<String>,
    pub default_object_type_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub id: Option<String>,
    pub operator: Option<String>,
    #[serde(rename = "RelationKey")]
    pub relation_key: Option<String>,
    pub relation_property: Option<String>,
    pub condition: Option<String>,
    pub value: Option<Vec<String>>,
    pub quick_option: Option<String>,
    pub format: Option<Format>,
    pub include_time: Option<bool>,
    pub nested_filters: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationElement {
    pub key: Option<String>,
    pub is_visible: Option<bool>,
    pub width: Option<i64>,
    pub date_include_time: Option<bool>,
    pub time_format: Option<TimeFormat>,
    pub date_format: Option<DateFormat>,
    pub formula: Option<FormulaEnum>,
    pub align: Option<Align>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    #[serde(rename = "RelationKey")]
    pub relation_key: Option<String>,
    #[serde(rename = "type")]
    pub sort_type: Option<String>,
    pub custom_order: Option<Vec<Option<serde_json::Value>>>,
    pub format: Option<Format>,
    pub include_time: Option<bool>,
    pub id: Option<String>,
    pub empty_placement: Option<String>,
    pub no_collate: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "analyticsID")]
    pub analytics_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Widget {
    pub layout: Option<String>,
    pub limit: Option<i64>,
    pub view_id: Option<String>,
}
