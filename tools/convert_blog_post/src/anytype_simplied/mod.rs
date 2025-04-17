pub mod article;
pub mod attribute;
pub mod collection;
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

pub type FileDataSet = Vec<AnytypeObject>;
pub type EntryDataSet = BTreeMap<String, ATObjData>;

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
