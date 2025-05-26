use anyhow::{Error, Result};
use serde::Serialize;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

/**
pub fn save_to_file<T: Serialize>(item: &T, fname: &str) -> Result<(), Error> {
    let list_as_json = serde_json::to_string_pretty(item).unwrap();

    let path = Path::new(fname);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
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

    Ok(())
}
 */
pub fn save_to_file_path<T: Serialize>(item: &T, fpath: &PathBuf) -> Result<(), Error> {
    let serialized = serde_json::to_string_pretty(item).unwrap();

    // let path = Path::new(fname);
    if let Some(parent) = fpath.parent() {
        fs::create_dir_all(parent)?;
    }

    if fs::exists(fpath).is_ok_and(|f| f) {
        fs::remove_file(fpath).expect("cannot clear the file")
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(fpath)
        .expect("Could not touch file!");
    // File::create(fname).expect("Could not create file!");

    file.write_all(serialized.as_bytes())
        .expect("Cannot write to the file!");

    Ok(())
}
