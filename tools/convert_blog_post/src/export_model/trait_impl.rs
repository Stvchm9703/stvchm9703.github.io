use anyhow::Error;

use crate::proto::anytype::SnapshotWithType;
use crate::proto::anytype::model::Block;

use super::content_block::ContentBlock;

pub trait FromContentBlock<T> {
    fn from_content_block(raw: &ContentBlock) -> Result<T, Error>;
}

pub trait FromRaw<T, O> {
    fn from_raw(raw_obj: &T) -> Result<O, Error>;
}

pub trait FromSnapshotList<T> {
    fn from_snapshot_list(raw_list: Vec<SnapshotWithType>) -> Result<Vec<T>, Error>;
}

pub trait FromBlock<T> {
    // type FromBlockError;
    fn from_block(raw_obj: &Block) -> Result<T, Error>;
    fn from_block_with_idx(raw_obj: &Block, idx: usize) -> Result<T, Error>;
}

pub trait FromBlockContent<Input, Output> {
    // type FromBlockContentError;
    fn from_block_content(raw_obj: &Input) -> Result<Output, Error>;
}

pub trait AddFromExternalFile<T> {
    fn add_from_external_file(&self, raw_obj: &T) -> Result<(), Error>;
}
