// Automatically generated rust module for 'changes.proto' file

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

