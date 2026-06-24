// Automatically generated mod.rs


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]



use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;
pub use crate::proto::anytype;
pub use crate::proto::google;



// models.proto -> the `model` namespace (file: models.rs).
// Referenced throughout the crate and the sibling proto modules as `anytype::model::*`.
#[path = "models.rs"]
pub mod model;

// changes.proto, events.proto and snapshot.proto all live directly in the
// `anytype` namespace, so their items are re-exported flat (e.g. `anytype::ModifyOp`,
// `anytype::mod_Event`, `anytype::SnapshotWithType`).
mod changes;
pub use changes::*;

mod events;
pub use events::*;

#[path = "anytype.rs"]
mod snapshot;
pub use snapshot::*;
