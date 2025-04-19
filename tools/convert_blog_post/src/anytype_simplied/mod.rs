pub mod attributes;
pub mod collection;
pub mod component_base;
pub mod content_block;
pub mod page;
// pub mod relation;
pub mod tag;
pub mod user;
pub mod workspace;

use std::{clone, collections::BTreeMap};

use collection::Collection;
use page::Page;
use tag::Tag;
use user::User;
use workspace::Workspace;

use crate::anytype::{
    enum_set::{ObjectType, SbType},
    object::{AnytypeObject, Data as ATObjData},
};

use crate::anytype_proto::{anytype::SnapshotWithType, anytype_model::SmartBlockType};

pub type FileDataSet = Vec<AnytypeObject>;
pub type EntryDataSet = BTreeMap<String, ATObjData>;

#[deprecated]
pub fn convert_anytype_object(at_object: &FileDataSet) -> BTreeMap<String, Workspace> {
    // Implementation of convert_anytype_object function
    //
    //

    let page_raw = at_object
        .into_iter()
        .filter(|obj| obj.sb_type == SbType::Page)
        .collect::<Vec<_>>();

    let mut user_raw = at_object
        .into_iter()
        .filter_map(|obj| {
            if obj.sb_type == SbType::Participant {
                Some(User::from_anytype(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut workspace_raw = at_object
        .into_iter()
        .filter_map(|obj| {
            if obj.sb_type == SbType::Workspace {
                Some(Workspace::from_anytype(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let article_raw = page_raw
        .clone()
        .into_iter()
        .filter_map(|obj| {
            if obj.snapshot.data.object_types.contains(&ObjectType::Page) {
                Some(Page::from_anytype(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let collection_raw = page_raw
        .clone()
        .into_iter()
        .filter_map(|obj| {
            if obj
                .snapshot
                .data
                .object_types
                .contains(&ObjectType::Collection)
            {
                Some(Collection::from_anytype(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("workspace {:?}", workspace_raw);
    println!("user {:?}", user_raw);
    println!("collection {:?}", collection_raw);

    // println!("article {:#?}", article_raw);

    // let user_map = User::from_anytype_list(&user_raw);
    let tag_map = Tag::from_anytype_list(
        &at_object
            .into_iter()
            .filter(|obj| obj.sb_type == SbType::STRelation)
            .collect::<Vec<_>>(),
    );

    let mut workspace_map: BTreeMap<String, Workspace> = BTreeMap::new();
    workspace_raw.into_iter().for_each(|workspace| {
        workspace_map.insert(workspace.id.clone(), workspace);
    });

    workspace_map
}

pub fn convert_snapshot(at_object: &Vec<SnapshotWithType>) -> BTreeMap<String, Workspace> {
    let page_raw = at_object
        .into_iter()
        .filter(|obj| obj.sb_type == SmartBlockType::Page as i32)
        .collect::<Vec<_>>();

    let mut user_raw = at_object
        .into_iter()
        .filter_map(|obj| {
            if obj.sb_type == SmartBlockType::Participant as i32 {
                Some(User::from_anytype_proto(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("user map ;{:#?} {}", user_raw[0], user_raw.len());

    let mut workspace_raw = at_object
        .into_iter()
        .filter_map(|obj| {
            if obj.sb_type == SmartBlockType::Workspace as i32 {
                Some(Workspace::from_anytype_proto(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!(
        "workspace map ;{:#?} {}",
        workspace_raw[0],
        workspace_raw.len()
    );

    let mut article_raw = page_raw
        .clone()
        .into_iter()
        .filter_map(|obj| {
            let smart_block_snapshot_base = obj.snapshot.as_ref()?.data.as_ref()?;
            if smart_block_snapshot_base
                .object_types
                .contains(&ObjectType::Page.to_string())
            {
                Some(Page::from_anytype_proto(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!(
        "article_raw map ;{:#?} {}",
        article_raw[0],
        article_raw.len()
    );

    let collection_raw = page_raw
        .clone()
        .into_iter()
        .filter_map(|obj| {
            if obj
                .snapshot
                .as_ref()?
                .data
                .as_ref()?
                .object_types
                .contains(&ObjectType::Collection.to_string())
            {
                Some(Collection::from_anytype_proto(obj))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    // println!("workspace {:?}", workspace_raw);
    // println!("user {:?}", user_raw);
    println!(
        "collection_raw map ;{:#?} {}",
        collection_raw[0],
        collection_raw.len()
    );

    // // println!("article {:#?}", article_raw);

    // // let user_map = User::from_anytype_list(&user_raw);
    // let tag_map = Tag::from_anytype_list(
    //     &at_object
    //         .into_iter()
    //         .filter(|obj| obj.sb_type == SbType::STRelation)
    //         .collect::<Vec<_>>(),
    // );

    let mut workspace_map: BTreeMap<String, Workspace> = BTreeMap::new();
    // workspace_raw.into_iter().for_each(|workspace| {
    //     workspace_map.insert(workspace.id.clone(), workspace);
    // });

    workspace_map
}
