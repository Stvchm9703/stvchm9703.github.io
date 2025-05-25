use anyhow::Error;

use crate::proto::anytype::SnapshotWithType;
use crate::proto::anytype::model::Block as RawBlock;

use super::content_block::ContentBlock;
use super::file_object::FileObject;

#[warn(dead_code)]
pub trait FromContentBlock: Sized {
    fn from_content_block(raw: &ContentBlock) -> Result<Self, Error>;
}

pub trait FromRaw<T>: Sized {
    fn from_raw(raw_obj: &T) -> Result<Self, Error>;
}

#[warn(dead_code)]
pub trait FromSnapshotList: Sized {
    fn from_snapshot_list(raw_list: Vec<SnapshotWithType>) -> Result<Vec<Self>, Error>;
}

pub trait FromBlock: Sized {
    // type FromBlockError;
    fn from_block(raw_obj: &RawBlock) -> Result<Self, Error>;
    fn from_block_with_idx(raw_obj: &RawBlock, idx: usize) -> Result<Self, Error>;
}

pub trait FromBlockContent<T>: Sized {
    // type FromBlockContentError;
    fn from_block_content(raw_obj: &T) -> Result<Self, Error>;
}

#[warn(dead_code)]
pub trait AddFromExternalFile {
    fn add_from_external_file(&self, raw_obj: &FileObject) -> Result<(), Error>;
}
