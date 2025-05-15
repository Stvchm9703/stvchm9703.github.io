mod export_model;
mod jupyter_notbook;
mod proto;
// mod anytype_simplied;

use std::{
    borrow::Borrow,
    fmt::format,
    fs::{self, File, OpenOptions, exists},
    io::Write,
    path::Path,
};

// use crate::proto::anytype::object::AnytypeObject;
// use crate::anytype_simplied::{convert_anytype_object, convert_snapshot};
use crate::export_model::{
    common::{ObjectTypes, get_snapshot_data},
    file_object::FileObject,
    file_util::save_to_file,
    page::Page,
    tag::Tag,
    tag_option::TagOption,
};
use crate::proto::anytype::SnapshotWithType;
// use crate::anytype_proto::SnapshotWithType;

use export_model::{
    collection::Collection,
    common::{DEFAULT_TAG, header_id_resolver},
    external_link::ExternalBookmarkLink,
    // file_util::copy_file,
    page_util,
    tag_util,
    trait_impl::FromRaw,
};
use glob;
use jupyter_notbook::model::JupyterNotebookRoot;
use proto::anytype::model::{SmartBlockSnapshotBase, SmartBlockType};
use quick_protobuf::{MessageRead, MessageWrite};

// use prost::{self, Message};

fn main() {
    // println!("Hello, world!");
    // test_run();
    test_prost();
}

// const ANYTYPE_BASE_PATH: &str = "../../blog_post/Anytype.20250415.122153.2";

const ANYTYPE_PB_PATH: &str = "../../blog_post/Anytype.20250514.130201.63";

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
    let mut jupyter_notbook_list: Vec<JupyterNotebookRoot> = vec![];

    for entry in glob::glob(&prefix).unwrap().into_iter() {
        // Process each file here
        let path = entry.ok().unwrap();

        let filename = path.file_name().unwrap();

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
                if let Ok(obj) = Page::from_raw(&fil) {
                    // println!("{:#?}", obj);
                    // println!("file ; {:#?}", fil);
                    println!("id: {:?}", obj.id);
                    println!("file: {:?}", obj.title);
                    println!("filename: {:?}", filename);

                    // let tmp_filename =
                    //     format!("rust-ver/debug/{:?}.json", filename.to_str().unwrap());

                    if obj.id == "bafyreialni2kwfzmrbytsbg3xl4zwug4mp4vbxxri5tcpdddtrfraexhha" {
                        println!("fil: {:#?}", fil);
                    }
                    // fil.(format!("rust-ver/debug/{:?}.json", filename.to_str()))
                    //     .unwrap();

                    page_list.push(obj);
                }

                // save_to_file(
                //     &fil,
                //     format!("rust-ver/debug/{:?}", filename.to_str()).as_str(),
                // );
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

        drop(fil);
        drop(file_bytes);
    }

    tag_util::resolve_tag_option(&mut tag_list, &tag_opt_list);

    let file_assets_path = format!("{}/files/*", ANYTYPE_PB_PATH);
    let file_tar_path = "rust-ver/";
    for entry in glob::glob(&file_assets_path).unwrap().into_iter() {
        let path = entry.ok().unwrap();
        // println!("path ; {:?}", path.file_name());
        let tar_path = path.canonicalize().unwrap().as_path().to_owned();
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();
        fs::copy(
            &tar_path,
            format!("{}/files/{}", file_tar_path, file_name).as_str(),
        );

        if path.extension().is_some_and(|f| f == "ipynb") {
            // jupyter_notbook_list
            if let Ok(mut nb_file) = jupyter_notbook::util::read_jupyter_notebook(&tar_path) {
                nb_file.file_url = Some(file_name.to_owned());
                jupyter_notbook_list.push(nb_file);
            }
        }
    }

    for page in page_list.iter_mut() {
        page_util::resolve_page_external(
            page,
            &file_obj_list,
            &bookmark_list,
            &tag_list,
            &jupyter_notbook_list,
        );
    }

    let page_ref_list = &page_list.to_owned();
    for page in page_list.iter_mut() {
        page_util::resolve_page_related_posts(page, page_ref_list);
    }

    for page in page_list.into_iter() {
        save_to_file(&page, format!("rust-ver/{}.json", page.id).as_str());
    }

    export_series_related(tag_list, page_ref_list);

    export_tags_related(page_ref_list);
}

fn export_tags_related(page_ref_list: &Vec<Page>) {
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

fn export_series_related(tag_list: Vec<Tag>, page_ref_list: &Vec<Page>) {
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
}

fn has_object_type(snapshot_data: &SmartBlockSnapshotBase, check_type: ObjectTypes) -> bool {
    snapshot_data
        .objectTypes
        .iter()
        .any(|x| x == check_type.to_str())
}
