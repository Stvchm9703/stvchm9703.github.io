use anyhow::{Error, anyhow};
// use core::unicode::conversions::to_lower;
use once_cell::sync::Lazy;
use std::{collections::BTreeMap, sync::Mutex};
// use super::{super::common::is_release, ComponentStyle};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::proto::anytype::{SnapshotWithType, model::SmartBlockSnapshotBase};

use super::tag::Tag;

pub(crate) fn is_release<T>(_field: T) -> bool
where
    T: ToOwned,
{
    cfg!(debug_assertions)
}

pub type AttributeMap = serde_json::Map<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectTypes {
    #[serde(rename = "unknown")]
    Unknown = -1,
    #[serde(rename = "ot-image")]
    Image = 0,
    #[serde(rename = "ot-participant")]
    Participant = 1,
    #[serde(rename = "ot-page")]
    Page = 2,
    #[serde(rename = "ot-collection")]
    Collection = 3,
    #[serde(rename = "ot-set")]
    Set = 4,
    #[serde(rename = "ot-bookmark")]
    Bookmark = 5,
    #[serde(rename = "ot-space")]
    Space = 6,
    #[serde(rename = "ot-dashboard")]
    Dashboard = 7,
    #[serde(rename = "ot-relation")]
    Relation = 8,
    #[serde(rename = "ot-relationOption")]
    RelationOption = 9,
    #[serde(rename = "ot-template")]
    Template = 10,
    #[serde(rename = "ot-task")]
    Task = 11,
    #[serde(rename = "ot-project")]
    Project = 12,
    #[serde(rename = "ot-objectType")]
    ObjectType = 13,
    #[serde(rename = "ot-file")]
    File = 14,
    #[serde(rename = "ot-audio")]
    Audio = 15,
    #[serde(rename = "ot-note")]
    Note = 16,
    #[serde(rename = "ot-chatDerived")]
    ChatDerived = 17,
    #[serde(rename = "ot-video")]
    Video = 18,
    #[serde(rename = "ot-profile")]
    Profile = 19,
    #[serde(rename = "ot-date")]
    Date = 20,
    #[serde(rename = "ot-spaceView")]
    SpaceView = 21,
}

impl Default for ObjectTypes {
    fn default() -> Self {
        ObjectTypes::Unknown
    }
}

impl<'a> From<&'a str> for ObjectTypes {
    fn from(s: &'a str) -> Self {
        match s {
            "ot-image" => ObjectTypes::Image,
            "ot-participant" => ObjectTypes::Participant,
            "ot-page" => ObjectTypes::Page,
            "ot-collection" => ObjectTypes::Collection,
            "ot-set" => ObjectTypes::Set,
            "ot-bookmark" => ObjectTypes::Bookmark,
            "ot-space" => ObjectTypes::Space,
            "ot-dashboard" => ObjectTypes::Dashboard,
            "ot-relation" => ObjectTypes::Relation,
            "ot-relationOption" => ObjectTypes::RelationOption,
            "ot-template" => ObjectTypes::Template,
            "ot-task" => ObjectTypes::Task,
            "ot-project" => ObjectTypes::Project,
            "ot-objectType" => ObjectTypes::ObjectType,
            "ot-file" => ObjectTypes::File,
            "ot-audio" => ObjectTypes::Audio,
            "ot-note" => ObjectTypes::Note,
            "ot-chatDerived" => ObjectTypes::ChatDerived,
            "ot-video" => ObjectTypes::Video,
            "ot-profile" => ObjectTypes::Profile,
            "ot-date" => ObjectTypes::Date,
            "ot-spaceView" => ObjectTypes::SpaceView,
            _ => ObjectTypes::Unknown,
        }
    }
}

impl ToString for ObjectTypes {
    fn to_string(&self) -> String {
        return self.to_str().to_owned();
    }
}

impl ObjectTypes {
    pub fn to_str(&self) -> &'static str {
        match &self {
            ObjectTypes::Image => "ot-image",
            ObjectTypes::Participant => "ot-participant",
            ObjectTypes::Page => "ot-page",
            ObjectTypes::Collection => "ot-collection",
            ObjectTypes::Set => "ot-set",
            ObjectTypes::Bookmark => "ot-bookmark",
            ObjectTypes::Space => "ot-space",
            ObjectTypes::Dashboard => "ot-dashboard",
            ObjectTypes::Relation => "ot-relation",
            ObjectTypes::RelationOption => "ot-relationOption",
            ObjectTypes::Template => "ot-template",
            ObjectTypes::Task => "ot-task",
            ObjectTypes::Project => "ot-project",
            ObjectTypes::ObjectType => "ot-objectType",
            ObjectTypes::File => "ot-file",
            ObjectTypes::Audio => "ot-audio",
            ObjectTypes::Note => "ot-note",
            ObjectTypes::ChatDerived => "ot-chatDerived",
            ObjectTypes::Video => "ot-video",
            ObjectTypes::Profile => "ot-profile",
            ObjectTypes::Date => "ot-date",
            ObjectTypes::SpaceView => "ot-spaceView",
            ObjectTypes::Unknown => "unknown",
        }
    }
}

pub fn has_object_type(snapshot_data: &SmartBlockSnapshotBase, check_type: ObjectTypes) -> bool {
    snapshot_data
        .objectTypes
        .iter()
        .any(|x| x == check_type.to_str())
}

pub(crate) static GLOBAL_RELATION_NAMEMAP_STR: Lazy<Mutex<BTreeMap<String, String>>> =
    Lazy::new(|| {
        let map = BTreeMap::new();
        Mutex::new(map)
    });

pub(crate) static GLOBAL_RELATION_NAMEMAP: Lazy<Mutex<BTreeMap<String, Tag>>> = Lazy::new(|| {
    let map = BTreeMap::new();
    Mutex::new(map)
});

pub(crate) static GLOBAL_RELATION_IDMAP: Lazy<Mutex<BTreeMap<String, Tag>>> = Lazy::new(|| {
    let map = BTreeMap::new();
    Mutex::new(map)
});
use lazy_static::lazy_static;
// pub static mut DEFAULT_TAG: Option<Tag> = None;
lazy_static! {
    pub static ref DEFAULT_TAG: Mutex<Tag> = Mutex::new(Tag::default());
}
pub fn set_relation_name_map(tag: &Tag) {
    GLOBAL_RELATION_IDMAP
        .lock()
        .unwrap()
        .insert(tag.id.clone(), tag.clone());
    GLOBAL_RELATION_NAMEMAP
        .lock()
        .unwrap()
        .insert(tag.relation_key.clone(), tag.clone());
    GLOBAL_RELATION_NAMEMAP_STR
        .lock()
        .unwrap()
        .insert(tag.name.clone(), tag.relation_key.clone());
}

#[warn(unsafe_code)]
pub fn set_tag_default_set(tag: &Tag) {
    let mut t = DEFAULT_TAG.lock().unwrap();
    *t = tag.to_owned();
}

pub(crate) fn get_snapshot_data<'a>(
    input: &SnapshotWithType<'a>,
) -> Result<SmartBlockSnapshotBase<'a>, Error> {
    if input.snapshot.is_none() {
        return Err(anyhow!("snapshot is none"));
    }

    let input_snapshot = &input.snapshot.as_ref().unwrap();
    if input_snapshot.data.is_none() {
        return Err(anyhow!("snapshot data is none"));
    }
    let snapshot_data = input_snapshot.data.as_ref().unwrap().to_owned();
    return Ok(snapshot_data);
}

pub(crate) fn get_snapshot_detail_field<'a>(
    input: &SmartBlockSnapshotBase<'a>,
) -> Result<AttributeMap, Error> {
    if input.details.is_none() {
        return Err(anyhow!("details is none"));
    }
    let detail = input.details.as_ref().unwrap();
    let tmp = serde_json::to_value(detail);
    if tmp.is_err() {
        return Err(anyhow!("parsing error"));
    }
    return Ok(tmp.unwrap().as_object().unwrap().to_owned());
}

pub(crate) fn get_snapshot_shorthanded<'a>(
    input: &SnapshotWithType<'a>,
) -> Result<
    (
        SmartBlockSnapshotBase<'a>,
        serde_json::Map<String, serde_json::Value>,
    ),
    Error,
> {
    let data_wrap = get_snapshot_data(input);
    if let Err(e) = data_wrap {
        return Err(anyhow!("snapshot data not found : {:?} , {:?}", input, e));
    }

    let field_map = get_snapshot_detail_field(data_wrap.as_ref().unwrap());
    if let Err(e) = field_map {
        return Err(anyhow!("fail field map : {:?}", e));
    }
    // let field_map_ref = field_map.as_ref().unwrap();
    return Ok((data_wrap.unwrap(), field_map.unwrap()));
}

pub(crate) fn get_field_value<T>(detail_map: &AttributeMap, input_key: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    if let Some(tmp) = detail_map.get(input_key) {
        // val
        let result_wrap = serde_json::from_value(tmp.clone());
        if let Err(e) = result_wrap {
            return Err(anyhow!("parse error :{:?}", e));
        }
        return Ok(result_wrap.unwrap());
    }
    let name_map = GLOBAL_RELATION_NAMEMAP_STR.lock().unwrap();
    if let Some(tmp_key) = name_map.get(input_key) {
        // println!("getta :{:?}", tmp_key);
        if let Some(tmp) = detail_map.get(tmp_key) {
            // println!("getta-2 :{:?}", tmp);
            let result_wrap = serde_json::from_value(tmp.clone());
            if let Err(e) = result_wrap {
                return Err(anyhow!("parse error :{:?}", e));
            }
            return Ok(result_wrap.unwrap());
        }
        return Err(anyhow!(
            "no field value in key <{:?}> ({:?})",
            input_key,
            tmp_key
        ));
    }

    // eprintln!("cannot find the value with key : {:?}", input_key);
    return Err(anyhow!("no field key in with named: <{:?}> ", input_key));
}

pub(crate) fn get_shorten_id(input: &str) -> String {
    let s_pos = input
        .to_string()
        .char_indices()
        .nth_back(7)
        .map(|x| x.0)
        .unwrap_or(0);
    input[s_pos..].to_string()
}

// export const pathResolver = (path: string) =>
//   path
//     .replace(/\s+/g, "_")
//     .replace(/\W/g, "")
//     .toLowerCase()
//     .split("_")
//     .filter((e) => !isEmpty(e))
//     .join("-");
//
use stringcase::kebab_case;

pub(crate) fn path_resolver(path: &str) -> String {
    return kebab_case(path);
}

// export const headerIdResolver = (text: string, id: string) => `${pathResolver(text || "")}-${id.slice(-6)}`;
pub(crate) fn header_id_resolver(text: &str, id: &str) -> String {
    format!("{}_{}", path_resolver(text), get_shorten_id(id))
}

#[test]
fn test_set_field_value() {
    use convert_blog_post_marco::set_field_value;
    let a = serde_json::json!({
        "x": 1,
        "y" : "z",
        "z" :{
            "s" : 5
        },
    });

    let atake = a.as_object().unwrap();

    println!("a ;: {:#?}", a);

    #[derive(Debug, Serialize, Deserialize)]
    struct Azas {
        s: usize,
    }

    let mut ax: i32 = 0;
    let _ay: String = String::from("");
    let mut az: Azas = Azas { s: 3 };

    set_field_value!(ax, atake, "x");
    println!("ax  : {:?}", ax);

    set_field_value!(az, atake, "z");
    println!("az  : {:#?}", az);

    set_field_value!(az.s, atake, "x");
    println!("az  : {:#?}", az);
}

#[test]
fn test_header_id_resolver() {
    let strin = header_id_resolver("Overall   progress   ", "67fd254de212b5271c38aade");
    println!("{strin}");
}
