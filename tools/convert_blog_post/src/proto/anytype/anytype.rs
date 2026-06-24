// Automatically generated rust module for 'snapshot.proto' file

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

