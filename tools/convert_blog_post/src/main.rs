mod cli;
mod export_model;
mod jupyter_notbook;
mod proto;
// mod anytype_simplied;

use std::{
    fs::{self},
    path::{self, PathBuf},
};

// use crate::proto::anytype::object::AnytypeObject;
// use crate::anytype_simplied::{convert_anytype_object, convert_snapshot};
use crate::export_model::{
    collection::Collection,
    common::{DEFAULT_TAG, ObjectTypes, get_snapshot_data, has_object_type, header_id_resolver},
    external_link::ExternalBookmarkLink,
    file_object::FileObject,
    file_util::save_to_file_path,
    page::{self, Page},
    tag::{self, Tag, option::TagOption},
    trait_impl::FromRaw,
};
use crate::proto::anytype::SnapshotWithType;
// use crate::anytype_proto::SnapshotWithType;

use anyhow::{Error, Result, anyhow};
use glob;
use jupyter_notbook::model::JupyterNotebookRoot;
use proto::anytype::model::SmartBlockType;
use quick_protobuf::MessageRead;

use clap::Parser;
use cli::Args;

// use prost::{self, Message};

fn main() {
    let args = Args::parse();
    main_process(&args).unwrap();
}

// const ANYTYPE_BASE_PATH: &str = "../../blog_post/Anytype.20250415.122153.2";

//const ANYTYPE_PB_PATH: &str = "../../blog_post/Anytype.20250514.130201.63";

fn main_process(arg: &Args) -> Result<(), anyhow::Error> {
    // Test the functionality of the convert_blog_post tool
    // ...
    let import_path = PathBuf::from(&arg.import_path).canonicalize().unwrap();

    if fs::exists(&arg.export_path).is_ok_and(|f| f == false) {
        fs::create_dir_all(&arg.export_path)?;
    }
    let export_path = PathBuf::from(&arg.export_path).canonicalize().unwrap();

    println!("import-path : {:?}", import_path);
    println!("export-path : {:?}", export_path);

    let check_import_path = fs::exists(&import_path);
    if check_import_path.as_ref().is_ok_and(|f| f == &false) || check_import_path.as_ref().is_err()
    {
        return Err(anyhow::anyhow!("import path does not exist"));
    }
    let prefix = format!("{}/**/*.pb", import_path.to_str().unwrap());
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
                    page_list.push(obj);
                }
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

    tag::util::resolve_tag_option(&mut tag_list, &tag_opt_list);

    // let file_tar_path = &arg.export_path.to_str().unwrap();
    let mut export_file_path = export_path.to_owned();
    export_file_path.push("files");
    if fs::exists(&export_file_path).is_ok_and(|f| f == false) {
        fs::create_dir_all(&export_file_path)?;
    }

    let file_assets_path = format!("{}/files/*", import_path.to_str().unwrap());
    for entry in glob::glob(&file_assets_path).unwrap().into_iter() {
        let path = entry.ok().unwrap();

        let tar_path = path.canonicalize().unwrap().as_path().to_owned();
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();

        let mut tar_copy_path = export_file_path.clone();
        tar_copy_path.push(file_name);
        if arg.skip_copy == false {
            fs::copy(&tar_path, &tar_copy_path).expect("failed to copy file");
        }

        if path.extension().is_some_and(|f| f == "ipynb") {
            // jupyter_notbook_list
            if let Ok(mut nb_file) = jupyter_notbook::util::read_jupyter_notebook(&tar_path) {
                nb_file.file_url = Some(file_name.to_owned());
                jupyter_notbook_list.push(nb_file);
            }
        }
    }

    for page in page_list.iter_mut() {
        page::util::resolve_page_external(
            page,
            &file_obj_list,
            &bookmark_list,
            &tag_list,
            &jupyter_notbook_list,
        );
    }

    let page_ref_list = &page_list.to_owned();
    for page in page_list.iter_mut() {
        page::util::resolve_page_related_posts(page, page_ref_list);
    }

    // if let Some(coll) = &arg.collections {
    //     for coll in coll {
    //         // let file_dir_name = header_id_resolver(&coll.name, &coll.id);

    //     }
    // }
    export_page_collection(page_list, coll_list, &export_path)?;
    export_series_related(tag_list, page_ref_list, &export_path)?;
    export_tags_related(page_ref_list, &export_path)?;

    Ok(())
}

fn export_page_collection(
    page_list: Vec<Page>,
    coll_list: Vec<Collection>,
    file_tar_path: &PathBuf,
) -> Result<(), anyhow::Error> {
    let mut index_list = vec![];
    for coll in coll_list.iter() {
        let file_dir_name = header_id_resolver(&coll.name, &coll.id);
        let mut coll_dir = file_tar_path.to_owned();
        coll_dir.push(file_dir_name);
        if let Err(e) = fs::create_dir_all(&coll_dir) {
            eprintln!("Failed to create directory: {}", e);
            return Err(anyhow!(e));
        }

        let sub_page = page_list
            .to_owned()
            .into_iter()
            .filter(|p| p.collection_id == coll.id)
            .collect::<Vec<_>>();

        for page in sub_page.iter() {
            // page::util::resolve_page_related_posts(page, page_ref_list);
            let mut page_file_path = coll_dir.to_owned();
            page_file_path.push(format!("{}.json", page.id));

            save_to_file_path(&page, &page_file_path)?;
            index_list.push(page.to_index_reference(coll_dir.to_str().unwrap()));
        }
    }
    let mut index_file_path = file_tar_path.to_owned();
    index_file_path.push("index.json");
    save_to_file_path(&index_list, &index_file_path)?;

    Ok(())
}

fn export_tags_related(page_ref_list: &Vec<Page>, file_tar_path: &PathBuf) -> Result<(), Error> {
    if let Ok(tag_set) = DEFAULT_TAG.lock() {
        let tag_page_list = tag::util::generate_tag_index(&tag_set, page_ref_list);
        let mut post_index_page = vec![];
        let mut tag_path = file_tar_path.to_owned();
        tag_path.push("tags");
        tag_page_list.iter().for_each(|f| {
            let mut tag_page_path = tag_path.clone();
            tag_page_path.push(header_id_resolver(f.name.as_str(), f.id.as_str()));
            tag_page_path.push(format!("p{}.json", f.page_index.unwrap()));

            save_to_file_path(f, &tag_page_path).expect("fail save tag page json");

            if f.page_index.is_some_and(|page_num| page_num == 0) {
                let mut y = f.to_owned();
                let short_list = y.result_list.chunks(3).next().unwrap();
                y.result_list = short_list.to_vec();
                post_index_page.push(y);
            }
        });

        let mut index_file_path = tag_path.clone();
        index_file_path.push("index.json");
        save_to_file_path(&post_index_page, &index_file_path)?;
    }

    Ok(())
}

fn export_series_related(
    tag_list: Vec<Tag>,
    page_ref_list: &Vec<Page>,
    file_tar_path: &PathBuf,
) -> Result<(), Error> {
    if let Some(serie_tag) = tag_list.iter().find(|f| f.name == "Series") {
        let serie_page_list = tag::util::generate_serie_index(serie_tag, page_ref_list);

        let mut post_index_page = vec![];
        let mut tag_path = file_tar_path.to_owned();
        tag_path.push("series");

        serie_page_list.iter().for_each(|f| {
            let mut tag_page_path = tag_path.clone();
            tag_page_path.push(header_id_resolver(&f.name, &f.id));
            tag_page_path.push(format!("p{}.json", f.page_index.unwrap()));
            save_to_file_path(f, &tag_page_path).expect("fail save series page json");

            if f.page_index.is_some_and(|page_num| page_num == 0) {
                let mut y = f.to_owned();
                let short_list = y.result_list.chunks(3).next().unwrap();
                y.result_list = short_list.to_vec();
                post_index_page.push(y);
            }
        });

        let mut index_file_path = tag_path.clone();
        index_file_path.push("index.json");
        save_to_file_path(&post_index_page, &index_file_path)?;
    }

    Ok(())
}
