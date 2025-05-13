mod export_model;
mod proto;
// mod anytype_simplied;

use std::{
    borrow::Borrow,
    fs::{self, File, OpenOptions, exists},
    io::Write,
    path::Path,
};

// use crate::proto::anytype::object::AnytypeObject;
// use crate::anytype_simplied::{convert_anytype_object, convert_snapshot};
use crate::export_model::{
    common::{ObjectTypes, get_snapshot_data},
    file_object::FileObject,
    page::Page,
    tag::Tag,
    tag_option::TagOption,
};
use crate::proto::anytype::SnapshotWithType;
// use crate::anytype_proto::SnapshotWithType;

use export_model::{
    collection::Collection,
    common::{DEFAULT_TAG, header_id_resolver, path_resolver},
    external_link::ExternalBookmarkLink,
    page_util, tag_util,
    trait_impl::FromRaw,
};
use glob;
use proto::anytype::model::{SmartBlockSnapshotBase, SmartBlockType};
use quick_protobuf::MessageRead;
use serde::{Deserialize, Serialize};
use serde_json::json;
// use prost::{self, Message};

fn main() {
    println!("Hello, world!");
    // test_run();
    test_prost();
}

// const ANYTYPE_BASE_PATH: &str = "../../blog_post/Anytype.20250415.122153.2";

const ANYTYPE_PB_PATH: &str = "../../blog_post/Anytype.20250505.160721.07";

fn test_prost() {
    // Test the functionality of the convert_blog_post tool
    // ...
    let prefix = format!("{}/**/*.pb", ANYTYPE_PB_PATH);
    let mut page_list: Vec<Page> = vec![];
    let mut coll_list: Vec<Collection> = vec![];
    let mut tag_list: Vec<Tag> = vec![];
    let mut tag_opt_list: Vec<TagOption> = vec![];
    let mut file_obj_list: Vec<FileObject> = vec![];
    let mut bookmark_list: Vec<ExternalBookmarkLink> = vec![];

    for entry in glob::glob(&prefix).unwrap().into_iter() {
        // Process each file here
        let path = entry.ok().unwrap();

        let file_bytes = std::fs::read(&path).unwrap();

        let mut reader = quick_protobuf::BytesReader::from_bytes(&file_bytes.clone());
        let fil: SnapshotWithType =
            SnapshotWithType::from_reader(&mut reader, &file_bytes).unwrap();
        // println!("fil:{:?}", fil);

        if let Ok(snapshot_data) = get_snapshot_data(&fil) {
            if has_object_type(&snapshot_data, ObjectTypes::Collection) {
                if let Ok(obj) = Collection::from_raw(&fil) {
                    coll_list.push(obj);
                }
            }

            if fil.sbType == SmartBlockType::Page
                && has_object_type(&snapshot_data, ObjectTypes::Page)
            {
                let inited = Page::from_raw(&fil);
                if let Ok(obj) = inited {
                    // println!("{:#?}", obj);
                    page_list.push(obj);
                }
                // else {
                //     let e = inited.err();
                //     println!("err : {:?}", e);
                // }
            }

            if fil.sbType == SmartBlockType::STRelation {
                if let Ok(obj) = Tag::from_raw(&fil) {
                    tag_list.push(obj);
                }
            }

            if fil.sbType == SmartBlockType::STRelationOption {
                if let Ok(obj) = TagOption::from_raw(&fil) {
                    tag_opt_list.push(obj);
                }
            }

            if fil.sbType == SmartBlockType::File || fil.sbType == SmartBlockType::FileObject {
                if let Ok(obj) = FileObject::from_raw(&fil) {
                    file_obj_list.push(obj);
                }
            }

            if has_object_type(&snapshot_data, ObjectTypes::Bookmark) {
                if let Ok(obj) = ExternalBookmarkLink::from_raw(&fil) {
                    bookmark_list.push(obj);
                }
            }
        }
        // println!(
        //     "file-btyps : <{:?}> : {:?} ",
        //     &path,
        //     size_of_val(&*file_bytes)
        // );
        // println!("decoded : <{:?}> : {:?} ", &path, size_of_val(&fil));

        drop(fil);
        drop(file_bytes);
    }

    tag_util::resolve_tag_option(&mut tag_list, &tag_opt_list);
    // page_util::resolve_page_external(&mut page_list, &file_obj_list, &bookmark_list, &tag_list);

    // println!("tag : {:#?}", tag_list);

    for page in page_list.iter_mut() {
        page_util::resolve_page_external(page, &file_obj_list, &bookmark_list, &tag_list);
    }

    let page_ref_list = &page_list.to_owned();
    for page in page_list.iter_mut() {
        page_util::resolve_page_related_posts(page, page_ref_list);
    }

    for page in page_list.into_iter() {
        save_to_file(&page, format!("rust-ver/{}.json", page.id).as_str());
    }

    if let Some(serie_tag) = tag_list.iter().find(|f| f.name == "Series") {
        let serie_page_list = tag_util::generate_serie_index(serie_tag, page_ref_list);

        let mut post_index_page = vec![];

        serie_page_list.iter().for_each(|f| {
            save_to_file(
                f,
                format!(
                    "rust-ver/series/{}/p{}.json",
                    header_id_resolver(f.name.as_str(), f.id.as_str()),
                    f.page_index.unwrap()
                )
                .as_str(),
            );

            if f.page_index.is_some_and(|page_num| page_num == 0) {
                let mut y = f.to_owned();
                let short_list = y.result_list.chunks(3).next().unwrap();
                y.result_list = short_list.to_vec();
                post_index_page.push(y);
            }
        });

        save_to_file(&post_index_page, "rust-ver/series/index.json");
    }

    if let Ok(tag_set) = DEFAULT_TAG.lock() {
        let tag_page_list = tag_util::generate_tag_index(&tag_set, page_ref_list);
        let mut post_index_page = vec![];
        tag_page_list.iter().for_each(|f| {
            save_to_file(
                f,
                format!(
                    "rust-ver/tags/{}/p{}.json",
                    header_id_resolver(f.name.as_str(), f.id.as_str()),
                    f.page_index.unwrap()
                )
                .as_str(),
            );

            if f.page_index.is_some_and(|page_num| page_num == 0) {
                let mut y = f.to_owned();
                let short_list = y.result_list.chunks(3).next().unwrap();
                y.result_list = short_list.to_vec();
                post_index_page.push(y);
            }
        });
        save_to_file(&post_index_page, "rust-ver/tags/index.json");
    }
}

fn has_object_type(snapshot_data: &SmartBlockSnapshotBase, check_type: ObjectTypes) -> bool {
    snapshot_data
        .objectTypes
        .iter()
        .any(|x| x == check_type.to_str())
}

fn save_to_file<T: Serialize>(item: &T, fname: &str) {
    let list_as_json = serde_json::to_string_pretty(item).unwrap();

    let path = Path::new(fname);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    if exists(fname).is_ok_and(|f| f) {
        fs::remove_file(fname).expect("cannot clear the file")
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fname)
        .expect("Could not touch file!");
    // File::create(fname).expect("Could not create file!");

    file.write_all(list_as_json.as_bytes())
        .expect("Cannot write to the file!");
}
