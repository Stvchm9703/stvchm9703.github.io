use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use serde::Serialize;

pub fn save_to_file<T: Serialize>(item: &T, fname: &str) {
    let list_as_json = serde_json::to_string_pretty(item).unwrap();

    let path = Path::new(fname);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    if fs::exists(fname).is_ok_and(|f| f) {
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
