// Automatically generated mod.rs


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]



pub mod model;



use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;
use crate::proto::anytype;



// Automatically generated rust module for 'changes.proto' file

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ModifyOp {
    Set = 0,
    Unset = 1,
    Inc = 2,
    AddToSet = 3,
    Pull = 4,
}

impl Default for ModifyOp {
    fn default() -> Self {
        ModifyOp::Set
    }
}

impl From<i32> for ModifyOp {
    fn from(i: i32) -> Self {
        match i {
            0 => ModifyOp::Set,
            1 => ModifyOp::Unset,
            2 => ModifyOp::Inc,
            3 => ModifyOp::AddToSet,
            4 => ModifyOp::Pull,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ModifyOp {
    fn from(s: &'a str) -> Self {
        match s {
            "Set" => ModifyOp::Set,
            "Unset" => ModifyOp::Unset,
            "Inc" => ModifyOp::Inc,
            "AddToSet" => ModifyOp::AddToSet,
            "Pull" => ModifyOp::Pull,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Change<'a> {
    pub content: Vec<mod_Change::Content<'a>>,
    pub snapshot: Option<mod_Change::Snapshot<'a>>,
    pub fileKeys: Vec<mod_Change::FileKeys<'a>>,
    pub timestamp: i64,
    pub version: u32,
    pub changeType: u32,
}

impl<'a> MessageRead<'a> for Change<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.content.push(r.read_message::<mod_Change::Content>(bytes)?),
                Ok(34) => msg.snapshot = Some(r.read_message::<mod_Change::Snapshot>(bytes)?),
                Ok(50) => msg.fileKeys.push(r.read_message::<mod_Change::FileKeys>(bytes)?),
                Ok(56) => msg.timestamp = r.read_int64(bytes)?,
                Ok(64) => msg.version = r.read_uint32(bytes)?,
                Ok(72) => msg.changeType = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Change<'a> {
    fn get_size(&self) -> usize {
        0
        + self.content.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.snapshot.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileKeys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.timestamp == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.changeType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.changeType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.content { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.snapshot { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.fileKeys { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.timestamp != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.timestamp))?; }
        if self.version != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.version))?; }
        if self.changeType != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.changeType))?; }
        Ok(())
    }
}

pub mod mod_Change {

use std::borrow::Cow;
use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Snapshot<'a> {
    pub logHeads: KVMap<Cow<'a, str>, Cow<'a, str>>,
    pub data: Option<anytype::model::SmartBlockSnapshotBase<'a>>,
    pub fileKeys: Vec<mod_Change::FileKeys<'a>>,
}

impl<'a> MessageRead<'a> for Snapshot<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?))?;
                    msg.logHeads.insert(key, value);
                }
                Ok(18) => msg.data = Some(r.read_message::<anytype::model::SmartBlockSnapshotBase>(bytes)?),
                Ok(26) => msg.fileKeys.push(r.read_message::<mod_Change::FileKeys>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Snapshot<'a> {
    fn get_size(&self) -> usize {
        0
        + self.logHeads.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fileKeys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.logHeads.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        if let Some(ref s) = self.data { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.fileKeys { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileKeys<'a> {
    pub hash: Cow<'a, str>,
    pub keys: KVMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for FileKeys<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?))?;
                    msg.keys.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileKeys<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.hash == "" { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + self.keys.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).len()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != "" { w.write_with_tag(10, |w| w.write_string(&**&self.hash))?; }
        for (k, v) in self.keys.iter() { w.write_with_tag(18, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).len()), 10, |w| w.write_string(&**k), 18, |w| w.write_string(&**v)))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Content<'a> {
    pub value: mod_Change::mod_Content::OneOfvalue<'a>,
}

impl<'a> MessageRead<'a> for Content<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = mod_Change::mod_Content::OneOfvalue::blockCreate(r.read_message::<mod_Change::BlockCreate>(bytes)?),
                Ok(18) => msg.value = mod_Change::mod_Content::OneOfvalue::blockUpdate(r.read_message::<mod_Change::BlockUpdate>(bytes)?),
                Ok(26) => msg.value = mod_Change::mod_Content::OneOfvalue::blockRemove(r.read_message::<mod_Change::BlockRemove>(bytes)?),
                Ok(34) => msg.value = mod_Change::mod_Content::OneOfvalue::blockMove(r.read_message::<mod_Change::BlockMove>(bytes)?),
                Ok(42) => msg.value = mod_Change::mod_Content::OneOfvalue::blockDuplicate(r.read_message::<mod_Change::BlockDuplicate>(bytes)?),
                Ok(402) => msg.value = mod_Change::mod_Content::OneOfvalue::relationAdd(r.read_message::<mod_Change::RelationAdd>(bytes)?),
                Ok(410) => msg.value = mod_Change::mod_Content::OneOfvalue::relationRemove(r.read_message::<mod_Change::RelationRemove>(bytes)?),
                Ok(802) => msg.value = mod_Change::mod_Content::OneOfvalue::detailsSet(r.read_message::<mod_Change::DetailsSet>(bytes)?),
                Ok(810) => msg.value = mod_Change::mod_Content::OneOfvalue::detailsUnset(r.read_message::<mod_Change::DetailsUnset>(bytes)?),
                Ok(842) => msg.value = mod_Change::mod_Content::OneOfvalue::objectTypeAdd(r.read_message::<mod_Change::ObjectTypeAdd>(bytes)?),
                Ok(850) => msg.value = mod_Change::mod_Content::OneOfvalue::objectTypeRemove(r.read_message::<mod_Change::ObjectTypeRemove>(bytes)?),
                Ok(858) => msg.value = mod_Change::mod_Content::OneOfvalue::storeKeySet(r.read_message::<mod_Change::StoreKeySet>(bytes)?),
                Ok(866) => msg.value = mod_Change::mod_Content::OneOfvalue::storeKeyUnset(r.read_message::<mod_Change::StoreKeyUnset>(bytes)?),
                Ok(874) => msg.value = mod_Change::mod_Content::OneOfvalue::storeSliceUpdate(r.read_message::<mod_Change::StoreSliceUpdate>(bytes)?),
                Ok(882) => msg.value = mod_Change::mod_Content::OneOfvalue::originalCreatedTimestampSet(r.read_message::<mod_Change::OriginalCreatedTimestampSet>(bytes)?),
                Ok(890) => msg.value = mod_Change::mod_Content::OneOfvalue::setFileInfo(r.read_message::<mod_Change::SetFileInfo>(bytes)?),
                Ok(898) => msg.value = mod_Change::mod_Content::OneOfvalue::notificationCreate(r.read_message::<mod_Change::NotificationCreate>(bytes)?),
                Ok(906) => msg.value = mod_Change::mod_Content::OneOfvalue::notificationUpdate(r.read_message::<mod_Change::NotificationUpdate>(bytes)?),
                Ok(914) => msg.value = mod_Change::mod_Content::OneOfvalue::deviceAdd(r.read_message::<mod_Change::DeviceAdd>(bytes)?),
                Ok(922) => msg.value = mod_Change::mod_Content::OneOfvalue::deviceUpdate(r.read_message::<mod_Change::DeviceUpdate>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Content<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.value {
            mod_Change::mod_Content::OneOfvalue::blockCreate(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::blockUpdate(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::blockRemove(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::blockMove(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::blockDuplicate(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::relationAdd(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::relationRemove(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::detailsSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::detailsUnset(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::objectTypeAdd(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::objectTypeRemove(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::storeKeySet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::storeKeyUnset(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::storeSliceUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::originalCreatedTimestampSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::setFileInfo(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::notificationCreate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::notificationUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::deviceAdd(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::deviceUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Change::mod_Content::OneOfvalue::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.value {            mod_Change::mod_Content::OneOfvalue::blockCreate(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::blockUpdate(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::blockRemove(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::blockMove(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::blockDuplicate(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::relationAdd(ref m) => { w.write_with_tag(402, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::relationRemove(ref m) => { w.write_with_tag(410, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::detailsSet(ref m) => { w.write_with_tag(802, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::detailsUnset(ref m) => { w.write_with_tag(810, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::objectTypeAdd(ref m) => { w.write_with_tag(842, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::objectTypeRemove(ref m) => { w.write_with_tag(850, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::storeKeySet(ref m) => { w.write_with_tag(858, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::storeKeyUnset(ref m) => { w.write_with_tag(866, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::storeSliceUpdate(ref m) => { w.write_with_tag(874, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::originalCreatedTimestampSet(ref m) => { w.write_with_tag(882, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::setFileInfo(ref m) => { w.write_with_tag(890, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::notificationCreate(ref m) => { w.write_with_tag(898, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::notificationUpdate(ref m) => { w.write_with_tag(906, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::deviceAdd(ref m) => { w.write_with_tag(914, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::deviceUpdate(ref m) => { w.write_with_tag(922, |w| w.write_message(m))? },
            mod_Change::mod_Content::OneOfvalue::None => {},
    }        Ok(())
    }
}

pub mod mod_Content {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfvalue<'a> {
    blockCreate(mod_Change::BlockCreate<'a>),
    blockUpdate(mod_Change::BlockUpdate<'a>),
    blockRemove(mod_Change::BlockRemove<'a>),
    blockMove(mod_Change::BlockMove<'a>),
    blockDuplicate(mod_Change::BlockDuplicate<'a>),
    relationAdd(mod_Change::RelationAdd<'a>),
    relationRemove(mod_Change::RelationRemove<'a>),
    detailsSet(mod_Change::DetailsSet<'a>),
    detailsUnset(mod_Change::DetailsUnset<'a>),
    objectTypeAdd(mod_Change::ObjectTypeAdd<'a>),
    objectTypeRemove(mod_Change::ObjectTypeRemove<'a>),
    storeKeySet(mod_Change::StoreKeySet<'a>),
    storeKeyUnset(mod_Change::StoreKeyUnset<'a>),
    storeSliceUpdate(mod_Change::StoreSliceUpdate<'a>),
    originalCreatedTimestampSet(mod_Change::OriginalCreatedTimestampSet),
    setFileInfo(mod_Change::SetFileInfo<'a>),
    notificationCreate(mod_Change::NotificationCreate<'a>),
    notificationUpdate(mod_Change::NotificationUpdate<'a>),
    deviceAdd(mod_Change::DeviceAdd<'a>),
    deviceUpdate(mod_Change::DeviceUpdate<'a>),
    None,
}

impl<'a> Default for OneOfvalue<'a> {
    fn default() -> Self {
        OneOfvalue::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockCreate<'a> {
    pub targetId: Cow<'a, str>,
    pub position: anytype::model::mod_Block::Position,
    pub blocks: Vec<anytype::model::Block<'a>>,
}

impl<'a> MessageRead<'a> for BlockCreate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.targetId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.position = r.read_enum(bytes)?,
                Ok(26) => msg.blocks.push(r.read_message::<anytype::model::Block>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockCreate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.targetId == "" { 0 } else { 1 + sizeof_len((&self.targetId).len()) }
        + if self.position == anytype::model::mod_Block::Position::None { 0 } else { 1 + sizeof_varint(*(&self.position) as u64) }
        + self.blocks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.targetId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.targetId))?; }
        if self.position != anytype::model::mod_Block::Position::None { w.write_with_tag(16, |w| w.write_enum(*&self.position as i32))?; }
        for s in &self.blocks { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockUpdate<'a> {
    pub events: Vec<anytype::mod_Event::Message<'a>>,
}

impl<'a> MessageRead<'a> for BlockUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.events.push(r.read_message::<anytype::mod_Event::Message>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + self.events.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.events { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockRemove<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BlockRemove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockRemove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockMove<'a> {
    pub targetId: Cow<'a, str>,
    pub position: anytype::model::mod_Block::Position,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BlockMove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.targetId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.position = r.read_enum(bytes)?,
                Ok(26) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockMove<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.targetId == "" { 0 } else { 1 + sizeof_len((&self.targetId).len()) }
        + if self.position == anytype::model::mod_Block::Position::None { 0 } else { 1 + sizeof_varint(*(&self.position) as u64) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.targetId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.targetId))?; }
        if self.position != anytype::model::mod_Block::Position::None { w.write_with_tag(16, |w| w.write_enum(*&self.position as i32))?; }
        for s in &self.ids { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockDuplicate<'a> {
    pub targetId: Cow<'a, str>,
    pub position: anytype::model::mod_Block::Position,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for BlockDuplicate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.targetId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.position = r.read_enum(bytes)?,
                Ok(26) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockDuplicate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.targetId == "" { 0 } else { 1 + sizeof_len((&self.targetId).len()) }
        + if self.position == anytype::model::mod_Block::Position::None { 0 } else { 1 + sizeof_varint(*(&self.position) as u64) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.targetId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.targetId))?; }
        if self.position != anytype::model::mod_Block::Position::None { w.write_with_tag(16, |w| w.write_enum(*&self.position as i32))?; }
        for s in &self.ids { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DetailsSet<'a> {
    pub key: Cow<'a, str>,
    pub value: Option<google::protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for DetailsSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = Some(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DetailsSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DetailsUnset<'a> {
    pub key: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DetailsUnset<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DetailsUnset<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationAdd<'a> {
    pub relationLinks: Vec<anytype::model::RelationLink<'a>>,
}

impl<'a> MessageRead<'a> for RelationAdd<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.relationLinks.push(r.read_message::<anytype::model::RelationLink>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationAdd<'a> {
    fn get_size(&self) -> usize {
        0
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.relationLinks { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationRemove<'a> {
    pub relationKey: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for RelationRemove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.relationKey.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationRemove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.relationKey.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.relationKey { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectTypeAdd<'a> {
    pub url: Cow<'a, str>,
    pub key: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ObjectTypeAdd<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjectTypeAdd<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.url != "" { w.write_with_tag(10, |w| w.write_string(&**&self.url))?; }
        if self.key != "" { w.write_with_tag(18, |w| w.write_string(&**&self.key))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectTypeRemove<'a> {
    pub url: Cow<'a, str>,
    pub key: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ObjectTypeRemove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjectTypeRemove<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.url != "" { w.write_with_tag(10, |w| w.write_string(&**&self.url))?; }
        if self.key != "" { w.write_with_tag(18, |w| w.write_string(&**&self.key))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoreKeySet<'a> {
    pub path: Vec<Cow<'a, str>>,
    pub value: Option<google::protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for StoreKeySet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.value = Some(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StoreKeySet<'a> {
    fn get_size(&self) -> usize {
        0
        + self.path.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.path { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoreKeyUnset<'a> {
    pub path: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for StoreKeyUnset<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StoreKeyUnset<'a> {
    fn get_size(&self) -> usize {
        0
        + self.path.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.path { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoreSliceUpdate<'a> {
    pub key: Cow<'a, str>,
    pub operation: mod_Change::mod_StoreSliceUpdate::OneOfoperation<'a>,
}

impl<'a> MessageRead<'a> for StoreSliceUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.operation = mod_Change::mod_StoreSliceUpdate::OneOfoperation::add(r.read_message::<mod_Change::mod_StoreSliceUpdate::Add>(bytes)?),
                Ok(26) => msg.operation = mod_Change::mod_StoreSliceUpdate::OneOfoperation::remove(r.read_message::<mod_Change::mod_StoreSliceUpdate::Remove>(bytes)?),
                Ok(34) => msg.operation = mod_Change::mod_StoreSliceUpdate::OneOfoperation::move_pb(r.read_message::<mod_Change::mod_StoreSliceUpdate::Move>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StoreSliceUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + match self.operation {
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::add(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::remove(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::move_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        match self.operation {            mod_Change::mod_StoreSliceUpdate::OneOfoperation::add(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::remove(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::move_pb(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Change::mod_StoreSliceUpdate::OneOfoperation::None => {},
    }        Ok(())
    }
}

pub mod mod_StoreSliceUpdate {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub afterId: Cow<'a, str>,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.ids { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Move<'a> {
    pub afterId: Cow<'a, str>,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Move<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Move<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.ids { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoperation<'a> {
    add(mod_Change::mod_StoreSliceUpdate::Add<'a>),
    remove(mod_Change::mod_StoreSliceUpdate::Remove<'a>),
    move_pb(mod_Change::mod_StoreSliceUpdate::Move<'a>),
    None,
}

impl<'a> Default for OneOfoperation<'a> {
    fn default() -> Self {
        OneOfoperation::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OriginalCreatedTimestampSet {
    pub ts: i64,
}

impl<'a> MessageRead<'a> for OriginalCreatedTimestampSet {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.ts = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OriginalCreatedTimestampSet {
    fn get_size(&self) -> usize {
        0
        + if self.ts == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.ts) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.ts != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.ts))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SetFileInfo<'a> {
    pub fileInfo: Option<anytype::model::FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for SetFileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileInfo = Some(r.read_message::<anytype::model::FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SetFileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fileInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fileInfo { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotificationCreate<'a> {
    pub notification: Option<anytype::model::Notification<'a>>,
}

impl<'a> MessageRead<'a> for NotificationCreate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.notification = Some(r.read_message::<anytype::model::Notification>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotificationCreate<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notification.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.notification { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct NotificationUpdate<'a> {
    pub id: Cow<'a, str>,
    pub status: anytype::model::mod_Notification::Status,
}

impl<'a> MessageRead<'a> for NotificationUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.status = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for NotificationUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.status == anytype::model::mod_Notification::Status::Created { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.status != anytype::model::mod_Notification::Status::Created { w.write_with_tag(16, |w| w.write_enum(*&self.status as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceAdd<'a> {
    pub device: Option<anytype::model::DeviceInfo<'a>>,
}

impl<'a> MessageRead<'a> for DeviceAdd<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.device = Some(r.read_message::<anytype::model::DeviceInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceAdd<'a> {
    fn get_size(&self) -> usize {
        0
        + self.device.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.device { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DeviceUpdate<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DeviceUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChangeNoSnapshot<'a> {
    pub content: Vec<mod_Change::Content<'a>>,
    pub fileKeys: Vec<mod_Change::FileKeys<'a>>,
    pub timestamp: i64,
    pub version: u32,
    pub changeType: u32,
}

impl<'a> MessageRead<'a> for ChangeNoSnapshot<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(26) => msg.content.push(r.read_message::<mod_Change::Content>(bytes)?),
                Ok(50) => msg.fileKeys.push(r.read_message::<mod_Change::FileKeys>(bytes)?),
                Ok(56) => msg.timestamp = r.read_int64(bytes)?,
                Ok(64) => msg.version = r.read_uint32(bytes)?,
                Ok(72) => msg.changeType = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChangeNoSnapshot<'a> {
    fn get_size(&self) -> usize {
        0
        + self.content.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.fileKeys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.timestamp == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.timestamp) as u64) }
        + if self.version == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.version) as u64) }
        + if self.changeType == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.changeType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.content { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.fileKeys { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.timestamp != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.timestamp))?; }
        if self.version != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.version))?; }
        if self.changeType != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.changeType))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoreChange<'a> {
    pub changeSet: Vec<StoreChangeContent<'a>>,
}

impl<'a> MessageRead<'a> for StoreChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.changeSet.push(r.read_message::<StoreChangeContent>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StoreChange<'a> {
    fn get_size(&self) -> usize {
        0
        + self.changeSet.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.changeSet { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct StoreChangeContent<'a> {
    pub change: mod_StoreChangeContent::OneOfchange<'a>,
}

impl<'a> MessageRead<'a> for StoreChangeContent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.change = mod_StoreChangeContent::OneOfchange::create(r.read_message::<DocumentCreate>(bytes)?),
                Ok(18) => msg.change = mod_StoreChangeContent::OneOfchange::modify(r.read_message::<DocumentModify>(bytes)?),
                Ok(26) => msg.change = mod_StoreChangeContent::OneOfchange::delete(r.read_message::<DocumentDelete>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for StoreChangeContent<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.change {
            mod_StoreChangeContent::OneOfchange::create(ref m) => 1 + sizeof_len((m).get_size()),
            mod_StoreChangeContent::OneOfchange::modify(ref m) => 1 + sizeof_len((m).get_size()),
            mod_StoreChangeContent::OneOfchange::delete(ref m) => 1 + sizeof_len((m).get_size()),
            mod_StoreChangeContent::OneOfchange::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.change {            mod_StoreChangeContent::OneOfchange::create(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_StoreChangeContent::OneOfchange::modify(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_StoreChangeContent::OneOfchange::delete(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_StoreChangeContent::OneOfchange::None => {},
    }        Ok(())
    }
}

pub mod mod_StoreChangeContent {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfchange<'a> {
    create(DocumentCreate<'a>),
    modify(DocumentModify<'a>),
    delete(DocumentDelete<'a>),
    None,
}

impl<'a> Default for OneOfchange<'a> {
    fn default() -> Self {
        OneOfchange::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DocumentCreate<'a> {
    pub collection: Cow<'a, str>,
    pub documentId: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DocumentCreate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.collection = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.documentId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DocumentCreate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.collection == "" { 0 } else { 1 + sizeof_len((&self.collection).len()) }
        + if self.documentId == "" { 0 } else { 1 + sizeof_len((&self.documentId).len()) }
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.collection != "" { w.write_with_tag(10, |w| w.write_string(&**&self.collection))?; }
        if self.documentId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.documentId))?; }
        if self.value != "" { w.write_with_tag(26, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DocumentModify<'a> {
    pub collection: Cow<'a, str>,
    pub documentId: Cow<'a, str>,
    pub keys: Vec<KeyModify<'a>>,
}

impl<'a> MessageRead<'a> for DocumentModify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.collection = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.documentId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.keys.push(r.read_message::<KeyModify>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DocumentModify<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.collection == "" { 0 } else { 1 + sizeof_len((&self.collection).len()) }
        + if self.documentId == "" { 0 } else { 1 + sizeof_len((&self.documentId).len()) }
        + self.keys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.collection != "" { w.write_with_tag(10, |w| w.write_string(&**&self.collection))?; }
        if self.documentId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.documentId))?; }
        for s in &self.keys { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeyModify<'a> {
    pub keyPath: Vec<Cow<'a, str>>,
    pub modifyOp: ModifyOp,
    pub modifyValue: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for KeyModify<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.keyPath.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(24) => msg.modifyOp = r.read_enum(bytes)?,
                Ok(34) => msg.modifyValue = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for KeyModify<'a> {
    fn get_size(&self) -> usize {
        0
        + self.keyPath.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.modifyOp == anytype::ModifyOp::Set { 0 } else { 1 + sizeof_varint(*(&self.modifyOp) as u64) }
        + if self.modifyValue == "" { 0 } else { 1 + sizeof_len((&self.modifyValue).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.keyPath { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.modifyOp != anytype::ModifyOp::Set { w.write_with_tag(24, |w| w.write_enum(*&self.modifyOp as i32))?; }
        if self.modifyValue != "" { w.write_with_tag(34, |w| w.write_string(&**&self.modifyValue))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DocumentDelete<'a> {
    pub collection: Cow<'a, str>,
    pub documentId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for DocumentDelete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.collection = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.documentId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DocumentDelete<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.collection == "" { 0 } else { 1 + sizeof_len((&self.collection).len()) }
        + if self.documentId == "" { 0 } else { 1 + sizeof_len((&self.documentId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.collection != "" { w.write_with_tag(10, |w| w.write_string(&**&self.collection))?; }
        if self.documentId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.documentId))?; }
        Ok(())
    }
}




/// end-of changes.proto


/// from anytype.proto

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SnapshotWithType<'a> {
    pub sbType: anytype::model::SmartBlockType,
    pub snapshot: Option<anytype::mod_Change::Snapshot<'a>>,
}

impl<'a> MessageRead<'a> for SnapshotWithType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.sbType = r.read_enum(bytes)?,
                Ok(18) => msg.snapshot = Some(r.read_message::<anytype::mod_Change::Snapshot>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SnapshotWithType<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.sbType == anytype::model::SmartBlockType::AccountOld { 0 } else { 1 + sizeof_varint(*(&self.sbType) as u64) }
        + self.snapshot.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.sbType != anytype::model::SmartBlockType::AccountOld { w.write_with_tag(8, |w| w.write_enum(*&self.sbType as i32))?; }
        if let Some(ref s) = self.snapshot { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Profile<'a> {
    pub name: Cow<'a, str>,
    pub avatar: Cow<'a, str>,
    pub address: Cow<'a, str>,
    pub spaceDashboardId: Cow<'a, str>,
    pub profileId: Cow<'a, str>,
    pub analyticsId: Cow<'a, str>,
    pub startingPage: Cow<'a, str>,
    pub widgets: Vec<anytype::WidgetBlock<'a>>,
}

impl<'a> MessageRead<'a> for Profile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.avatar = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.address = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.spaceDashboardId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.profileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.analyticsId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.startingPage = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.widgets.push(r.read_message::<anytype::WidgetBlock>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Profile<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.avatar == "" { 0 } else { 1 + sizeof_len((&self.avatar).len()) }
        + if self.address == "" { 0 } else { 1 + sizeof_len((&self.address).len()) }
        + if self.spaceDashboardId == "" { 0 } else { 1 + sizeof_len((&self.spaceDashboardId).len()) }
        + if self.profileId == "" { 0 } else { 1 + sizeof_len((&self.profileId).len()) }
        + if self.analyticsId == "" { 0 } else { 1 + sizeof_len((&self.analyticsId).len()) }
        + if self.startingPage == "" { 0 } else { 1 + sizeof_len((&self.startingPage).len()) }
        + self.widgets.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.avatar != "" { w.write_with_tag(18, |w| w.write_string(&**&self.avatar))?; }
        if self.address != "" { w.write_with_tag(34, |w| w.write_string(&**&self.address))?; }
        if self.spaceDashboardId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceDashboardId))?; }
        if self.profileId != "" { w.write_with_tag(50, |w| w.write_string(&**&self.profileId))?; }
        if self.analyticsId != "" { w.write_with_tag(58, |w| w.write_string(&**&self.analyticsId))?; }
        if self.startingPage != "" { w.write_with_tag(66, |w| w.write_string(&**&self.startingPage))?; }
        for s in &self.widgets { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WidgetBlock<'a> {
    pub layout: anytype::model::mod_Block::mod_Content::mod_Widget::Layout,
    pub targetObjectId: Cow<'a, str>,
    pub objectLimit: i32,
}

impl<'a> MessageRead<'a> for WidgetBlock<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.layout = r.read_enum(bytes)?,
                Ok(18) => msg.targetObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.objectLimit = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for WidgetBlock<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.layout == anytype::model::mod_Block::mod_Content::mod_Widget::Layout::Link { 0 } else { 1 + sizeof_varint(*(&self.layout) as u64) }
        + if self.targetObjectId == "" { 0 } else { 1 + sizeof_len((&self.targetObjectId).len()) }
        + if self.objectLimit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.objectLimit) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.layout != anytype::model::mod_Block::mod_Content::mod_Widget::Layout::Link { w.write_with_tag(8, |w| w.write_enum(*&self.layout as i32))?; }
        if self.targetObjectId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.targetObjectId))?; }
        if self.objectLimit != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.objectLimit))?; }
        Ok(())
    }
}

/// end-of anytype.proto



// Automatically generated rust module for 'events.proto' file


#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Event<'a> {
    pub messages: Vec<mod_Event::Message<'a>>,
    pub contextId: Cow<'a, str>,
    pub initiator: Option<anytype::model::Account<'a>>,
    pub traceId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Event<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.messages.push(r.read_message::<mod_Event::Message>(bytes)?),
                Ok(18) => msg.contextId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.initiator = Some(r.read_message::<anytype::model::Account>(bytes)?),
                Ok(34) => msg.traceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Event<'a> {
    fn get_size(&self) -> usize {
        0
        + self.messages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.contextId == "" { 0 } else { 1 + sizeof_len((&self.contextId).len()) }
        + self.initiator.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.traceId == "" { 0 } else { 1 + sizeof_len((&self.traceId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.messages { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.contextId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.contextId))?; }
        if let Some(ref s) = self.initiator { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.traceId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.traceId))?; }
        Ok(())
    }
}

pub mod mod_Event {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Message<'a> {
    pub spaceId: Cow<'a, str>,
    pub value: mod_Event::mod_Message::OneOfvalue<'a>,
}

impl<'a> MessageRead<'a> for Message<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(1058) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(10) => msg.value = mod_Event::mod_Message::OneOfvalue::accountShow(r.read_message::<mod_Event::mod_Account::Show>(bytes)?),
                Ok(1610) => msg.value = mod_Event::mod_Message::OneOfvalue::accountDetails(r.read_message::<mod_Event::mod_Account::Details>(bytes)?),
                Ok(1618) => msg.value = mod_Event::mod_Message::OneOfvalue::accountConfigUpdate(r.read_message::<mod_Event::mod_Account::mod_Config::Update>(bytes)?),
                Ok(1626) => msg.value = mod_Event::mod_Message::OneOfvalue::accountUpdate(r.read_message::<mod_Event::mod_Account::Update>(bytes)?),
                Ok(1634) => msg.value = mod_Event::mod_Message::OneOfvalue::accountLinkChallenge(r.read_message::<mod_Event::mod_Account::LinkChallenge>(bytes)?),
                Ok(1642) => msg.value = mod_Event::mod_Message::OneOfvalue::accountLinkChallengeHide(r.read_message::<mod_Event::mod_Account::LinkChallengeHide>(bytes)?),
                Ok(130) => msg.value = mod_Event::mod_Message::OneOfvalue::objectDetailsSet(r.read_message::<mod_Event::mod_Object::mod_Details::Set>(bytes)?),
                Ok(402) => msg.value = mod_Event::mod_Message::OneOfvalue::objectDetailsAmend(r.read_message::<mod_Event::mod_Object::mod_Details::Amend>(bytes)?),
                Ok(410) => msg.value = mod_Event::mod_Message::OneOfvalue::objectDetailsUnset(r.read_message::<mod_Event::mod_Object::mod_Details::Unset>(bytes)?),
                Ok(418) => msg.value = mod_Event::mod_Message::OneOfvalue::objectRelationsAmend(r.read_message::<mod_Event::mod_Object::mod_Relations::Amend>(bytes)?),
                Ok(426) => msg.value = mod_Event::mod_Message::OneOfvalue::objectRelationsRemove(r.read_message::<mod_Event::mod_Object::mod_Relations::Remove>(bytes)?),
                Ok(434) => msg.value = mod_Event::mod_Message::OneOfvalue::objectRemove(r.read_message::<mod_Event::mod_Object::Remove>(bytes)?),
                Ok(522) => msg.value = mod_Event::mod_Message::OneOfvalue::objectClose(r.read_message::<mod_Event::mod_Object::Close>(bytes)?),
                Ok(442) => msg.value = mod_Event::mod_Message::OneOfvalue::objectRestrictionsSet(r.read_message::<mod_Event::mod_Object::mod_Restrictions::Set>(bytes)?),
                Ok(482) => msg.value = mod_Event::mod_Message::OneOfvalue::subscriptionAdd(r.read_message::<mod_Event::mod_Object::mod_Subscription::Add>(bytes)?),
                Ok(490) => msg.value = mod_Event::mod_Message::OneOfvalue::subscriptionRemove(r.read_message::<mod_Event::mod_Object::mod_Subscription::Remove>(bytes)?),
                Ok(498) => msg.value = mod_Event::mod_Message::OneOfvalue::subscriptionPosition(r.read_message::<mod_Event::mod_Object::mod_Subscription::Position>(bytes)?),
                Ok(506) => msg.value = mod_Event::mod_Message::OneOfvalue::subscriptionCounters(r.read_message::<mod_Event::mod_Object::mod_Subscription::Counters>(bytes)?),
                Ok(514) => msg.value = mod_Event::mod_Message::OneOfvalue::subscriptionGroups(r.read_message::<mod_Event::mod_Object::mod_Subscription::Groups>(bytes)?),
                Ok(18) => msg.value = mod_Event::mod_Message::OneOfvalue::blockAdd(r.read_message::<mod_Event::mod_Block::Add>(bytes)?),
                Ok(26) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDelete(r.read_message::<mod_Event::mod_Block::Delete>(bytes)?),
                Ok(34) => msg.value = mod_Event::mod_Message::OneOfvalue::filesUpload(r.read_message::<mod_Event::mod_Block::FilesUpload>(bytes)?),
                Ok(42) => msg.value = mod_Event::mod_Message::OneOfvalue::marksInfo(r.read_message::<mod_Event::mod_Block::MarksInfo>(bytes)?),
                Ok(50) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetFields(r.read_message::<mod_Event::mod_Block::mod_Set::Fields>(bytes)?),
                Ok(58) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetChildrenIds(r.read_message::<mod_Event::mod_Block::mod_Set::ChildrenIds>(bytes)?),
                Ok(66) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetRestrictions(r.read_message::<mod_Event::mod_Block::mod_Set::Restrictions>(bytes)?),
                Ok(74) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetBackgroundColor(r.read_message::<mod_Event::mod_Block::mod_Set::BackgroundColor>(bytes)?),
                Ok(82) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetText(r.read_message::<mod_Event::mod_Block::mod_Set::Text>(bytes)?),
                Ok(90) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetFile(r.read_message::<mod_Event::mod_Block::mod_Set::File>(bytes)?),
                Ok(106) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetLink(r.read_message::<mod_Event::mod_Block::mod_Set::Link>(bytes)?),
                Ok(114) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetBookmark(r.read_message::<mod_Event::mod_Block::mod_Set::Bookmark>(bytes)?),
                Ok(122) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetAlign(r.read_message::<mod_Event::mod_Block::mod_Set::Align>(bytes)?),
                Ok(138) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetDiv(r.read_message::<mod_Event::mod_Block::mod_Set::Div>(bytes)?),
                Ok(170) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetRelation(r.read_message::<mod_Event::mod_Block::mod_Set::Relation>(bytes)?),
                Ok(202) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetLatex(r.read_message::<mod_Event::mod_Block::mod_Set::Latex>(bytes)?),
                Ok(290) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetVerticalAlign(r.read_message::<mod_Event::mod_Block::mod_Set::VerticalAlign>(bytes)?),
                Ok(298) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetTableRow(r.read_message::<mod_Event::mod_Block::mod_Set::TableRow>(bytes)?),
                Ok(322) => msg.value = mod_Event::mod_Message::OneOfvalue::blockSetWidget(r.read_message::<mod_Event::mod_Block::mod_Set::Widget>(bytes)?),
                Ok(154) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewViewSet(r.read_message::<mod_Event::mod_Block::mod_Dataview::ViewSet>(bytes)?),
                Ok(162) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewViewDelete(r.read_message::<mod_Event::mod_Block::mod_Dataview::ViewDelete>(bytes)?),
                Ok(234) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewViewOrder(r.read_message::<mod_Event::mod_Block::mod_Dataview::ViewOrder>(bytes)?),
                Ok(282) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewSourceSet(r.read_message::<mod_Event::mod_Block::mod_Dataview::SourceSet>(bytes)?),
                Ok(306) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataViewGroupOrderUpdate(r.read_message::<mod_Event::mod_Block::mod_Dataview::GroupOrderUpdate>(bytes)?),
                Ok(314) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataViewObjectOrderUpdate(r.read_message::<mod_Event::mod_Block::mod_Dataview::ObjectOrderUpdate>(bytes)?),
                Ok(994) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewRelationDelete(r.read_message::<mod_Event::mod_Block::mod_Dataview::RelationDelete>(bytes)?),
                Ok(986) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewRelationSet(r.read_message::<mod_Event::mod_Block::mod_Dataview::RelationSet>(bytes)?),
                Ok(1002) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewViewUpdate(r.read_message::<mod_Event::mod_Block::mod_Dataview::ViewUpdate>(bytes)?),
                Ok(1010) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewTargetObjectIdSet(r.read_message::<mod_Event::mod_Block::mod_Dataview::TargetObjectIdSet>(bytes)?),
                Ok(1018) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewIsCollectionSet(r.read_message::<mod_Event::mod_Block::mod_Dataview::IsCollectionSet>(bytes)?),
                Ok(194) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewOldRelationDelete(r.read_message::<mod_Event::mod_Block::mod_Dataview::OldRelationDelete>(bytes)?),
                Ok(186) => msg.value = mod_Event::mod_Message::OneOfvalue::blockDataviewOldRelationSet(r.read_message::<mod_Event::mod_Block::mod_Dataview::OldRelationSet>(bytes)?),
                Ok(250) => msg.value = mod_Event::mod_Message::OneOfvalue::userBlockJoin(r.read_message::<mod_Event::mod_User::mod_Block::Join>(bytes)?),
                Ok(258) => msg.value = mod_Event::mod_Message::OneOfvalue::userBlockLeft(r.read_message::<mod_Event::mod_User::mod_Block::Left>(bytes)?),
                Ok(266) => msg.value = mod_Event::mod_Message::OneOfvalue::userBlockSelectRange(r.read_message::<mod_Event::mod_User::mod_Block::SelectRange>(bytes)?),
                Ok(274) => msg.value = mod_Event::mod_Message::OneOfvalue::userBlockTextRange(r.read_message::<mod_Event::mod_User::mod_Block::TextRange>(bytes)?),
                Ok(802) => msg.value = mod_Event::mod_Message::OneOfvalue::ping(r.read_message::<mod_Event::Ping>(bytes)?),
                Ok(810) => msg.value = mod_Event::mod_Message::OneOfvalue::processNew(r.read_message::<mod_Event::mod_Process::New>(bytes)?),
                Ok(818) => msg.value = mod_Event::mod_Message::OneOfvalue::processUpdate(r.read_message::<mod_Event::mod_Process::Update>(bytes)?),
                Ok(826) => msg.value = mod_Event::mod_Message::OneOfvalue::processDone(r.read_message::<mod_Event::mod_Process::Done>(bytes)?),
                Ok(882) => msg.value = mod_Event::mod_Message::OneOfvalue::threadStatus(r.read_message::<mod_Event::mod_Status::Thread>(bytes)?),
                Ok(890) => msg.value = mod_Event::mod_Message::OneOfvalue::fileLimitReached(r.read_message::<mod_Event::mod_File::LimitReached>(bytes)?),
                Ok(898) => msg.value = mod_Event::mod_Message::OneOfvalue::fileSpaceUsage(r.read_message::<mod_Event::mod_File::SpaceUsage>(bytes)?),
                Ok(906) => msg.value = mod_Event::mod_Message::OneOfvalue::fileLocalUsage(r.read_message::<mod_Event::mod_File::LocalUsage>(bytes)?),
                Ok(946) => msg.value = mod_Event::mod_Message::OneOfvalue::fileLimitUpdated(r.read_message::<mod_Event::mod_File::LimitUpdated>(bytes)?),
                Ok(914) => msg.value = mod_Event::mod_Message::OneOfvalue::notificationSend(r.read_message::<mod_Event::mod_Notification::Send>(bytes)?),
                Ok(922) => msg.value = mod_Event::mod_Message::OneOfvalue::notificationUpdate(r.read_message::<mod_Event::mod_Notification::Update>(bytes)?),
                Ok(930) => msg.value = mod_Event::mod_Message::OneOfvalue::payloadBroadcast(r.read_message::<mod_Event::mod_Payload::Broadcast>(bytes)?),
                Ok(938) => msg.value = mod_Event::mod_Message::OneOfvalue::membershipUpdate(r.read_message::<mod_Event::mod_Membership::Update>(bytes)?),
                Ok(1098) => msg.value = mod_Event::mod_Message::OneOfvalue::membershipTiersUpdate(r.read_message::<mod_Event::mod_Membership::TiersUpdate>(bytes)?),
                Ok(954) => msg.value = mod_Event::mod_Message::OneOfvalue::spaceSyncStatusUpdate(r.read_message::<mod_Event::mod_Space::mod_SyncStatus::Update>(bytes)?),
                Ok(962) => msg.value = mod_Event::mod_Message::OneOfvalue::p2pStatusUpdate(r.read_message::<mod_Event::mod_P2PStatus::Update>(bytes)?),
                Ok(970) => msg.value = mod_Event::mod_Message::OneOfvalue::importFinish(r.read_message::<mod_Event::mod_Import::Finish>(bytes)?),
                Ok(1026) => msg.value = mod_Event::mod_Message::OneOfvalue::chatAdd(r.read_message::<mod_Event::mod_Chat::Add>(bytes)?),
                Ok(1034) => msg.value = mod_Event::mod_Message::OneOfvalue::chatUpdate(r.read_message::<mod_Event::mod_Chat::Update>(bytes)?),
                Ok(1042) => msg.value = mod_Event::mod_Message::OneOfvalue::chatUpdateReactions(r.read_message::<mod_Event::mod_Chat::UpdateReactions>(bytes)?),
                Ok(1074) => msg.value = mod_Event::mod_Message::OneOfvalue::chatUpdateMessageReadStatus(r.read_message::<mod_Event::mod_Chat::UpdateMessageReadStatus>(bytes)?),
                Ok(1082) => msg.value = mod_Event::mod_Message::OneOfvalue::chatUpdateMentionReadStatus(r.read_message::<mod_Event::mod_Chat::UpdateMentionReadStatus>(bytes)?),
                Ok(1090) => msg.value = mod_Event::mod_Message::OneOfvalue::chatUpdateMessageSyncStatus(r.read_message::<mod_Event::mod_Chat::UpdateMessageSyncStatus>(bytes)?),
                Ok(1050) => msg.value = mod_Event::mod_Message::OneOfvalue::chatDelete(r.read_message::<mod_Event::mod_Chat::Delete>(bytes)?),
                Ok(1066) => msg.value = mod_Event::mod_Message::OneOfvalue::chatStateUpdate(r.read_message::<mod_Event::mod_Chat::UpdateState>(bytes)?),
                Ok(1106) => msg.value = mod_Event::mod_Message::OneOfvalue::membershipV2Update(r.read_message::<mod_Event::mod_MembershipV2::Update>(bytes)?),
                Ok(1114) => msg.value = mod_Event::mod_Message::OneOfvalue::membershipV2ProductsUpdate(r.read_message::<mod_Event::mod_MembershipV2::ProductsUpdate>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Message<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 2 + sizeof_len((&self.spaceId).len()) }
        + match self.value {
            mod_Event::mod_Message::OneOfvalue::accountShow(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::accountDetails(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::accountConfigUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::accountUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::accountLinkChallenge(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::accountLinkChallengeHide(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectDetailsSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectDetailsAmend(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectDetailsUnset(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectRelationsAmend(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectRelationsRemove(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectRemove(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectClose(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::objectRestrictionsSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::subscriptionAdd(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::subscriptionRemove(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::subscriptionPosition(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::subscriptionCounters(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::subscriptionGroups(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockAdd(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDelete(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::filesUpload(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::marksInfo(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetFields(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetChildrenIds(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetRestrictions(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetBackgroundColor(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetText(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetFile(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetLink(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetBookmark(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetAlign(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetDiv(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetRelation(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetLatex(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetVerticalAlign(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetTableRow(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockSetWidget(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewDelete(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewOrder(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewSourceSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataViewGroupOrderUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataViewObjectOrderUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewRelationDelete(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewRelationSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewTargetObjectIdSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewIsCollectionSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewOldRelationDelete(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::blockDataviewOldRelationSet(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::userBlockJoin(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::userBlockLeft(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::userBlockSelectRange(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::userBlockTextRange(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::ping(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::processNew(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::processUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::processDone(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::threadStatus(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::fileLimitReached(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::fileSpaceUsage(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::fileLocalUsage(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::fileLimitUpdated(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::notificationSend(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::notificationUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::payloadBroadcast(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::membershipUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::membershipTiersUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::spaceSyncStatusUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::p2pStatusUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::importFinish(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatAdd(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatUpdateReactions(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatUpdateMessageReadStatus(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatUpdateMentionReadStatus(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatUpdateMessageSyncStatus(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatDelete(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::chatStateUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::membershipV2Update(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::membershipV2ProductsUpdate(ref m) => 2 + sizeof_len((m).get_size()),
            mod_Event::mod_Message::OneOfvalue::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(1058, |w| w.write_string(&**&self.spaceId))?; }
        match self.value {            mod_Event::mod_Message::OneOfvalue::accountShow(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::accountDetails(ref m) => { w.write_with_tag(1610, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::accountConfigUpdate(ref m) => { w.write_with_tag(1618, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::accountUpdate(ref m) => { w.write_with_tag(1626, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::accountLinkChallenge(ref m) => { w.write_with_tag(1634, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::accountLinkChallengeHide(ref m) => { w.write_with_tag(1642, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectDetailsSet(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectDetailsAmend(ref m) => { w.write_with_tag(402, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectDetailsUnset(ref m) => { w.write_with_tag(410, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectRelationsAmend(ref m) => { w.write_with_tag(418, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectRelationsRemove(ref m) => { w.write_with_tag(426, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectRemove(ref m) => { w.write_with_tag(434, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectClose(ref m) => { w.write_with_tag(522, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::objectRestrictionsSet(ref m) => { w.write_with_tag(442, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::subscriptionAdd(ref m) => { w.write_with_tag(482, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::subscriptionRemove(ref m) => { w.write_with_tag(490, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::subscriptionPosition(ref m) => { w.write_with_tag(498, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::subscriptionCounters(ref m) => { w.write_with_tag(506, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::subscriptionGroups(ref m) => { w.write_with_tag(514, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockAdd(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDelete(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::filesUpload(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::marksInfo(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetFields(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetChildrenIds(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetRestrictions(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetBackgroundColor(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetText(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetFile(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetLink(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetBookmark(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetAlign(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetDiv(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetRelation(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetLatex(ref m) => { w.write_with_tag(202, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetVerticalAlign(ref m) => { w.write_with_tag(290, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetTableRow(ref m) => { w.write_with_tag(298, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockSetWidget(ref m) => { w.write_with_tag(322, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewSet(ref m) => { w.write_with_tag(154, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewDelete(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewOrder(ref m) => { w.write_with_tag(234, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewSourceSet(ref m) => { w.write_with_tag(282, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataViewGroupOrderUpdate(ref m) => { w.write_with_tag(306, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataViewObjectOrderUpdate(ref m) => { w.write_with_tag(314, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewRelationDelete(ref m) => { w.write_with_tag(994, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewRelationSet(ref m) => { w.write_with_tag(986, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewViewUpdate(ref m) => { w.write_with_tag(1002, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewTargetObjectIdSet(ref m) => { w.write_with_tag(1010, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewIsCollectionSet(ref m) => { w.write_with_tag(1018, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewOldRelationDelete(ref m) => { w.write_with_tag(194, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::blockDataviewOldRelationSet(ref m) => { w.write_with_tag(186, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::userBlockJoin(ref m) => { w.write_with_tag(250, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::userBlockLeft(ref m) => { w.write_with_tag(258, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::userBlockSelectRange(ref m) => { w.write_with_tag(266, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::userBlockTextRange(ref m) => { w.write_with_tag(274, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::ping(ref m) => { w.write_with_tag(802, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::processNew(ref m) => { w.write_with_tag(810, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::processUpdate(ref m) => { w.write_with_tag(818, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::processDone(ref m) => { w.write_with_tag(826, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::threadStatus(ref m) => { w.write_with_tag(882, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::fileLimitReached(ref m) => { w.write_with_tag(890, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::fileSpaceUsage(ref m) => { w.write_with_tag(898, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::fileLocalUsage(ref m) => { w.write_with_tag(906, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::fileLimitUpdated(ref m) => { w.write_with_tag(946, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::notificationSend(ref m) => { w.write_with_tag(914, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::notificationUpdate(ref m) => { w.write_with_tag(922, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::payloadBroadcast(ref m) => { w.write_with_tag(930, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::membershipUpdate(ref m) => { w.write_with_tag(938, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::membershipTiersUpdate(ref m) => { w.write_with_tag(1098, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::spaceSyncStatusUpdate(ref m) => { w.write_with_tag(954, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::p2pStatusUpdate(ref m) => { w.write_with_tag(962, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::importFinish(ref m) => { w.write_with_tag(970, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatAdd(ref m) => { w.write_with_tag(1026, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatUpdate(ref m) => { w.write_with_tag(1034, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatUpdateReactions(ref m) => { w.write_with_tag(1042, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatUpdateMessageReadStatus(ref m) => { w.write_with_tag(1074, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatUpdateMentionReadStatus(ref m) => { w.write_with_tag(1082, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatUpdateMessageSyncStatus(ref m) => { w.write_with_tag(1090, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatDelete(ref m) => { w.write_with_tag(1050, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::chatStateUpdate(ref m) => { w.write_with_tag(1066, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::membershipV2Update(ref m) => { w.write_with_tag(1106, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::membershipV2ProductsUpdate(ref m) => { w.write_with_tag(1114, |w| w.write_message(m))? },
            mod_Event::mod_Message::OneOfvalue::None => {},
    }        Ok(())
    }
}

pub mod mod_Message {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfvalue<'a> {
    accountShow(mod_Event::mod_Account::Show<'a>),
    accountDetails(mod_Event::mod_Account::Details<'a>),
    accountConfigUpdate(mod_Event::mod_Account::mod_Config::Update<'a>),
    accountUpdate(mod_Event::mod_Account::Update<'a>),
    accountLinkChallenge(mod_Event::mod_Account::LinkChallenge<'a>),
    accountLinkChallengeHide(mod_Event::mod_Account::LinkChallengeHide<'a>),
    objectDetailsSet(mod_Event::mod_Object::mod_Details::Set<'a>),
    objectDetailsAmend(mod_Event::mod_Object::mod_Details::Amend<'a>),
    objectDetailsUnset(mod_Event::mod_Object::mod_Details::Unset<'a>),
    objectRelationsAmend(mod_Event::mod_Object::mod_Relations::Amend<'a>),
    objectRelationsRemove(mod_Event::mod_Object::mod_Relations::Remove<'a>),
    objectRemove(mod_Event::mod_Object::Remove<'a>),
    objectClose(mod_Event::mod_Object::Close<'a>),
    objectRestrictionsSet(mod_Event::mod_Object::mod_Restrictions::Set<'a>),
    subscriptionAdd(mod_Event::mod_Object::mod_Subscription::Add<'a>),
    subscriptionRemove(mod_Event::mod_Object::mod_Subscription::Remove<'a>),
    subscriptionPosition(mod_Event::mod_Object::mod_Subscription::Position<'a>),
    subscriptionCounters(mod_Event::mod_Object::mod_Subscription::Counters<'a>),
    subscriptionGroups(mod_Event::mod_Object::mod_Subscription::Groups<'a>),
    blockAdd(mod_Event::mod_Block::Add<'a>),
    blockDelete(mod_Event::mod_Block::Delete<'a>),
    filesUpload(mod_Event::mod_Block::FilesUpload<'a>),
    marksInfo(mod_Event::mod_Block::MarksInfo),
    blockSetFields(mod_Event::mod_Block::mod_Set::Fields<'a>),
    blockSetChildrenIds(mod_Event::mod_Block::mod_Set::ChildrenIds<'a>),
    blockSetRestrictions(mod_Event::mod_Block::mod_Set::Restrictions<'a>),
    blockSetBackgroundColor(mod_Event::mod_Block::mod_Set::BackgroundColor<'a>),
    blockSetText(mod_Event::mod_Block::mod_Set::Text<'a>),
    blockSetFile(mod_Event::mod_Block::mod_Set::File<'a>),
    blockSetLink(mod_Event::mod_Block::mod_Set::Link<'a>),
    blockSetBookmark(mod_Event::mod_Block::mod_Set::Bookmark<'a>),
    blockSetAlign(mod_Event::mod_Block::mod_Set::Align<'a>),
    blockSetDiv(mod_Event::mod_Block::mod_Set::Div<'a>),
    blockSetRelation(mod_Event::mod_Block::mod_Set::Relation<'a>),
    blockSetLatex(mod_Event::mod_Block::mod_Set::Latex<'a>),
    blockSetVerticalAlign(mod_Event::mod_Block::mod_Set::VerticalAlign<'a>),
    blockSetTableRow(mod_Event::mod_Block::mod_Set::TableRow<'a>),
    blockSetWidget(mod_Event::mod_Block::mod_Set::Widget<'a>),
    blockDataviewViewSet(mod_Event::mod_Block::mod_Dataview::ViewSet<'a>),
    blockDataviewViewDelete(mod_Event::mod_Block::mod_Dataview::ViewDelete<'a>),
    blockDataviewViewOrder(mod_Event::mod_Block::mod_Dataview::ViewOrder<'a>),
    blockDataviewSourceSet(mod_Event::mod_Block::mod_Dataview::SourceSet<'a>),
    blockDataViewGroupOrderUpdate(mod_Event::mod_Block::mod_Dataview::GroupOrderUpdate<'a>),
    blockDataViewObjectOrderUpdate(mod_Event::mod_Block::mod_Dataview::ObjectOrderUpdate<'a>),
    blockDataviewRelationDelete(mod_Event::mod_Block::mod_Dataview::RelationDelete<'a>),
    blockDataviewRelationSet(mod_Event::mod_Block::mod_Dataview::RelationSet<'a>),
    blockDataviewViewUpdate(mod_Event::mod_Block::mod_Dataview::ViewUpdate<'a>),
    blockDataviewTargetObjectIdSet(mod_Event::mod_Block::mod_Dataview::TargetObjectIdSet<'a>),
    blockDataviewIsCollectionSet(mod_Event::mod_Block::mod_Dataview::IsCollectionSet<'a>),
    blockDataviewOldRelationDelete(mod_Event::mod_Block::mod_Dataview::OldRelationDelete<'a>),
    blockDataviewOldRelationSet(mod_Event::mod_Block::mod_Dataview::OldRelationSet<'a>),
    userBlockJoin(mod_Event::mod_User::mod_Block::Join),
    userBlockLeft(mod_Event::mod_User::mod_Block::Left),
    userBlockSelectRange(mod_Event::mod_User::mod_Block::SelectRange<'a>),
    userBlockTextRange(mod_Event::mod_User::mod_Block::TextRange<'a>),
    ping(mod_Event::Ping),
    processNew(mod_Event::mod_Process::New<'a>),
    processUpdate(mod_Event::mod_Process::Update<'a>),
    processDone(mod_Event::mod_Process::Done<'a>),
    threadStatus(mod_Event::mod_Status::Thread<'a>),
    fileLimitReached(mod_Event::mod_File::LimitReached<'a>),
    fileSpaceUsage(mod_Event::mod_File::SpaceUsage<'a>),
    fileLocalUsage(mod_Event::mod_File::LocalUsage),
    fileLimitUpdated(mod_Event::mod_File::LimitUpdated),
    notificationSend(mod_Event::mod_Notification::Send<'a>),
    notificationUpdate(mod_Event::mod_Notification::Update<'a>),
    payloadBroadcast(mod_Event::mod_Payload::Broadcast<'a>),
    membershipUpdate(mod_Event::mod_Membership::Update<'a>),
    membershipTiersUpdate(mod_Event::mod_Membership::TiersUpdate<'a>),
    spaceSyncStatusUpdate(mod_Event::mod_Space::mod_SyncStatus::Update<'a>),
    p2pStatusUpdate(mod_Event::mod_P2PStatus::Update<'a>),
    importFinish(mod_Event::mod_Import::Finish<'a>),
    chatAdd(mod_Event::mod_Chat::Add<'a>),
    chatUpdate(mod_Event::mod_Chat::Update<'a>),
    chatUpdateReactions(mod_Event::mod_Chat::UpdateReactions<'a>),
    chatUpdateMessageReadStatus(mod_Event::mod_Chat::UpdateMessageReadStatus<'a>),
    chatUpdateMentionReadStatus(mod_Event::mod_Chat::UpdateMentionReadStatus<'a>),
    chatUpdateMessageSyncStatus(mod_Event::mod_Chat::UpdateMessageSyncStatus<'a>),
    chatDelete(mod_Event::mod_Chat::Delete<'a>),
    chatStateUpdate(mod_Event::mod_Chat::UpdateState<'a>),
    membershipV2Update(mod_Event::mod_MembershipV2::Update<'a>),
    membershipV2ProductsUpdate(mod_Event::mod_MembershipV2::ProductsUpdate<'a>),
    None,
}

impl<'a> Default for OneOfvalue<'a> {
    fn default() -> Self {
        OneOfvalue::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chat {
}

impl<'a> MessageRead<'a> for Chat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Chat {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Chat {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub id: Cow<'a, str>,
    pub orderId: Cow<'a, str>,
    pub afterOrderId: Cow<'a, str>,
    pub message: Option<anytype::model::ChatMessage<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
    pub dependencies: Vec<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.orderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.afterOrderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.message = Some(r.read_message::<anytype::model::ChatMessage>(bytes)?),
                Ok(34) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.dependencies.push(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.orderId == "" { 0 } else { 1 + sizeof_len((&self.orderId).len()) }
        + if self.afterOrderId == "" { 0 } else { 1 + sizeof_len((&self.afterOrderId).len()) }
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.dependencies.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.orderId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.orderId))?; }
        if self.afterOrderId != "" { w.write_with_tag(50, |w| w.write_string(&**&self.afterOrderId))?; }
        if let Some(ref s) = self.message { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.subIds { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        for s in &self.dependencies { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Delete<'a> {
    pub id: Cow<'a, str>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Delete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Delete<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.subIds { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub id: Cow<'a, str>,
    pub message: Option<anytype::model::ChatMessage<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.message = Some(r.read_message::<anytype::model::ChatMessage>(bytes)?),
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.message { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateReactions<'a> {
    pub id: Cow<'a, str>,
    pub reactions: Option<anytype::model::mod_ChatMessage::Reactions<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for UpdateReactions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.reactions = Some(r.read_message::<anytype::model::mod_ChatMessage::Reactions>(bytes)?),
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateReactions<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.reactions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.reactions { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateMessageReadStatus<'a> {
    pub ids: Vec<Cow<'a, str>>,
    pub isRead: bool,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for UpdateMessageReadStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.isRead = r.read_bool(bytes)?,
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateMessageReadStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.isRead == false { 0 } else { 1 + sizeof_varint(*(&self.isRead) as u64) }
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.isRead != false { w.write_with_tag(16, |w| w.write_bool(*&self.isRead))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateMentionReadStatus<'a> {
    pub ids: Vec<Cow<'a, str>>,
    pub isRead: bool,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for UpdateMentionReadStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.isRead = r.read_bool(bytes)?,
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateMentionReadStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.isRead == false { 0 } else { 1 + sizeof_varint(*(&self.isRead) as u64) }
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.isRead != false { w.write_with_tag(16, |w| w.write_bool(*&self.isRead))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateMessageSyncStatus<'a> {
    pub ids: Vec<Cow<'a, str>>,
    pub isSynced: bool,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for UpdateMessageSyncStatus<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.isSynced = r.read_bool(bytes)?,
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateMessageSyncStatus<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.isSynced == false { 0 } else { 1 + sizeof_varint(*(&self.isSynced) as u64) }
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if self.isSynced != false { w.write_with_tag(16, |w| w.write_bool(*&self.isSynced))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateState<'a> {
    pub state: Option<anytype::model::ChatState<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for UpdateState<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.state = Some(r.read_message::<anytype::model::ChatState>(bytes)?),
                Ok(18) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateState<'a> {
    fn get_size(&self) -> usize {
        0
        + self.state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.state { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.subIds { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Account {
}

impl<'a> MessageRead<'a> for Account {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Account {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Account {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Show<'a> {
    pub index: i32,
    pub account: Option<anytype::model::Account<'a>>,
}

impl<'a> MessageRead<'a> for Show<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = r.read_int32(bytes)?,
                Ok(18) => msg.account = Some(r.read_message::<anytype::model::Account>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Show<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.index == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.index != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.index))?; }
        if let Some(ref s) = self.account { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Details<'a> {
    pub profileId: Cow<'a, str>,
    pub details: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Details<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.profileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.details = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Details<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.profileId == "" { 0 } else { 1 + sizeof_len((&self.profileId).len()) }
        + self.details.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.profileId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.profileId))?; }
        if let Some(ref s) = self.details { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Config {
}

impl<'a> MessageRead<'a> for Config {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Config {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Config {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub config: Option<anytype::model::mod_Account::Config<'a>>,
    pub status: Option<anytype::model::mod_Account::Status>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.config = Some(r.read_message::<anytype::model::mod_Account::Config>(bytes)?),
                Ok(18) => msg.status = Some(r.read_message::<anytype::model::mod_Account::Status>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + self.config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.config { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub config: Option<anytype::model::mod_Account::Config<'a>>,
    pub status: Option<anytype::model::mod_Account::Status>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.config = Some(r.read_message::<anytype::model::mod_Account::Config>(bytes)?),
                Ok(18) => msg.status = Some(r.read_message::<anytype::model::mod_Account::Status>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + self.config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.config { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LinkChallenge<'a> {
    pub challenge: Cow<'a, str>,
    pub clientInfo: Option<mod_Event::mod_Account::mod_LinkChallenge::ClientInfo<'a>>,
    pub scope: anytype::model::mod_Account::mod_Auth::LocalApiScope,
}

impl<'a> MessageRead<'a> for LinkChallenge<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.challenge = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.clientInfo = Some(r.read_message::<mod_Event::mod_Account::mod_LinkChallenge::ClientInfo>(bytes)?),
                Ok(24) => msg.scope = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LinkChallenge<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.challenge == "" { 0 } else { 1 + sizeof_len((&self.challenge).len()) }
        + self.clientInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.scope == anytype::model::mod_Account::mod_Auth::LocalApiScope::Limited { 0 } else { 1 + sizeof_varint(*(&self.scope) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.challenge != "" { w.write_with_tag(10, |w| w.write_string(&**&self.challenge))?; }
        if let Some(ref s) = self.clientInfo { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.scope != anytype::model::mod_Account::mod_Auth::LocalApiScope::Limited { w.write_with_tag(24, |w| w.write_enum(*&self.scope as i32))?; }
        Ok(())
    }
}

pub mod mod_LinkChallenge {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ClientInfo<'a> {
    pub processName: Cow<'a, str>,
    pub processPath: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub signatureVerified: bool,
}

impl<'a> MessageRead<'a> for ClientInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.processName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.processPath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.signatureVerified = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ClientInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.processName == "" { 0 } else { 1 + sizeof_len((&self.processName).len()) }
        + if self.processPath == "" { 0 } else { 1 + sizeof_len((&self.processPath).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.signatureVerified == false { 0 } else { 1 + sizeof_varint(*(&self.signatureVerified) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.processName != "" { w.write_with_tag(10, |w| w.write_string(&**&self.processName))?; }
        if self.processPath != "" { w.write_with_tag(18, |w| w.write_string(&**&self.processPath))?; }
        if self.name != "" { w.write_with_tag(34, |w| w.write_string(&**&self.name))?; }
        if self.signatureVerified != false { w.write_with_tag(24, |w| w.write_bool(*&self.signatureVerified))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LinkChallengeHide<'a> {
    pub challenge: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LinkChallengeHide<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.challenge = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LinkChallengeHide<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.challenge == "" { 0 } else { 1 + sizeof_len((&self.challenge).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.challenge != "" { w.write_with_tag(10, |w| w.write_string(&**&self.challenge))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Object {
}

impl<'a> MessageRead<'a> for Object {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Object {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Object {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Details {
}

impl<'a> MessageRead<'a> for Details {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Details {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Details {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Amend<'a> {
    pub id: Cow<'a, str>,
    pub details: Vec<mod_Event::mod_Object::mod_Details::mod_Amend::KeyValue<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Amend<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.details.push(r.read_message::<mod_Event::mod_Object::mod_Details::mod_Amend::KeyValue>(bytes)?),
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Amend<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.details.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.details { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_Amend {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct KeyValue<'a> {
    pub key: Cow<'a, str>,
    pub value: Option<google::protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for KeyValue<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.value = Some(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for KeyValue<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set<'a> {
    pub id: Cow<'a, str>,
    pub details: Option<google::protobuf::Struct<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Set<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.details = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Set<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.details.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.details { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Unset<'a> {
    pub id: Cow<'a, str>,
    pub keys: Vec<Cow<'a, str>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Unset<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.keys.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.subIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Unset<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.keys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.subIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.keys { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        for s in &self.subIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Subscription {
}

impl<'a> MessageRead<'a> for Subscription {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Subscription {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Subscription {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub id: Cow<'a, str>,
    pub afterId: Cow<'a, str>,
    pub subId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.subId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + if self.subId == "" { 0 } else { 1 + sizeof_len((&self.subId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.afterId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.afterId))?; }
        if self.subId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.subId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub id: Cow<'a, str>,
    pub subId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.subId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.subId == "" { 0 } else { 1 + sizeof_len((&self.subId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.subId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.subId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Position<'a> {
    pub id: Cow<'a, str>,
    pub afterId: Cow<'a, str>,
    pub subId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Position<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.subId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Position<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + if self.subId == "" { 0 } else { 1 + sizeof_len((&self.subId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.afterId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.afterId))?; }
        if self.subId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.subId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Counters<'a> {
    pub total: i64,
    pub nextCount: i64,
    pub prevCount: i64,
    pub subId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Counters<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.total = r.read_int64(bytes)?,
                Ok(16) => msg.nextCount = r.read_int64(bytes)?,
                Ok(24) => msg.prevCount = r.read_int64(bytes)?,
                Ok(34) => msg.subId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Counters<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.total == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.total) as u64) }
        + if self.nextCount == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.nextCount) as u64) }
        + if self.prevCount == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.prevCount) as u64) }
        + if self.subId == "" { 0 } else { 1 + sizeof_len((&self.subId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.total != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.total))?; }
        if self.nextCount != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.nextCount))?; }
        if self.prevCount != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.prevCount))?; }
        if self.subId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.subId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Groups<'a> {
    pub subId: Cow<'a, str>,
    pub group: Option<anytype::model::mod_Block::mod_Content::mod_Dataview::Group<'a>>,
    pub remove: bool,
}

impl<'a> MessageRead<'a> for Groups<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.group = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Group>(bytes)?),
                Ok(24) => msg.remove = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Groups<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.subId == "" { 0 } else { 1 + sizeof_len((&self.subId).len()) }
        + self.group.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.remove == false { 0 } else { 1 + sizeof_varint(*(&self.remove) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.subId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.subId))?; }
        if let Some(ref s) = self.group { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.remove != false { w.write_with_tag(24, |w| w.write_bool(*&self.remove))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relations {
}

impl<'a> MessageRead<'a> for Relations {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Relations {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Relations {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Amend<'a> {
    pub id: Cow<'a, str>,
    pub relationLinks: Vec<anytype::model::RelationLink<'a>>,
}

impl<'a> MessageRead<'a> for Amend<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relationLinks.push(r.read_message::<anytype::model::RelationLink>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Amend<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.relationLinks { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub id: Cow<'a, str>,
    pub relationKeys: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relationKeys.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.relationKeys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.relationKeys { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Restrictions {
}

impl<'a> MessageRead<'a> for Restrictions {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Restrictions {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Restrictions {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set<'a> {
    pub id: Cow<'a, str>,
    pub restrictions: Option<anytype::model::Restrictions<'a>>,
}

impl<'a> MessageRead<'a> for Set<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.restrictions = Some(r.read_message::<anytype::model::Restrictions>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Set<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.restrictions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.restrictions { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Close<'a> {
    pub id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Close<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Close<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Block {
}

impl<'a> MessageRead<'a> for Block {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Block {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Block {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub blocks: Vec<anytype::model::Block<'a>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.blocks.push(r.read_message::<anytype::model::Block>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + self.blocks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.blocks { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FilesUpload<'a> {
    pub blockId: Cow<'a, str>,
    pub filePath: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for FilesUpload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.blockId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.filePath.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FilesUpload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.blockId == "" { 0 } else { 1 + sizeof_len((&self.blockId).len()) }
        + self.filePath.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.blockId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.blockId))?; }
        for s in &self.filePath { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Delete<'a> {
    pub blockIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Delete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.blockIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Delete<'a> {
    fn get_size(&self) -> usize {
        0
        + self.blockIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.blockIds { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MarksInfo {
    pub marksInRange: Vec<anytype::model::mod_Block::mod_Content::mod_Text::mod_Mark::Type>,
}

impl<'a> MessageRead<'a> for MarksInfo {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.marksInRange.push(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MarksInfo {
    fn get_size(&self) -> usize {
        0
        + self.marksInRange.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.marksInRange { w.write_with_tag(8, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Set {
}

impl<'a> MessageRead<'a> for Set {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Set {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Set {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation<'a> {
    pub id: Cow<'a, str>,
    pub key: Option<mod_Event::mod_Block::mod_Set::mod_Relation::Key<'a>>,
}

impl<'a> MessageRead<'a> for Relation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.key = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Relation::Key>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Relation<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.key.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.key { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Relation {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Key<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Key<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Key<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fields<'a> {
    pub id: Cow<'a, str>,
    pub fields: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Fields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.fields = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Fields<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChildrenIds<'a> {
    pub id: Cow<'a, str>,
    pub childrenIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ChildrenIds<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.childrenIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChildrenIds<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.childrenIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.childrenIds { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Restrictions<'a> {
    pub id: Cow<'a, str>,
    pub restrictions: Option<anytype::model::mod_Block::Restrictions>,
}

impl<'a> MessageRead<'a> for Restrictions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.restrictions = Some(r.read_message::<anytype::model::mod_Block::Restrictions>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Restrictions<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.restrictions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.restrictions { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackgroundColor<'a> {
    pub id: Cow<'a, str>,
    pub backgroundColor: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for BackgroundColor<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.backgroundColor = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BackgroundColor<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.backgroundColor == "" { 0 } else { 1 + sizeof_len((&self.backgroundColor).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.backgroundColor != "" { w.write_with_tag(18, |w| w.write_string(&**&self.backgroundColor))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Align<'a> {
    pub id: Cow<'a, str>,
    pub align: anytype::model::mod_Block::Align,
}

impl<'a> MessageRead<'a> for Align<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.align = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Align<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.align == anytype::model::mod_Block::Align::AlignLeft { 0 } else { 1 + sizeof_varint(*(&self.align) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.align != anytype::model::mod_Block::Align::AlignLeft { w.write_with_tag(16, |w| w.write_enum(*&self.align as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct VerticalAlign<'a> {
    pub id: Cow<'a, str>,
    pub verticalAlign: anytype::model::mod_Block::VerticalAlign,
}

impl<'a> MessageRead<'a> for VerticalAlign<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.verticalAlign = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for VerticalAlign<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.verticalAlign == anytype::model::mod_Block::VerticalAlign::VerticalAlignTop { 0 } else { 1 + sizeof_varint(*(&self.verticalAlign) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.verticalAlign != anytype::model::mod_Block::VerticalAlign::VerticalAlignTop { w.write_with_tag(16, |w| w.write_enum(*&self.verticalAlign as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub id: Cow<'a, str>,
    pub text: Option<mod_Event::mod_Block::mod_Set::mod_Text::Text<'a>>,
    pub style: Option<mod_Event::mod_Block::mod_Set::mod_Text::Style>,
    pub marks: Option<mod_Event::mod_Block::mod_Set::mod_Text::Marks<'a>>,
    pub checked: Option<mod_Event::mod_Block::mod_Set::mod_Text::Checked>,
    pub color: Option<mod_Event::mod_Block::mod_Set::mod_Text::Color<'a>>,
    pub iconEmoji: Option<mod_Event::mod_Block::mod_Set::mod_Text::IconEmoji<'a>>,
    pub iconImage: Option<mod_Event::mod_Block::mod_Set::mod_Text::IconImage<'a>>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.text = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::Text>(bytes)?),
                Ok(26) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::Style>(bytes)?),
                Ok(34) => msg.marks = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::Marks>(bytes)?),
                Ok(42) => msg.checked = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::Checked>(bytes)?),
                Ok(50) => msg.color = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::Color>(bytes)?),
                Ok(58) => msg.iconEmoji = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::IconEmoji>(bytes)?),
                Ok(66) => msg.iconImage = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Text::IconImage>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.marks.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.checked.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.iconEmoji.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.iconImage.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.style { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.marks { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.checked { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.color { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.iconEmoji { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.iconImage { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Text {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_Text::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Marks<'a> {
    pub value: Option<anytype::model::mod_Block::mod_Content::mod_Text::Marks<'a>>,
}

impl<'a> MessageRead<'a> for Marks<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Text::Marks>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Marks<'a> {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Checked {
    pub value: bool,
}

impl<'a> MessageRead<'a> for Checked {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Checked {
    fn get_size(&self) -> usize {
        0
        + if self.value == false { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != false { w.write_with_tag(8, |w| w.write_bool(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Color<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Color<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Color<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IconEmoji<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for IconEmoji<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IconEmoji<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IconImage<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for IconImage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IconImage<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Latex<'a> {
    pub id: Cow<'a, str>,
    pub text: Option<mod_Event::mod_Block::mod_Set::mod_Latex::Text<'a>>,
    pub processor: Option<mod_Event::mod_Block::mod_Set::mod_Latex::Processor>,
}

impl<'a> MessageRead<'a> for Latex<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.text = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Latex::Text>(bytes)?),
                Ok(26) => msg.processor = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Latex::Processor>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Latex<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.processor.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.processor { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Latex {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Processor {
    pub value: anytype::model::mod_Block::mod_Content::mod_Latex::Processor,
}

impl<'a> MessageRead<'a> for Processor {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Processor {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Latex::Processor::Latex { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Latex::Processor::Latex { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Div<'a> {
    pub id: Cow<'a, str>,
    pub style: Option<mod_Event::mod_Block::mod_Set::mod_Div::Style>,
}

impl<'a> MessageRead<'a> for Div<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Div::Style>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Div<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.style { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Div {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_Div::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Div::Style::Line { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Div::Style::Line { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct File<'a> {
    pub id: Cow<'a, str>,
    pub type_pb: Option<mod_Event::mod_Block::mod_Set::mod_File::Type>,
    pub state: Option<mod_Event::mod_Block::mod_Set::mod_File::State>,
    pub mime: Option<mod_Event::mod_Block::mod_Set::mod_File::Mime<'a>>,
    pub hash: Option<mod_Event::mod_Block::mod_Set::mod_File::Hash<'a>>,
    pub name: Option<mod_Event::mod_Block::mod_Set::mod_File::Name<'a>>,
    pub size: Option<mod_Event::mod_Block::mod_Set::mod_File::Size>,
    pub style: Option<mod_Event::mod_Block::mod_Set::mod_File::Style>,
    pub targetObjectId: Option<mod_Event::mod_Block::mod_Set::mod_File::TargetObjectId<'a>>,
}

impl<'a> MessageRead<'a> for File<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.type_pb = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::Type>(bytes)?),
                Ok(26) => msg.state = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::State>(bytes)?),
                Ok(34) => msg.mime = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::Mime>(bytes)?),
                Ok(42) => msg.hash = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::Hash>(bytes)?),
                Ok(50) => msg.name = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::Name>(bytes)?),
                Ok(58) => msg.size = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::Size>(bytes)?),
                Ok(66) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::Style>(bytes)?),
                Ok(74) => msg.targetObjectId = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_File::TargetObjectId>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for File<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mime.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.hash.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.targetObjectId.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.state { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mime { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.hash { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.style { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.targetObjectId { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_File {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Name<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Name<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Name<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Width {
    pub value: i32,
}

impl<'a> MessageRead<'a> for Width {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Width {
    fn get_size(&self) -> usize {
        0
        + if self.value == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct State {
    pub value: anytype::model::mod_Block::mod_Content::mod_File::State,
}

impl<'a> MessageRead<'a> for State {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for State {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_File::State::Empty { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_File::State::Empty { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Type {
    pub value: anytype::model::mod_Block::mod_Content::mod_File::Type,
}

impl<'a> MessageRead<'a> for Type {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Type {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_File::Type::None { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_File::Type::None { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_File::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_File::Style::Auto { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_File::Style::Auto { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Hash<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Hash<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Hash<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Mime<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Mime<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Mime<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Size {
    pub value: i64,
}

impl<'a> MessageRead<'a> for Size {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Size {
    fn get_size(&self) -> usize {
        0
        + if self.value == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetObjectId<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TargetObjectId<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TargetObjectId<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Link<'a> {
    pub id: Cow<'a, str>,
    pub targetBlockId: Option<mod_Event::mod_Block::mod_Set::mod_Link::TargetBlockId<'a>>,
    pub style: Option<mod_Event::mod_Block::mod_Set::mod_Link::Style>,
    pub fields: Option<mod_Event::mod_Block::mod_Set::mod_Link::Fields<'a>>,
    pub iconSize: Option<mod_Event::mod_Block::mod_Set::mod_Link::IconSize>,
    pub cardStyle: Option<mod_Event::mod_Block::mod_Set::mod_Link::CardStyle>,
    pub description: Option<mod_Event::mod_Block::mod_Set::mod_Link::Description>,
    pub relations: Option<mod_Event::mod_Block::mod_Set::mod_Link::Relations<'a>>,
}

impl<'a> MessageRead<'a> for Link<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.targetBlockId = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::TargetBlockId>(bytes)?),
                Ok(26) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::Style>(bytes)?),
                Ok(34) => msg.fields = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::Fields>(bytes)?),
                Ok(42) => msg.iconSize = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::IconSize>(bytes)?),
                Ok(50) => msg.cardStyle = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::CardStyle>(bytes)?),
                Ok(58) => msg.description = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::Description>(bytes)?),
                Ok(66) => msg.relations = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Link::Relations>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Link<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.targetBlockId.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.iconSize.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.cardStyle.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.relations.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.targetBlockId { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.style { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.iconSize { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cardStyle { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.description { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.relations { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Link {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetBlockId<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TargetBlockId<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TargetBlockId<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_Link::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Link::Style::Page { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Link::Style::Page { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fields<'a> {
    pub value: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Fields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Fields<'a> {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IconSize {
    pub value: anytype::model::mod_Block::mod_Content::mod_Link::IconSize,
}

impl<'a> MessageRead<'a> for IconSize {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for IconSize {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Link::IconSize::SizeNone { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Link::IconSize::SizeNone { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CardStyle {
    pub value: anytype::model::mod_Block::mod_Content::mod_Link::CardStyle,
}

impl<'a> MessageRead<'a> for CardStyle {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CardStyle {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Link::CardStyle::Text { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Link::CardStyle::Text { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Description {
    pub value: anytype::model::mod_Block::mod_Content::mod_Link::Description,
}

impl<'a> MessageRead<'a> for Description {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Description {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Link::Description::None { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Link::Description::None { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relations<'a> {
    pub value: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Relations<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Relations<'a> {
    fn get_size(&self) -> usize {
        0
        + self.value.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.value { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Bookmark<'a> {
    pub id: Cow<'a, str>,
    pub url: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::Url<'a>>,
    pub title: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::Title<'a>>,
    pub description: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::Description<'a>>,
    pub imageHash: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::ImageHash<'a>>,
    pub faviconHash: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::FaviconHash<'a>>,
    pub type_pb: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::Type>,
    pub targetObjectId: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::TargetObjectId<'a>>,
    pub state: Option<mod_Event::mod_Block::mod_Set::mod_Bookmark::State>,
}

impl<'a> MessageRead<'a> for Bookmark<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.url = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::Url>(bytes)?),
                Ok(26) => msg.title = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::Title>(bytes)?),
                Ok(34) => msg.description = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::Description>(bytes)?),
                Ok(42) => msg.imageHash = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::ImageHash>(bytes)?),
                Ok(50) => msg.faviconHash = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::FaviconHash>(bytes)?),
                Ok(58) => msg.type_pb = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::Type>(bytes)?),
                Ok(66) => msg.targetObjectId = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::TargetObjectId>(bytes)?),
                Ok(74) => msg.state = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Bookmark::State>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Bookmark<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.title.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.imageHash.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.faviconHash.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.targetObjectId.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.title { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.description { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.imageHash { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.faviconHash { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.targetObjectId { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.state { w.write_with_tag(74, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Bookmark {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Url<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Url<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Url<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Title<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Title<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Title<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Description<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Description<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Description<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImageHash<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ImageHash<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ImageHash<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FaviconHash<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FaviconHash<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FaviconHash<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Type {
    pub value: anytype::model::mod_LinkPreview::Type,
}

impl<'a> MessageRead<'a> for Type {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Type {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_LinkPreview::Type::Unknown { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_LinkPreview::Type::Unknown { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetObjectId<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TargetObjectId<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TargetObjectId<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct State {
    pub value: anytype::model::mod_Block::mod_Content::mod_Bookmark::State,
}

impl<'a> MessageRead<'a> for State {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for State {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Bookmark::State::Empty { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Bookmark::State::Empty { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableRow<'a> {
    pub id: Cow<'a, str>,
    pub isHeader: Option<mod_Event::mod_Block::mod_Set::mod_TableRow::IsHeader>,
}

impl<'a> MessageRead<'a> for TableRow<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.isHeader = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_TableRow::IsHeader>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TableRow<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.isHeader.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.isHeader { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_TableRow {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IsHeader {
    pub value: bool,
}

impl<'a> MessageRead<'a> for IsHeader {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for IsHeader {
    fn get_size(&self) -> usize {
        0
        + if self.value == false { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != false { w.write_with_tag(8, |w| w.write_bool(*&self.value))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Widget<'a> {
    pub id: Cow<'a, str>,
    pub layout: Option<mod_Event::mod_Block::mod_Set::mod_Widget::Layout>,
    pub limit: Option<mod_Event::mod_Block::mod_Set::mod_Widget::Limit>,
    pub viewId: Option<mod_Event::mod_Block::mod_Set::mod_Widget::ViewId<'a>>,
}

impl<'a> MessageRead<'a> for Widget<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.layout = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Widget::Layout>(bytes)?),
                Ok(26) => msg.limit = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Widget::Limit>(bytes)?),
                Ok(34) => msg.viewId = Some(r.read_message::<mod_Event::mod_Block::mod_Set::mod_Widget::ViewId>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Widget<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.layout.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.limit.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.viewId.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.layout { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.limit { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.viewId { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Widget {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Layout {
    pub value: anytype::model::mod_Block::mod_Content::mod_Widget::Layout,
}

impl<'a> MessageRead<'a> for Layout {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Layout {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Widget::Layout::Link { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Widget::Layout::Link { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Limit {
    pub value: i32,
}

impl<'a> MessageRead<'a> for Limit {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Limit {
    fn get_size(&self) -> usize {
        0
        + if self.value == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViewId<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ViewId<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ViewId<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fill {
}

impl<'a> MessageRead<'a> for Fill {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Fill {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Fill {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Details<'a> {
    pub id: Cow<'a, str>,
    pub details: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Details<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.details = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Details<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.details.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.details { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DatabaseRecords<'a> {
    pub id: Cow<'a, str>,
    pub records: Vec<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for DatabaseRecords<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.records.push(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DatabaseRecords<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.records.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.records { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fields<'a> {
    pub id: Cow<'a, str>,
    pub fields: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Fields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.fields = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Fields<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChildrenIds<'a> {
    pub id: Cow<'a, str>,
    pub childrenIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ChildrenIds<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.childrenIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChildrenIds<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.childrenIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.childrenIds { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Restrictions<'a> {
    pub id: Cow<'a, str>,
    pub restrictions: Option<anytype::model::mod_Block::Restrictions>,
}

impl<'a> MessageRead<'a> for Restrictions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.restrictions = Some(r.read_message::<anytype::model::mod_Block::Restrictions>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Restrictions<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.restrictions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.restrictions { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BackgroundColor<'a> {
    pub id: Cow<'a, str>,
    pub backgroundColor: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for BackgroundColor<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.backgroundColor = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BackgroundColor<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.backgroundColor == "" { 0 } else { 1 + sizeof_len((&self.backgroundColor).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.backgroundColor != "" { w.write_with_tag(18, |w| w.write_string(&**&self.backgroundColor))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Align<'a> {
    pub id: Cow<'a, str>,
    pub align: anytype::model::mod_Block::Align,
}

impl<'a> MessageRead<'a> for Align<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.align = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Align<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.align == anytype::model::mod_Block::Align::AlignLeft { 0 } else { 1 + sizeof_varint(*(&self.align) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.align != anytype::model::mod_Block::Align::AlignLeft { w.write_with_tag(16, |w| w.write_enum(*&self.align as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub id: Cow<'a, str>,
    pub text: Option<mod_Event::mod_Block::mod_Fill::mod_Text::Text<'a>>,
    pub style: Option<mod_Event::mod_Block::mod_Fill::mod_Text::Style>,
    pub marks: Option<mod_Event::mod_Block::mod_Fill::mod_Text::Marks<'a>>,
    pub checked: Option<mod_Event::mod_Block::mod_Fill::mod_Text::Checked>,
    pub color: Option<mod_Event::mod_Block::mod_Fill::mod_Text::Color<'a>>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.text = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Text::Text>(bytes)?),
                Ok(26) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Text::Style>(bytes)?),
                Ok(34) => msg.marks = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Text::Marks>(bytes)?),
                Ok(42) => msg.checked = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Text::Checked>(bytes)?),
                Ok(50) => msg.color = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Text::Color>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.text.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.marks.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.checked.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.color.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.text { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.style { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.marks { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.checked { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.color { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Text {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Text<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_Text::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Marks<'a> {
    pub value: Option<anytype::model::mod_Block::mod_Content::mod_Text::Marks<'a>>,
}

impl<'a> MessageRead<'a> for Marks<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Text::Marks>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Marks<'a> {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Checked {
    pub value: bool,
}

impl<'a> MessageRead<'a> for Checked {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Checked {
    fn get_size(&self) -> usize {
        0
        + if self.value == false { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != false { w.write_with_tag(8, |w| w.write_bool(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Color<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Color<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Color<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Div<'a> {
    pub id: Cow<'a, str>,
    pub style: Option<mod_Event::mod_Block::mod_Fill::mod_Div::Style>,
}

impl<'a> MessageRead<'a> for Div<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Div::Style>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Div<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.style { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Div {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_Div::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Div::Style::Line { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Div::Style::Line { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct File<'a> {
    pub id: Cow<'a, str>,
    pub type_pb: Option<mod_Event::mod_Block::mod_Fill::mod_File::Type>,
    pub state: Option<mod_Event::mod_Block::mod_Fill::mod_File::State>,
    pub mime: Option<mod_Event::mod_Block::mod_Fill::mod_File::Mime<'a>>,
    pub hash: Option<mod_Event::mod_Block::mod_Fill::mod_File::Hash<'a>>,
    pub name: Option<mod_Event::mod_Block::mod_Fill::mod_File::Name<'a>>,
    pub size: Option<mod_Event::mod_Block::mod_Fill::mod_File::Size>,
    pub style: Option<mod_Event::mod_Block::mod_Fill::mod_File::Style>,
}

impl<'a> MessageRead<'a> for File<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.type_pb = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::Type>(bytes)?),
                Ok(26) => msg.state = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::State>(bytes)?),
                Ok(34) => msg.mime = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::Mime>(bytes)?),
                Ok(42) => msg.hash = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::Hash>(bytes)?),
                Ok(50) => msg.name = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::Name>(bytes)?),
                Ok(58) => msg.size = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::Size>(bytes)?),
                Ok(66) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_File::Style>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for File<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.state.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mime.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.hash.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.name.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.size.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.state { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mime { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.hash { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.name { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.size { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.style { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_File {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Name<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Name<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Name<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Width {
    pub value: i32,
}

impl<'a> MessageRead<'a> for Width {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Width {
    fn get_size(&self) -> usize {
        0
        + if self.value == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct State {
    pub value: anytype::model::mod_Block::mod_Content::mod_File::State,
}

impl<'a> MessageRead<'a> for State {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for State {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_File::State::Empty { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_File::State::Empty { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Type {
    pub value: anytype::model::mod_Block::mod_Content::mod_File::Type,
}

impl<'a> MessageRead<'a> for Type {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Type {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_File::Type::None { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_File::Type::None { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_File::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_File::Style::Auto { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_File::Style::Auto { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Hash<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Hash<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Hash<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Mime<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Mime<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Mime<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Size {
    pub value: i64,
}

impl<'a> MessageRead<'a> for Size {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Size {
    fn get_size(&self) -> usize {
        0
        + if self.value == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.value))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Link<'a> {
    pub id: Cow<'a, str>,
    pub targetBlockId: Option<mod_Event::mod_Block::mod_Fill::mod_Link::TargetBlockId<'a>>,
    pub style: Option<mod_Event::mod_Block::mod_Fill::mod_Link::Style>,
    pub fields: Option<mod_Event::mod_Block::mod_Fill::mod_Link::Fields<'a>>,
}

impl<'a> MessageRead<'a> for Link<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.targetBlockId = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Link::TargetBlockId>(bytes)?),
                Ok(26) => msg.style = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Link::Style>(bytes)?),
                Ok(34) => msg.fields = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Link::Fields>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Link<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.targetBlockId.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.style.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.targetBlockId { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.style { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Link {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetBlockId<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TargetBlockId<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TargetBlockId<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Style {
    pub value: anytype::model::mod_Block::mod_Content::mod_Link::Style,
}

impl<'a> MessageRead<'a> for Style {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Style {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_Block::mod_Content::mod_Link::Style::Page { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_Block::mod_Content::mod_Link::Style::Page { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fields<'a> {
    pub value: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Fields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Fields<'a> {
    fn get_size(&self) -> usize {
        0
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.value { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Bookmark<'a> {
    pub id: Cow<'a, str>,
    pub url: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Url<'a>>,
    pub title: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Title<'a>>,
    pub description: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Description<'a>>,
    pub imageHash: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::ImageHash<'a>>,
    pub faviconHash: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::FaviconHash<'a>>,
    pub type_pb: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Type>,
    pub targetObjectId: Option<mod_Event::mod_Block::mod_Fill::mod_Bookmark::TargetObjectId<'a>>,
}

impl<'a> MessageRead<'a> for Bookmark<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.url = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Url>(bytes)?),
                Ok(26) => msg.title = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Title>(bytes)?),
                Ok(34) => msg.description = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Description>(bytes)?),
                Ok(42) => msg.imageHash = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::ImageHash>(bytes)?),
                Ok(50) => msg.faviconHash = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::FaviconHash>(bytes)?),
                Ok(58) => msg.type_pb = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::Type>(bytes)?),
                Ok(66) => msg.targetObjectId = Some(r.read_message::<mod_Event::mod_Block::mod_Fill::mod_Bookmark::TargetObjectId>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Bookmark<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.title.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.description.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.imageHash.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.faviconHash.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.targetObjectId.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.title { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.description { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.imageHash { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.faviconHash { w.write_with_tag(50, |w| w.write_message(s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.targetObjectId { w.write_with_tag(66, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Bookmark {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Url<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Url<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Url<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Title<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Title<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Title<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Description<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Description<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Description<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ImageHash<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ImageHash<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ImageHash<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FaviconHash<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FaviconHash<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FaviconHash<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Type {
    pub value: anytype::model::mod_LinkPreview::Type,
}

impl<'a> MessageRead<'a> for Type {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.value = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Type {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_LinkPreview::Type::Unknown { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_LinkPreview::Type::Unknown { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetObjectId<'a> {
    pub value: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TargetObjectId<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.value = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TargetObjectId<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.value == "" { 0 } else { 1 + sizeof_len((&self.value).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != "" { w.write_with_tag(10, |w| w.write_string(&**&self.value))?; }
        Ok(())
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dataview {
}

impl<'a> MessageRead<'a> for Dataview {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Dataview {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Dataview {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViewSet<'a> {
    pub id: Cow<'a, str>,
    pub viewId: Cow<'a, str>,
    pub view: Option<anytype::model::mod_Block::mod_Content::mod_Dataview::View<'a>>,
}

impl<'a> MessageRead<'a> for ViewSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.view = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::View>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ViewSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
        + self.view.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.viewId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.viewId))?; }
        if let Some(ref s) = self.view { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViewUpdate<'a> {
    pub id: Cow<'a, str>,
    pub viewId: Cow<'a, str>,
    pub filter: Vec<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Filter<'a>>,
    pub relation: Vec<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Relation<'a>>,
    pub sort: Vec<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Sort<'a>>,
    pub fields: Option<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Fields<'a>>,
}

impl<'a> MessageRead<'a> for ViewUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.filter.push(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Filter>(bytes)?),
                Ok(34) => msg.relation.push(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Relation>(bytes)?),
                Ok(42) => msg.sort.push(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Sort>(bytes)?),
                Ok(50) => msg.fields = Some(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::Fields>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ViewUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
        + self.filter.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.relation.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.sort.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.viewId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.viewId))?; }
        for s in &self.filter { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.relation { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.sort { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ViewUpdate {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fields<'a> {
    pub type_pb: anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Type,
    pub name: Cow<'a, str>,
    pub coverRelationKey: Cow<'a, str>,
    pub hideIcon: bool,
    pub cardSize: anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Size,
    pub coverFit: bool,
    pub groupRelationKey: Cow<'a, str>,
    pub endRelationKey: Cow<'a, str>,
    pub groupBackgroundColors: bool,
    pub pageLimit: i32,
    pub defaultTemplateId: Cow<'a, str>,
    pub defaultObjectTypeId: Cow<'a, str>,
    pub wrapContent: bool,
}

impl<'a> MessageRead<'a> for Fields<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.type_pb = r.read_enum(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.coverRelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.hideIcon = r.read_bool(bytes)?,
                Ok(40) => msg.cardSize = r.read_enum(bytes)?,
                Ok(48) => msg.coverFit = r.read_bool(bytes)?,
                Ok(58) => msg.groupRelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(130) => msg.endRelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.groupBackgroundColors = r.read_bool(bytes)?,
                Ok(72) => msg.pageLimit = r.read_int32(bytes)?,
                Ok(82) => msg.defaultTemplateId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.defaultObjectTypeId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(136) => msg.wrapContent = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Fields<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.type_pb == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Type::Table { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.coverRelationKey == "" { 0 } else { 1 + sizeof_len((&self.coverRelationKey).len()) }
        + if self.hideIcon == false { 0 } else { 1 + sizeof_varint(*(&self.hideIcon) as u64) }
        + if self.cardSize == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Size::Small { 0 } else { 1 + sizeof_varint(*(&self.cardSize) as u64) }
        + if self.coverFit == false { 0 } else { 1 + sizeof_varint(*(&self.coverFit) as u64) }
        + if self.groupRelationKey == "" { 0 } else { 1 + sizeof_len((&self.groupRelationKey).len()) }
        + if self.endRelationKey == "" { 0 } else { 2 + sizeof_len((&self.endRelationKey).len()) }
        + if self.groupBackgroundColors == false { 0 } else { 1 + sizeof_varint(*(&self.groupBackgroundColors) as u64) }
        + if self.pageLimit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.pageLimit) as u64) }
        + if self.defaultTemplateId == "" { 0 } else { 1 + sizeof_len((&self.defaultTemplateId).len()) }
        + if self.defaultObjectTypeId == "" { 0 } else { 1 + sizeof_len((&self.defaultObjectTypeId).len()) }
        + if self.wrapContent == false { 0 } else { 2 + sizeof_varint(*(&self.wrapContent) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.type_pb != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Type::Table { w.write_with_tag(8, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.coverRelationKey != "" { w.write_with_tag(26, |w| w.write_string(&**&self.coverRelationKey))?; }
        if self.hideIcon != false { w.write_with_tag(32, |w| w.write_bool(*&self.hideIcon))?; }
        if self.cardSize != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Size::Small { w.write_with_tag(40, |w| w.write_enum(*&self.cardSize as i32))?; }
        if self.coverFit != false { w.write_with_tag(48, |w| w.write_bool(*&self.coverFit))?; }
        if self.groupRelationKey != "" { w.write_with_tag(58, |w| w.write_string(&**&self.groupRelationKey))?; }
        if self.endRelationKey != "" { w.write_with_tag(130, |w| w.write_string(&**&self.endRelationKey))?; }
        if self.groupBackgroundColors != false { w.write_with_tag(64, |w| w.write_bool(*&self.groupBackgroundColors))?; }
        if self.pageLimit != 0i32 { w.write_with_tag(72, |w| w.write_int32(*&self.pageLimit))?; }
        if self.defaultTemplateId != "" { w.write_with_tag(82, |w| w.write_string(&**&self.defaultTemplateId))?; }
        if self.defaultObjectTypeId != "" { w.write_with_tag(122, |w| w.write_string(&**&self.defaultObjectTypeId))?; }
        if self.wrapContent != false { w.write_with_tag(136, |w| w.write_bool(*&self.wrapContent))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Filter<'a> {
    pub operation: mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation<'a>,
}

impl<'a> MessageRead<'a> for Filter<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::add(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Add>(bytes)?),
                Ok(18) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::remove(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Remove>(bytes)?),
                Ok(26) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::update(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Update>(bytes)?),
                Ok(34) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::move_pb(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Move>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Filter<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.operation {
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::add(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::remove(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::update(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::move_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.operation {            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::add(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::remove(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::update(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::move_pb(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::OneOfoperation::None => {},
    }        Ok(())
    }
}

pub mod mod_Filter {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub afterId: Cow<'a, str>,
    pub items: Vec<anytype::model::mod_Block::mod_Content::mod_Dataview::Filter<'a>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.items.push(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Filter>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.items.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.items { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub id: Cow<'a, str>,
    pub item: Option<anytype::model::mod_Block::mod_Content::mod_Dataview::Filter<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.item = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Filter>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.item.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.item { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Move<'a> {
    pub afterId: Cow<'a, str>,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Move<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Move<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.ids { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoperation<'a> {
    add(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Add<'a>),
    remove(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Remove<'a>),
    update(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Update<'a>),
    move_pb(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Filter::Move<'a>),
    None,
}

impl<'a> Default for OneOfoperation<'a> {
    fn default() -> Self {
        OneOfoperation::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation<'a> {
    pub operation: mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation<'a>,
}

impl<'a> MessageRead<'a> for Relation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::add(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Add>(bytes)?),
                Ok(18) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::remove(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Remove>(bytes)?),
                Ok(26) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::update(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Update>(bytes)?),
                Ok(34) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::move_pb(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Move>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Relation<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.operation {
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::add(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::remove(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::update(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::move_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.operation {            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::add(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::remove(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::update(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::move_pb(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::OneOfoperation::None => {},
    }        Ok(())
    }
}

pub mod mod_Relation {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub afterId: Cow<'a, str>,
    pub items: Vec<anytype::model::mod_Block::mod_Content::mod_Dataview::Relation<'a>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.items.push(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Relation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.items.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.items { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub id: Cow<'a, str>,
    pub item: Option<anytype::model::mod_Block::mod_Content::mod_Dataview::Relation<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.item = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Relation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.item.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.item { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Move<'a> {
    pub afterId: Cow<'a, str>,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Move<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Move<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.ids { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoperation<'a> {
    add(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Add<'a>),
    remove(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Remove<'a>),
    update(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Update<'a>),
    move_pb(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Relation::Move<'a>),
    None,
}

impl<'a> Default for OneOfoperation<'a> {
    fn default() -> Self {
        OneOfoperation::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sort<'a> {
    pub operation: mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation<'a>,
}

impl<'a> MessageRead<'a> for Sort<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::add(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Add>(bytes)?),
                Ok(18) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::remove(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Remove>(bytes)?),
                Ok(26) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::update(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Update>(bytes)?),
                Ok(34) => msg.operation = mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::move_pb(r.read_message::<mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Move>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Sort<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.operation {
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::add(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::remove(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::update(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::move_pb(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.operation {            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::add(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::remove(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::update(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::move_pb(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::OneOfoperation::None => {},
    }        Ok(())
    }
}

pub mod mod_Sort {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Add<'a> {
    pub afterId: Cow<'a, str>,
    pub items: Vec<anytype::model::mod_Block::mod_Content::mod_Dataview::Sort<'a>>,
}

impl<'a> MessageRead<'a> for Add<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.items.push(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Sort>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Add<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.items.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.items { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Remove<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Remove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Remove<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub id: Cow<'a, str>,
    pub item: Option<anytype::model::mod_Block::mod_Content::mod_Dataview::Sort<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.item = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::Sort>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.item.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.item { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Move<'a> {
    pub afterId: Cow<'a, str>,
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Move<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Move<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.afterId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.afterId))?; }
        for s in &self.ids { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfoperation<'a> {
    add(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Add<'a>),
    remove(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Remove<'a>),
    update(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Update<'a>),
    move_pb(mod_Event::mod_Block::mod_Dataview::mod_ViewUpdate::mod_Sort::Move<'a>),
    None,
}

impl<'a> Default for OneOfoperation<'a> {
    fn default() -> Self {
        OneOfoperation::None
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViewDelete<'a> {
    pub id: Cow<'a, str>,
    pub viewId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ViewDelete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ViewDelete<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.viewId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.viewId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViewOrder<'a> {
    pub id: Cow<'a, str>,
    pub viewIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ViewOrder<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.viewIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ViewOrder<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.viewIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.viewIds { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SourceSet<'a> {
    pub id: Cow<'a, str>,
    pub source: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for SourceSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.source.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SourceSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.source.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.source { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OldRelationDelete<'a> {
    pub id: Cow<'a, str>,
    pub relationKey: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for OldRelationDelete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OldRelationDelete<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.relationKey == "" { 0 } else { 1 + sizeof_len((&self.relationKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.relationKey != "" { w.write_with_tag(18, |w| w.write_string(&**&self.relationKey))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OldRelationSet<'a> {
    pub id: Cow<'a, str>,
    pub relationKey: Cow<'a, str>,
    pub relation: Option<anytype::model::Relation<'a>>,
}

impl<'a> MessageRead<'a> for OldRelationSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.relation = Some(r.read_message::<anytype::model::Relation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OldRelationSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.relationKey == "" { 0 } else { 1 + sizeof_len((&self.relationKey).len()) }
        + self.relation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.relationKey != "" { w.write_with_tag(18, |w| w.write_string(&**&self.relationKey))?; }
        if let Some(ref s) = self.relation { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationDelete<'a> {
    pub id: Cow<'a, str>,
    pub relationKeys: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for RelationDelete<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relationKeys.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationDelete<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.relationKeys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.relationKeys { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationSet<'a> {
    pub id: Cow<'a, str>,
    pub relationLinks: Vec<anytype::model::RelationLink<'a>>,
}

impl<'a> MessageRead<'a> for RelationSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relationLinks.push(r.read_message::<anytype::model::RelationLink>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        for s in &self.relationLinks { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupOrderUpdate<'a> {
    pub id: Cow<'a, str>,
    pub groupOrder: Option<anytype::model::mod_Block::mod_Content::mod_Dataview::GroupOrder<'a>>,
}

impl<'a> MessageRead<'a> for GroupOrderUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.groupOrder = Some(r.read_message::<anytype::model::mod_Block::mod_Content::mod_Dataview::GroupOrder>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupOrderUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.groupOrder.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.groupOrder { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectOrderUpdate<'a> {
    pub id: Cow<'a, str>,
    pub viewId: Cow<'a, str>,
    pub groupId: Cow<'a, str>,
    pub sliceChanges: Vec<mod_Event::mod_Block::mod_Dataview::SliceChange<'a>>,
}

impl<'a> MessageRead<'a> for ObjectOrderUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.groupId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.sliceChanges.push(r.read_message::<mod_Event::mod_Block::mod_Dataview::SliceChange>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjectOrderUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
        + if self.groupId == "" { 0 } else { 1 + sizeof_len((&self.groupId).len()) }
        + self.sliceChanges.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.viewId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.viewId))?; }
        if self.groupId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.groupId))?; }
        for s in &self.sliceChanges { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SliceChange<'a> {
    pub op: mod_Event::mod_Block::mod_Dataview::SliceOperation,
    pub ids: Vec<Cow<'a, str>>,
    pub afterId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SliceChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.op = r.read_enum(bytes)?,
                Ok(18) => msg.ids.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(26) => msg.afterId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SliceChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.op == anytype::mod_Event::mod_Block::mod_Dataview::SliceOperation::SliceOperationNone { 0 } else { 1 + sizeof_varint(*(&self.op) as u64) }
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.afterId == "" { 0 } else { 1 + sizeof_len((&self.afterId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.op != anytype::mod_Event::mod_Block::mod_Dataview::SliceOperation::SliceOperationNone { w.write_with_tag(8, |w| w.write_enum(*&self.op as i32))?; }
        for s in &self.ids { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        if self.afterId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.afterId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TargetObjectIdSet<'a> {
    pub id: Cow<'a, str>,
    pub targetObjectId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TargetObjectIdSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.targetObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TargetObjectIdSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.targetObjectId == "" { 0 } else { 1 + sizeof_len((&self.targetObjectId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.targetObjectId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.targetObjectId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IsCollectionSet<'a> {
    pub id: Cow<'a, str>,
    pub value: bool,
}

impl<'a> MessageRead<'a> for IsCollectionSet<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.value = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IsCollectionSet<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.value == false { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.value != false { w.write_with_tag(16, |w| w.write_bool(*&self.value))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SliceOperation {
    SliceOperationNone = 0,
    SliceOperationAdd = 1,
    SliceOperationMove = 2,
    SliceOperationRemove = 3,
    SliceOperationReplace = 4,
}

impl Default for SliceOperation {
    fn default() -> Self {
        SliceOperation::SliceOperationNone
    }
}

impl From<i32> for SliceOperation {
    fn from(i: i32) -> Self {
        match i {
            0 => SliceOperation::SliceOperationNone,
            1 => SliceOperation::SliceOperationAdd,
            2 => SliceOperation::SliceOperationMove,
            3 => SliceOperation::SliceOperationRemove,
            4 => SliceOperation::SliceOperationReplace,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SliceOperation {
    fn from(s: &'a str) -> Self {
        match s {
            "SliceOperationNone" => SliceOperation::SliceOperationNone,
            "SliceOperationAdd" => SliceOperation::SliceOperationAdd,
            "SliceOperationMove" => SliceOperation::SliceOperationMove,
            "SliceOperationRemove" => SliceOperation::SliceOperationRemove,
            "SliceOperationReplace" => SliceOperation::SliceOperationReplace,
            _ => Self::default(),
        }
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
}

impl<'a> MessageRead<'a> for User {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for User {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_User {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Block {
}

impl<'a> MessageRead<'a> for Block {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Block {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Block {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Join {
    pub account: Option<mod_Event::Account>,
}

impl<'a> MessageRead<'a> for Join {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = Some(r.read_message::<mod_Event::Account>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Join {
    fn get_size(&self) -> usize {
        0
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.account { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Left {
    pub account: Option<mod_Event::Account>,
}

impl<'a> MessageRead<'a> for Left {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = Some(r.read_message::<mod_Event::Account>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Left {
    fn get_size(&self) -> usize {
        0
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.account { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TextRange<'a> {
    pub account: Option<mod_Event::Account>,
    pub blockId: Cow<'a, str>,
    pub range: Option<anytype::model::Range>,
}

impl<'a> MessageRead<'a> for TextRange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = Some(r.read_message::<mod_Event::Account>(bytes)?),
                Ok(18) => msg.blockId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.range = Some(r.read_message::<anytype::model::Range>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TextRange<'a> {
    fn get_size(&self) -> usize {
        0
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.blockId == "" { 0 } else { 1 + sizeof_len((&self.blockId).len()) }
        + self.range.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.account { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.blockId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.blockId))?; }
        if let Some(ref s) = self.range { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SelectRange<'a> {
    pub account: Option<mod_Event::Account>,
    pub blockIdsArray: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for SelectRange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.account = Some(r.read_message::<mod_Event::Account>(bytes)?),
                Ok(18) => msg.blockIdsArray.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SelectRange<'a> {
    fn get_size(&self) -> usize {
        0
        + self.account.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.blockIdsArray.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.account { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.blockIdsArray { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ping {
    pub index: i32,
}

impl<'a> MessageRead<'a> for Ping {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.index = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Ping {
    fn get_size(&self) -> usize {
        0
        + if self.index == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.index != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.index))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Process {
}

impl<'a> MessageRead<'a> for Process {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Process {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Process {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct New<'a> {
    pub process: Option<mod_EventModel::Process<'a>>,
}

impl<'a> MessageRead<'a> for New<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.process = Some(r.read_message::<mod_EventModel::Process>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for New<'a> {
    fn get_size(&self) -> usize {
        0
        + self.process.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.process { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub process: Option<mod_EventModel::Process<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.process = Some(r.read_message::<mod_EventModel::Process>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + self.process.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.process { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Done<'a> {
    pub process: Option<mod_EventModel::Process<'a>>,
}

impl<'a> MessageRead<'a> for Done<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.process = Some(r.read_message::<mod_EventModel::Process>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Done<'a> {
    fn get_size(&self) -> usize {
        0
        + self.process.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.process { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Status {
}

impl<'a> MessageRead<'a> for Status {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Status {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Status {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Thread<'a> {
    pub summary: Option<mod_Event::mod_Status::mod_Thread::Summary>,
    pub cafe: Option<mod_Event::mod_Status::mod_Thread::Cafe>,
    pub accounts: Vec<mod_Event::mod_Status::mod_Thread::Account<'a>>,
}

impl<'a> MessageRead<'a> for Thread<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.summary = Some(r.read_message::<mod_Event::mod_Status::mod_Thread::Summary>(bytes)?),
                Ok(18) => msg.cafe = Some(r.read_message::<mod_Event::mod_Status::mod_Thread::Cafe>(bytes)?),
                Ok(26) => msg.accounts.push(r.read_message::<mod_Event::mod_Status::mod_Thread::Account>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Thread<'a> {
    fn get_size(&self) -> usize {
        0
        + self.summary.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.cafe.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.accounts.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.summary { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cafe { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.accounts { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Thread {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Summary {
    pub status: mod_Event::mod_Status::mod_Thread::SyncStatus,
}

impl<'a> MessageRead<'a> for Summary {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Summary {
    fn get_size(&self) -> usize {
        0
        + if self.status == anytype::mod_Event::mod_Status::mod_Thread::SyncStatus::Unknown { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != anytype::mod_Event::mod_Status::mod_Thread::SyncStatus::Unknown { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Cafe {
    pub status: mod_Event::mod_Status::mod_Thread::SyncStatus,
    pub lastPulled: i64,
    pub lastPushSucceed: bool,
    pub files: Option<mod_Event::mod_Status::mod_Thread::mod_Cafe::PinStatus>,
}

impl<'a> MessageRead<'a> for Cafe {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.status = r.read_enum(bytes)?,
                Ok(16) => msg.lastPulled = r.read_int64(bytes)?,
                Ok(24) => msg.lastPushSucceed = r.read_bool(bytes)?,
                Ok(34) => msg.files = Some(r.read_message::<mod_Event::mod_Status::mod_Thread::mod_Cafe::PinStatus>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Cafe {
    fn get_size(&self) -> usize {
        0
        + if self.status == anytype::mod_Event::mod_Status::mod_Thread::SyncStatus::Unknown { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.lastPulled == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.lastPulled) as u64) }
        + if self.lastPushSucceed == false { 0 } else { 1 + sizeof_varint(*(&self.lastPushSucceed) as u64) }
        + self.files.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.status != anytype::mod_Event::mod_Status::mod_Thread::SyncStatus::Unknown { w.write_with_tag(8, |w| w.write_enum(*&self.status as i32))?; }
        if self.lastPulled != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.lastPulled))?; }
        if self.lastPushSucceed != false { w.write_with_tag(24, |w| w.write_bool(*&self.lastPushSucceed))?; }
        if let Some(ref s) = self.files { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Cafe {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PinStatus {
    pub pinning: i32,
    pub pinned: i32,
    pub failed: i32,
    pub updated: i64,
}

impl<'a> MessageRead<'a> for PinStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.pinning = r.read_int32(bytes)?,
                Ok(16) => msg.pinned = r.read_int32(bytes)?,
                Ok(24) => msg.failed = r.read_int32(bytes)?,
                Ok(32) => msg.updated = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for PinStatus {
    fn get_size(&self) -> usize {
        0
        + if self.pinning == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.pinning) as u64) }
        + if self.pinned == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.pinned) as u64) }
        + if self.failed == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.failed) as u64) }
        + if self.updated == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.updated) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.pinning != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.pinning))?; }
        if self.pinned != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.pinned))?; }
        if self.failed != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.failed))?; }
        if self.updated != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.updated))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Account<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub imageHash: Cow<'a, str>,
    pub online: bool,
    pub lastPulled: i64,
    pub lastEdited: i64,
    pub devices: Vec<mod_Event::mod_Status::mod_Thread::Device<'a>>,
}

impl<'a> MessageRead<'a> for Account<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.imageHash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.online = r.read_bool(bytes)?,
                Ok(40) => msg.lastPulled = r.read_int64(bytes)?,
                Ok(48) => msg.lastEdited = r.read_int64(bytes)?,
                Ok(58) => msg.devices.push(r.read_message::<mod_Event::mod_Status::mod_Thread::Device>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Account<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.imageHash == "" { 0 } else { 1 + sizeof_len((&self.imageHash).len()) }
        + if self.online == false { 0 } else { 1 + sizeof_varint(*(&self.online) as u64) }
        + if self.lastPulled == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.lastPulled) as u64) }
        + if self.lastEdited == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.lastEdited) as u64) }
        + self.devices.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.imageHash != "" { w.write_with_tag(26, |w| w.write_string(&**&self.imageHash))?; }
        if self.online != false { w.write_with_tag(32, |w| w.write_bool(*&self.online))?; }
        if self.lastPulled != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.lastPulled))?; }
        if self.lastEdited != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.lastEdited))?; }
        for s in &self.devices { w.write_with_tag(58, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Device<'a> {
    pub name: Cow<'a, str>,
    pub online: bool,
    pub lastPulled: i64,
    pub lastEdited: i64,
}

impl<'a> MessageRead<'a> for Device<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.online = r.read_bool(bytes)?,
                Ok(24) => msg.lastPulled = r.read_int64(bytes)?,
                Ok(32) => msg.lastEdited = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Device<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.online == false { 0 } else { 1 + sizeof_varint(*(&self.online) as u64) }
        + if self.lastPulled == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.lastPulled) as u64) }
        + if self.lastEdited == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.lastEdited) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.online != false { w.write_with_tag(16, |w| w.write_bool(*&self.online))?; }
        if self.lastPulled != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.lastPulled))?; }
        if self.lastEdited != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.lastEdited))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyncStatus {
    Unknown = 0,
    Offline = 1,
    Syncing = 2,
    Synced = 3,
    Failed = 4,
    IncompatibleVersion = 5,
    NetworkNeedsUpdate = 6,
}

impl Default for SyncStatus {
    fn default() -> Self {
        SyncStatus::Unknown
    }
}

impl From<i32> for SyncStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => SyncStatus::Unknown,
            1 => SyncStatus::Offline,
            2 => SyncStatus::Syncing,
            3 => SyncStatus::Synced,
            4 => SyncStatus::Failed,
            5 => SyncStatus::IncompatibleVersion,
            6 => SyncStatus::NetworkNeedsUpdate,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SyncStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "Unknown" => SyncStatus::Unknown,
            "Offline" => SyncStatus::Offline,
            "Syncing" => SyncStatus::Syncing,
            "Synced" => SyncStatus::Synced,
            "Failed" => SyncStatus::Failed,
            "IncompatibleVersion" => SyncStatus::IncompatibleVersion,
            "NetworkNeedsUpdate" => SyncStatus::NetworkNeedsUpdate,
            _ => Self::default(),
        }
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct File {
}

impl<'a> MessageRead<'a> for File {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for File {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_File {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LimitReached<'a> {
    pub spaceId: Cow<'a, str>,
    pub fileId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for LimitReached<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LimitReached<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.fileId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.fileId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SpaceUsage<'a> {
    pub bytesUsage: u64,
    pub spaceId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SpaceUsage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.bytesUsage = r.read_uint64(bytes)?,
                Ok(18) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SpaceUsage<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.bytesUsage == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.bytesUsage) as u64) }
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.bytesUsage != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.bytesUsage))?; }
        if self.spaceId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.spaceId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LocalUsage {
    pub localBytesUsage: u64,
}

impl<'a> MessageRead<'a> for LocalUsage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.localBytesUsage = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LocalUsage {
    fn get_size(&self) -> usize {
        0
        + if self.localBytesUsage == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.localBytesUsage) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.localBytesUsage != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.localBytesUsage))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LimitUpdated {
    pub bytesLimit: u64,
}

impl<'a> MessageRead<'a> for LimitUpdated {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.bytesLimit = r.read_uint64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LimitUpdated {
    fn get_size(&self) -> usize {
        0
        + if self.bytesLimit == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.bytesLimit) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.bytesLimit != 0u64 { w.write_with_tag(8, |w| w.write_uint64(*&self.bytesLimit))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Membership {
}

impl<'a> MessageRead<'a> for Membership {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Membership {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Membership {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub data: Option<anytype::model::Membership<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.data = Some(r.read_message::<anytype::model::Membership>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.data { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TiersUpdate<'a> {
    pub tiers: Vec<anytype::model::MembershipTierData<'a>>,
}

impl<'a> MessageRead<'a> for TiersUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tiers.push(r.read_message::<anytype::model::MembershipTierData>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TiersUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + self.tiers.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.tiers { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MembershipV2 {
}

impl<'a> MessageRead<'a> for MembershipV2 {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MembershipV2 {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_MembershipV2 {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub data: Option<anytype::model::mod_MembershipV2::Data<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.data = Some(r.read_message::<anytype::model::mod_MembershipV2::Data>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + self.data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.data { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ProductsUpdate<'a> {
    pub products: Vec<anytype::model::mod_MembershipV2::Product<'a>>,
}

impl<'a> MessageRead<'a> for ProductsUpdate<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.products.push(r.read_message::<anytype::model::mod_MembershipV2::Product>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ProductsUpdate<'a> {
    fn get_size(&self) -> usize {
        0
        + self.products.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.products { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Notification {
}

impl<'a> MessageRead<'a> for Notification {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Notification {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Notification {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Send<'a> {
    pub notification: Option<anytype::model::Notification<'a>>,
}

impl<'a> MessageRead<'a> for Send<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.notification = Some(r.read_message::<anytype::model::Notification>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Send<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notification.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.notification { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub notification: Option<anytype::model::Notification<'a>>,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.notification = Some(r.read_message::<anytype::model::Notification>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + self.notification.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.notification { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Payload {
}

impl<'a> MessageRead<'a> for Payload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Payload {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Payload {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Broadcast<'a> {
    pub payload: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Broadcast<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.payload = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Broadcast<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.payload == "" { 0 } else { 1 + sizeof_len((&self.payload).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.payload != "" { w.write_with_tag(10, |w| w.write_string(&**&self.payload))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Space {
}

impl<'a> MessageRead<'a> for Space {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Space {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Space {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SyncStatus {
}

impl<'a> MessageRead<'a> for SyncStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SyncStatus {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_SyncStatus {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub id: Cow<'a, str>,
    pub status: mod_Event::mod_Space::Status,
    pub network: mod_Event::mod_Space::Network,
    pub error: mod_Event::mod_Space::SyncError,
    pub syncingObjectsCounter: i64,
    pub notSyncedFilesCounter: i64,
    pub uploadingFilesCounter: i64,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.status = r.read_enum(bytes)?,
                Ok(24) => msg.network = r.read_enum(bytes)?,
                Ok(32) => msg.error = r.read_enum(bytes)?,
                Ok(40) => msg.syncingObjectsCounter = r.read_int64(bytes)?,
                Ok(48) => msg.notSyncedFilesCounter = r.read_int64(bytes)?,
                Ok(56) => msg.uploadingFilesCounter = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.status == anytype::mod_Event::mod_Space::Status::Synced { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.network == anytype::mod_Event::mod_Space::Network::Anytype { 0 } else { 1 + sizeof_varint(*(&self.network) as u64) }
        + if self.error == anytype::mod_Event::mod_Space::SyncError::Null { 0 } else { 1 + sizeof_varint(*(&self.error) as u64) }
        + if self.syncingObjectsCounter == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.syncingObjectsCounter) as u64) }
        + if self.notSyncedFilesCounter == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.notSyncedFilesCounter) as u64) }
        + if self.uploadingFilesCounter == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.uploadingFilesCounter) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.status != anytype::mod_Event::mod_Space::Status::Synced { w.write_with_tag(16, |w| w.write_enum(*&self.status as i32))?; }
        if self.network != anytype::mod_Event::mod_Space::Network::Anytype { w.write_with_tag(24, |w| w.write_enum(*&self.network as i32))?; }
        if self.error != anytype::mod_Event::mod_Space::SyncError::Null { w.write_with_tag(32, |w| w.write_enum(*&self.error as i32))?; }
        if self.syncingObjectsCounter != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.syncingObjectsCounter))?; }
        if self.notSyncedFilesCounter != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.notSyncedFilesCounter))?; }
        if self.uploadingFilesCounter != 0i64 { w.write_with_tag(56, |w| w.write_int64(*&self.uploadingFilesCounter))?; }
        Ok(())
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Synced = 0,
    Syncing = 1,
    Error = 2,
    Offline = 3,
    NetworkNeedsUpdate = 4,
}

impl Default for Status {
    fn default() -> Self {
        Status::Synced
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            0 => Status::Synced,
            1 => Status::Syncing,
            2 => Status::Error,
            3 => Status::Offline,
            4 => Status::NetworkNeedsUpdate,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "Synced" => Status::Synced,
            "Syncing" => Status::Syncing,
            "Error" => Status::Error,
            "Offline" => Status::Offline,
            "NetworkNeedsUpdate" => Status::NetworkNeedsUpdate,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Network {
    Anytype = 0,
    SelfHost = 1,
    LocalOnly = 2,
}

impl Default for Network {
    fn default() -> Self {
        Network::Anytype
    }
}

impl From<i32> for Network {
    fn from(i: i32) -> Self {
        match i {
            0 => Network::Anytype,
            1 => Network::SelfHost,
            2 => Network::LocalOnly,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Network {
    fn from(s: &'a str) -> Self {
        match s {
            "Anytype" => Network::Anytype,
            "SelfHost" => Network::SelfHost,
            "LocalOnly" => Network::LocalOnly,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SyncError {
    Null = 0,
    StorageLimitExceed = 1,
    IncompatibleVersion = 2,
    NetworkError = 3,
}

impl Default for SyncError {
    fn default() -> Self {
        SyncError::Null
    }
}

impl From<i32> for SyncError {
    fn from(i: i32) -> Self {
        match i {
            0 => SyncError::Null,
            1 => SyncError::StorageLimitExceed,
            2 => SyncError::IncompatibleVersion,
            3 => SyncError::NetworkError,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SyncError {
    fn from(s: &'a str) -> Self {
        match s {
            "Null" => SyncError::Null,
            "StorageLimitExceed" => SyncError::StorageLimitExceed,
            "IncompatibleVersion" => SyncError::IncompatibleVersion,
            "NetworkError" => SyncError::NetworkError,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct P2PStatus {
}

impl<'a> MessageRead<'a> for P2PStatus {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for P2PStatus {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_P2PStatus {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Update<'a> {
    pub spaceId: Cow<'a, str>,
    pub status: mod_Event::mod_P2PStatus::Status,
    pub devicesCounter: i64,
}

impl<'a> MessageRead<'a> for Update<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.status = r.read_enum(bytes)?,
                Ok(24) => msg.devicesCounter = r.read_int64(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Update<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.status == anytype::mod_Event::mod_P2PStatus::Status::NotConnected { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.devicesCounter == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.devicesCounter) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.status != anytype::mod_Event::mod_P2PStatus::Status::NotConnected { w.write_with_tag(16, |w| w.write_enum(*&self.status as i32))?; }
        if self.devicesCounter != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.devicesCounter))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    NotConnected = 0,
    NotPossible = 1,
    Connected = 2,
    Restricted = 3,
}

impl Default for Status {
    fn default() -> Self {
        Status::NotConnected
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            0 => Status::NotConnected,
            1 => Status::NotPossible,
            2 => Status::Connected,
            3 => Status::Restricted,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "NotConnected" => Status::NotConnected,
            "NotPossible" => Status::NotPossible,
            "Connected" => Status::Connected,
            "Restricted" => Status::Restricted,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Import {
}

impl<'a> MessageRead<'a> for Import {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Import {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Import {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Finish<'a> {
    pub rootCollectionID: Cow<'a, str>,
    pub objectsCount: i64,
    pub importType: anytype::model::mod_Import::Type,
}

impl<'a> MessageRead<'a> for Finish<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rootCollectionID = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.objectsCount = r.read_int64(bytes)?,
                Ok(24) => msg.importType = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Finish<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.rootCollectionID == "" { 0 } else { 1 + sizeof_len((&self.rootCollectionID).len()) }
        + if self.objectsCount == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.objectsCount) as u64) }
        + if self.importType == anytype::model::mod_Import::Type::Notion { 0 } else { 1 + sizeof_varint(*(&self.importType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.rootCollectionID != "" { w.write_with_tag(10, |w| w.write_string(&**&self.rootCollectionID))?; }
        if self.objectsCount != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.objectsCount))?; }
        if self.importType != anytype::model::mod_Import::Type::Notion { w.write_with_tag(24, |w| w.write_enum(*&self.importType as i32))?; }
        Ok(())
    }
}

}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ResponseEvent<'a> {
    pub messages: Vec<mod_Event::Message<'a>>,
    pub contextId: Cow<'a, str>,
    pub traceId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ResponseEvent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.messages.push(r.read_message::<mod_Event::Message>(bytes)?),
                Ok(18) => msg.contextId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.traceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ResponseEvent<'a> {
    fn get_size(&self) -> usize {
        0
        + self.messages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.contextId == "" { 0 } else { 1 + sizeof_len((&self.contextId).len()) }
        + if self.traceId == "" { 0 } else { 1 + sizeof_len((&self.traceId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.messages { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.contextId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.contextId))?; }
        if self.traceId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.traceId))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct EventModel {
}

impl<'a> MessageRead<'a> for EventModel {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for EventModel {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_EventModel {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Process<'a> {
    pub id: Cow<'a, str>,
    pub state: mod_EventModel::mod_Process::State,
    pub progress: Option<mod_EventModel::mod_Process::Progress<'a>>,
    pub spaceId: Cow<'a, str>,
    pub error: Cow<'a, str>,
    pub message: mod_EventModel::mod_Process::OneOfmessage,
}

impl<'a> MessageRead<'a> for Process<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.state = r.read_enum(bytes)?,
                Ok(34) => msg.progress = Some(r.read_message::<mod_EventModel::mod_Process::Progress>(bytes)?),
                Ok(42) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.error = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.message = mod_EventModel::mod_Process::OneOfmessage::dropFiles(r.read_message::<mod_EventModel::mod_Process::DropFiles>(bytes)?),
                Ok(58) => msg.message = mod_EventModel::mod_Process::OneOfmessage::import(r.read_message::<mod_EventModel::mod_Process::Import>(bytes)?),
                Ok(66) => msg.message = mod_EventModel::mod_Process::OneOfmessage::export(r.read_message::<mod_EventModel::mod_Process::Export>(bytes)?),
                Ok(74) => msg.message = mod_EventModel::mod_Process::OneOfmessage::saveFile(r.read_message::<mod_EventModel::mod_Process::SaveFile>(bytes)?),
                Ok(82) => msg.message = mod_EventModel::mod_Process::OneOfmessage::migration(r.read_message::<mod_EventModel::mod_Process::Migration>(bytes)?),
                Ok(98) => msg.message = mod_EventModel::mod_Process::OneOfmessage::preloadFile(r.read_message::<mod_EventModel::mod_Process::PreloadFile>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Process<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.state == anytype::mod_EventModel::mod_Process::State::None { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
        + self.progress.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.error == "" { 0 } else { 1 + sizeof_len((&self.error).len()) }
        + match self.message {
            mod_EventModel::mod_Process::OneOfmessage::dropFiles(ref m) => 1 + sizeof_len((m).get_size()),
            mod_EventModel::mod_Process::OneOfmessage::import(ref m) => 1 + sizeof_len((m).get_size()),
            mod_EventModel::mod_Process::OneOfmessage::export(ref m) => 1 + sizeof_len((m).get_size()),
            mod_EventModel::mod_Process::OneOfmessage::saveFile(ref m) => 1 + sizeof_len((m).get_size()),
            mod_EventModel::mod_Process::OneOfmessage::migration(ref m) => 1 + sizeof_len((m).get_size()),
            mod_EventModel::mod_Process::OneOfmessage::preloadFile(ref m) => 1 + sizeof_len((m).get_size()),
            mod_EventModel::mod_Process::OneOfmessage::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.state != anytype::mod_EventModel::mod_Process::State::None { w.write_with_tag(24, |w| w.write_enum(*&self.state as i32))?; }
        if let Some(ref s) = self.progress { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.spaceId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceId))?; }
        if self.error != "" { w.write_with_tag(90, |w| w.write_string(&**&self.error))?; }
        match self.message {            mod_EventModel::mod_Process::OneOfmessage::dropFiles(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            mod_EventModel::mod_Process::OneOfmessage::import(ref m) => { w.write_with_tag(58, |w| w.write_message(m))? },
            mod_EventModel::mod_Process::OneOfmessage::export(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            mod_EventModel::mod_Process::OneOfmessage::saveFile(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            mod_EventModel::mod_Process::OneOfmessage::migration(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            mod_EventModel::mod_Process::OneOfmessage::preloadFile(ref m) => { w.write_with_tag(98, |w| w.write_message(m))? },
            mod_EventModel::mod_Process::OneOfmessage::None => {},
    }        Ok(())
    }
}

pub mod mod_Process {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DropFiles { }

impl<'a> MessageRead<'a> for DropFiles {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for DropFiles { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Import { }

impl<'a> MessageRead<'a> for Import {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Import { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Export { }

impl<'a> MessageRead<'a> for Export {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Export { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SaveFile { }

impl<'a> MessageRead<'a> for SaveFile {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for SaveFile { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Migration { }

impl<'a> MessageRead<'a> for Migration {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Migration { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PreloadFile { }

impl<'a> MessageRead<'a> for PreloadFile {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for PreloadFile { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Progress<'a> {
    pub total: i64,
    pub done: i64,
    pub message: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Progress<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.total = r.read_int64(bytes)?,
                Ok(16) => msg.done = r.read_int64(bytes)?,
                Ok(26) => msg.message = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Progress<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.total == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.total) as u64) }
        + if self.done == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.done) as u64) }
        + if self.message == "" { 0 } else { 1 + sizeof_len((&self.message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.total != 0i64 { w.write_with_tag(8, |w| w.write_int64(*&self.total))?; }
        if self.done != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.done))?; }
        if self.message != "" { w.write_with_tag(26, |w| w.write_string(&**&self.message))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    None = 0,
    Running = 1,
    Done = 2,
    Canceled = 3,
    Error = 4,
}

impl Default for State {
    fn default() -> Self {
        State::None
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::None,
            1 => State::Running,
            2 => State::Done,
            3 => State::Canceled,
            4 => State::Error,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => State::None,
            "Running" => State::Running,
            "Done" => State::Done,
            "Canceled" => State::Canceled,
            "Error" => State::Error,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfmessage {
    dropFiles(mod_EventModel::mod_Process::DropFiles),
    import(mod_EventModel::mod_Process::Import),
    export(mod_EventModel::mod_Process::Export),
    saveFile(mod_EventModel::mod_Process::SaveFile),
    migration(mod_EventModel::mod_Process::Migration),
    preloadFile(mod_EventModel::mod_Process::PreloadFile),
    None,
}

impl Default for OneOfmessage {
    fn default() -> Self {
        OneOfmessage::None
    }
}

}

}

// end-of 'events.proto' file
