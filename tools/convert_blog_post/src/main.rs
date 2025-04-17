mod anytype;
mod anytype_simplied;
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use crate::anytype::{
    enum_set::ObjectType,
    object::{AnytypeObject, Data},
};

use crate::anytype_simplied::convert_anytype_object;

use glob;

fn main() {
    println!("Hello, world!");
    test_run();
}

const ANYTYPE_BASE_PATH: &str =
    "/Users/stephencheng/git_src/stvchm9703/blog_post/Anytype.20250415.122153.2";

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
