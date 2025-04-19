mod anytype;
mod anytype_proto;
mod anytype_simplied;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{BufReader, Bytes},
    path::PathBuf,
};

use crate::anytype::{
    enum_set::ObjectType,
    object::{AnytypeObject, Data},
};

use crate::anytype_simplied::{convert_anytype_object, convert_snapshot};
use serde_json::to_string;

use crate::anytype_proto::anytype::SnapshotWithType;

use glob;
use prost::{self, Message};

fn main() {
    println!("Hello, world!");
    // test_run();
    test_prost();
}

const ANYTYPE_BASE_PATH: &str = "../../blog_post/Anytype.20250415.122153.2";

const ANYTYPE_PB_PATH: &str = "../../blog_post/Anytype.20250418.013727.47";

fn test_run() {
    // Test the functionality of the convert_blog_post tool
    // ...
    let prefix = format!("{}/**/*.json", ANYTYPE_BASE_PATH);
    let mut file_objs: Vec<AnytypeObject> = vec![];

    // let mut data_objs: BTreeMap<String, Data> = BTreeMap::new();

    for entry in glob::glob(&prefix).unwrap().into_iter() {
        // Process each file here
        //
        let path = entry.ok().unwrap();
        let mut fil = parse_json_file(&path).unwrap();
        file_objs.push(fil);
        // let snapshot = fil.snapshot;
        // if let Some(data) = snapshot.data {
        // let id = snapshot.data.details.id.to_owned();
        // data_objs.insert(id, snapshot.data);
        // }
    }

    convert_anytype_object(&file_objs);
}

fn parse_json_file(file_path: &PathBuf) -> Result<AnytypeObject, Box<dyn std::error::Error>> {
    // Parse the JSON file and convert it to a blog post
    // ...
    //
    println!("parse json : {}", file_path.display());
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    // Return the `User`.
    Ok(u)
}

fn test_prost() {
    // Test the functionality of the convert_blog_post tool
    // ...
    let prefix = format!("{}/**/*.pb", ANYTYPE_PB_PATH);
    let mut file_objs: Vec<SnapshotWithType> = vec![];

    // let mut data_objs: BTreeMap<String, Data> = BTreeMap::new();

    for entry in glob::glob(&prefix).unwrap().into_iter() {
        // Process each file here
        //
        let path = entry.ok().unwrap();
        println!("path {:?}", path);
        let fil = parse_from_pb_file(&path).unwrap();
        file_objs.push(fil);
        // let snapshot = fil.snapshot;
        // if let Some(data) = snapshot.data {
        // let id = snapshot.data.details.id.to_owned();
        // data_objs.insert(id, snapshot.data);
        // }
    }

    // println!("file {:#?}", file_objs[0]);
    convert_snapshot(&file_objs);

    // convert_anytype_object(&file_objs);
}

fn parse_from_pb_file(file_path: &PathBuf) -> Result<SnapshotWithType, Box<dyn std::error::Error>> {
    let buffer = std::fs::read(file_path)?;
    // let mut buf = Bytes::from(buffer);
    let mut buf = prost::bytes::Bytes::from(buffer);

    let test_file: SnapshotWithType = SnapshotWithType::decode(buf.as_ref())?;
    // println!("test_file {:#?}", test_file);
    // assert_eq!(test_file.data.first().unwrap().hello, "hello world 1");
    Ok(test_file)
}
