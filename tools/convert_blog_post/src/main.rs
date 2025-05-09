mod export_model;
mod proto;
// mod anytype_simplied;
use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf};

// use crate::proto::anytype::object::AnytypeObject;
// use crate::anytype_simplied::{convert_anytype_object, convert_snapshot};
use crate::export_model::{
    common::{ObjectTypes, get_snapshot_data},
    external_link::ExternalBookmarkLink,
    file_object::FileObject,
    page::Page,
    tag::Tag,
    tag_option::TagOption,
};
use crate::proto::anytype::SnapshotWithType;
// use crate::anytype_proto::SnapshotWithType;

use export_model::trait_impl::FromRaw;
use glob;
use proto::anytype::model::{SmartBlockSnapshotBase, SmartBlockType};
use quick_protobuf::MessageRead;
// use prost::{self, Message};

fn main() {
    println!("Hello, world!");
    // test_run();
    test_prost();
}

const ANYTYPE_BASE_PATH: &str = "../../blog_post/Anytype.20250415.122153.2";

const ANYTYPE_PB_PATH: &str = "../../blog_post/Anytype.20250505.160721.07";

fn test_prost() {
    // Test the functionality of the convert_blog_post tool
    // ...
    let prefix = format!("{}/**/*.pb", ANYTYPE_PB_PATH);
    let mut page_list: Vec<Page> = vec![];
    let mut tag_list: Vec<Tag> = vec![];
    let mut tag_opt_list: Vec<TagOption> = vec![];
    let mut file_obj_list: Vec<FileObject> = vec![];

    for entry in glob::glob(&prefix).unwrap().into_iter() {
        // Process each file here
        let path = entry.ok().unwrap();
        println!("path {:?}", path);
        // let fil = parse_from_pb_file(&path).unwrap();

        let file_bytes = std::fs::read(path).unwrap();

        let mut reader = quick_protobuf::BytesReader::from_bytes(&file_bytes.clone());
        let fil: SnapshotWithType =
            SnapshotWithType::from_reader(&mut reader, &file_bytes).unwrap();
        // println!("fil:{:?}", fil);

        if let Ok(snapshot_data) = get_snapshot_data(&fil) {
            if fil.sbType == SmartBlockType::Page
                && has_object_type(&snapshot_data, ObjectTypes::Page)
            {
                let inited = Page::from_raw(&fil);
                if let Ok(page) = inited {
                    println!("{:#?}", page);
                    page_list.push(page);
                } else {
                    let e = inited.err();
                    println!("err : {:?}", e);
                }
            }

            if fil.sbType == SmartBlockType::STRelation {
                if let Ok(tag) = Tag::from_raw(&fil) {
                    tag_list.push(tag);
                }
            }

            if fil.sbType == SmartBlockType::STRelationOption {
                if let Ok(tag_opt) = TagOption::from_raw(&fil) {
                    tag_opt_list.push(tag_opt);
                }
            }

            if fil.sbType == SmartBlockType::File || fil.sbType == SmartBlockType::FileObject {
                if let Ok(obj) = FileObject::from_raw(&fil) {
                    file_obj_list.push(obj);
                }
            }
        }

        // for test print
        // if let Some(shot) = fil.snapshot {
        //     if let Some(data) = shot.data {
        //         for block in data.blocks {
        //             println!("blk : {:?}", block.id);
        //             if let Some(fields) = block.fields {
        //                 println!("  field : {:?}", serde_json::to_value(&fields))
        //             }
        //         }
        //     }
        // }
        // println!();

        // file_objs.push(fil);
    }

    // convert_snapshot(&file_objs);

    // convert_anytype_object(&file_objs);
}

// fn parse_from_pb_file<'a>(
//     file_path: &'a PathBuf,
// ) -> Result<SnapshotWithType<'a>, Box<dyn std::error::Error>> {

//     // let mut inner = reader.inner().to_owned();
//     // let buffer = reader.buffer();
//     let test_file: SnapshotWithType = SnapshotWithType::from_reader(&mut reader, &'a bytes)?;
//     // println!("test_file : {:#?}", test_file.to_owned());

//     return Ok(test_file);
// }

fn has_object_type(snapshot_data: &SmartBlockSnapshotBase, check_type: ObjectTypes) -> bool {
    snapshot_data
        .objectTypes
        .iter()
        .any(|x| x == check_type.to_str())
}
