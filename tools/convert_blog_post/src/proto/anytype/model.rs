// Automatically generated rust module for 'models.proto' file

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
use serde::Serialize;
use super::*;
use crate::proto::google;
use crate::proto::anytype;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SmartBlockType {
    AccountOld = 0,
    Page = 16,
    ProfilePage = 17,
    Home = 32,
    Archive = 48,
    Widget = 112,
    File = 256,
    Template = 288,
    BundledTemplate = 289,
    BundledRelation = 512,
    SubObject = 513,
    BundledObjectType = 514,
    AnytypeProfile = 515,
    Date = 516,
    Workspace = 518,
    STRelation = 521,
    STType = 528,
    STRelationOption = 529,
    SpaceView = 530,
    Identity = 532,
    Participant = 534,
    MissingObject = 519,
    FileObject = 533,
    NotificationObject = 535,
    DevicesObject = 536,
    ChatObject = 537,
    ChatDerivedObject = 544,
    AccountObject = 545,
}

impl Default for SmartBlockType {
    fn default() -> Self {
        SmartBlockType::AccountOld
    }
}

impl From<i32> for SmartBlockType {
    fn from(i: i32) -> Self {
        match i {
            0 => SmartBlockType::AccountOld,
            16 => SmartBlockType::Page,
            17 => SmartBlockType::ProfilePage,
            32 => SmartBlockType::Home,
            48 => SmartBlockType::Archive,
            112 => SmartBlockType::Widget,
            256 => SmartBlockType::File,
            288 => SmartBlockType::Template,
            289 => SmartBlockType::BundledTemplate,
            512 => SmartBlockType::BundledRelation,
            513 => SmartBlockType::SubObject,
            514 => SmartBlockType::BundledObjectType,
            515 => SmartBlockType::AnytypeProfile,
            516 => SmartBlockType::Date,
            518 => SmartBlockType::Workspace,
            521 => SmartBlockType::STRelation,
            528 => SmartBlockType::STType,
            529 => SmartBlockType::STRelationOption,
            530 => SmartBlockType::SpaceView,
            532 => SmartBlockType::Identity,
            534 => SmartBlockType::Participant,
            519 => SmartBlockType::MissingObject,
            533 => SmartBlockType::FileObject,
            535 => SmartBlockType::NotificationObject,
            536 => SmartBlockType::DevicesObject,
            537 => SmartBlockType::ChatObject,
            544 => SmartBlockType::ChatDerivedObject,
            545 => SmartBlockType::AccountObject,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SmartBlockType {
    fn from(s: &'a str) -> Self {
        match s {
            "AccountOld" => SmartBlockType::AccountOld,
            "Page" => SmartBlockType::Page,
            "ProfilePage" => SmartBlockType::ProfilePage,
            "Home" => SmartBlockType::Home,
            "Archive" => SmartBlockType::Archive,
            "Widget" => SmartBlockType::Widget,
            "File" => SmartBlockType::File,
            "Template" => SmartBlockType::Template,
            "BundledTemplate" => SmartBlockType::BundledTemplate,
            "BundledRelation" => SmartBlockType::BundledRelation,
            "SubObject" => SmartBlockType::SubObject,
            "BundledObjectType" => SmartBlockType::BundledObjectType,
            "AnytypeProfile" => SmartBlockType::AnytypeProfile,
            "Date" => SmartBlockType::Date,
            "Workspace" => SmartBlockType::Workspace,
            "STRelation" => SmartBlockType::STRelation,
            "STType" => SmartBlockType::STType,
            "STRelationOption" => SmartBlockType::STRelationOption,
            "SpaceView" => SmartBlockType::SpaceView,
            "Identity" => SmartBlockType::Identity,
            "Participant" => SmartBlockType::Participant,
            "MissingObject" => SmartBlockType::MissingObject,
            "FileObject" => SmartBlockType::FileObject,
            "NotificationObject" => SmartBlockType::NotificationObject,
            "DevicesObject" => SmartBlockType::DevicesObject,
            "ChatObject" => SmartBlockType::ChatObject,
            "ChatDerivedObject" => SmartBlockType::ChatDerivedObject,
            "AccountObject" => SmartBlockType::AccountObject,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RelationFormat {
    longtext = 0,
    shorttext = 1,
    number = 2,
    status = 3,
    tag = 11,
    date = 4,
    file = 5,
    checkbox = 6,
    url = 7,
    email = 8,
    phone = 9,
    emoji = 10,
    object = 100,
    relations = 101,
}

impl Default for RelationFormat {
    fn default() -> Self {
        RelationFormat::longtext
    }
}

impl From<i32> for RelationFormat {
    fn from(i: i32) -> Self {
        match i {
            0 => RelationFormat::longtext,
            1 => RelationFormat::shorttext,
            2 => RelationFormat::number,
            3 => RelationFormat::status,
            11 => RelationFormat::tag,
            4 => RelationFormat::date,
            5 => RelationFormat::file,
            6 => RelationFormat::checkbox,
            7 => RelationFormat::url,
            8 => RelationFormat::email,
            9 => RelationFormat::phone,
            10 => RelationFormat::emoji,
            100 => RelationFormat::object,
            101 => RelationFormat::relations,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RelationFormat {
    fn from(s: &'a str) -> Self {
        match s {
            "longtext" => RelationFormat::longtext,
            "shorttext" => RelationFormat::shorttext,
            "number" => RelationFormat::number,
            "status" => RelationFormat::status,
            "tag" => RelationFormat::tag,
            "date" => RelationFormat::date,
            "file" => RelationFormat::file,
            "checkbox" => RelationFormat::checkbox,
            "url" => RelationFormat::url,
            "email" => RelationFormat::email,
            "phone" => RelationFormat::phone,
            "emoji" => RelationFormat::emoji,
            "object" => RelationFormat::object,
            "relations" => RelationFormat::relations,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjectOrigin {
    none = 0,
    clipboard = 1,
    dragAndDrop = 2,
    import = 3,
    webclipper = 4,
    sharingExtension = 5,
    usecase = 6,
    builtin = 7,
    bookmark = 8,
    api = 9,
}

impl Default for ObjectOrigin {
    fn default() -> Self {
        ObjectOrigin::none
    }
}

impl From<i32> for ObjectOrigin {
    fn from(i: i32) -> Self {
        match i {
            0 => ObjectOrigin::none,
            1 => ObjectOrigin::clipboard,
            2 => ObjectOrigin::dragAndDrop,
            3 => ObjectOrigin::import,
            4 => ObjectOrigin::webclipper,
            5 => ObjectOrigin::sharingExtension,
            6 => ObjectOrigin::usecase,
            7 => ObjectOrigin::builtin,
            8 => ObjectOrigin::bookmark,
            9 => ObjectOrigin::api,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ObjectOrigin {
    fn from(s: &'a str) -> Self {
        match s {
            "none" => ObjectOrigin::none,
            "clipboard" => ObjectOrigin::clipboard,
            "dragAndDrop" => ObjectOrigin::dragAndDrop,
            "import" => ObjectOrigin::import,
            "webclipper" => ObjectOrigin::webclipper,
            "sharingExtension" => ObjectOrigin::sharingExtension,
            "usecase" => ObjectOrigin::usecase,
            "builtin" => ObjectOrigin::builtin,
            "bookmark" => ObjectOrigin::bookmark,
            "api" => ObjectOrigin::api,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpaceStatus {
    Unknown = 0,
    Loading = 1,
    Ok_pb = 2,
    Missing = 3,
    Error = 4,
    RemoteWaitingDeletion = 5,
    RemoteDeleted = 6,
    SpaceDeleted = 7,
    SpaceActive = 8,
    SpaceJoining = 9,
    SpaceRemoving = 10,
}

impl Default for SpaceStatus {
    fn default() -> Self {
        SpaceStatus::Unknown
    }
}

impl From<i32> for SpaceStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => SpaceStatus::Unknown,
            1 => SpaceStatus::Loading,
            2 => SpaceStatus::Ok_pb,
            3 => SpaceStatus::Missing,
            4 => SpaceStatus::Error,
            5 => SpaceStatus::RemoteWaitingDeletion,
            6 => SpaceStatus::RemoteDeleted,
            7 => SpaceStatus::SpaceDeleted,
            8 => SpaceStatus::SpaceActive,
            9 => SpaceStatus::SpaceJoining,
            10 => SpaceStatus::SpaceRemoving,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SpaceStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "Unknown" => SpaceStatus::Unknown,
            "Loading" => SpaceStatus::Loading,
            "Ok_pb" => SpaceStatus::Ok_pb,
            "Missing" => SpaceStatus::Missing,
            "Error" => SpaceStatus::Error,
            "RemoteWaitingDeletion" => SpaceStatus::RemoteWaitingDeletion,
            "RemoteDeleted" => SpaceStatus::RemoteDeleted,
            "SpaceDeleted" => SpaceStatus::SpaceDeleted,
            "SpaceActive" => SpaceStatus::SpaceActive,
            "SpaceJoining" => SpaceStatus::SpaceJoining,
            "SpaceRemoving" => SpaceStatus::SpaceRemoving,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParticipantPermissions {
    Reader = 0,
    Writer_pb = 1,
    Owner = 2,
    NoPermissions = 3,
}

impl Default for ParticipantPermissions {
    fn default() -> Self {
        ParticipantPermissions::Reader
    }
}

impl From<i32> for ParticipantPermissions {
    fn from(i: i32) -> Self {
        match i {
            0 => ParticipantPermissions::Reader,
            1 => ParticipantPermissions::Writer_pb,
            2 => ParticipantPermissions::Owner,
            3 => ParticipantPermissions::NoPermissions,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ParticipantPermissions {
    fn from(s: &'a str) -> Self {
        match s {
            "Reader" => ParticipantPermissions::Reader,
            "Writer_pb" => ParticipantPermissions::Writer_pb,
            "Owner" => ParticipantPermissions::Owner,
            "NoPermissions" => ParticipantPermissions::NoPermissions,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ParticipantStatus {
    Joining = 0,
    Active = 1,
    Removed = 2,
    Declined = 3,
    Removing = 4,
    Canceled = 5,
}

impl Default for ParticipantStatus {
    fn default() -> Self {
        ParticipantStatus::Joining
    }
}

impl From<i32> for ParticipantStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => ParticipantStatus::Joining,
            1 => ParticipantStatus::Active,
            2 => ParticipantStatus::Removed,
            3 => ParticipantStatus::Declined,
            4 => ParticipantStatus::Removing,
            5 => ParticipantStatus::Canceled,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ParticipantStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "Joining" => ParticipantStatus::Joining,
            "Active" => ParticipantStatus::Active,
            "Removed" => ParticipantStatus::Removed,
            "Declined" => ParticipantStatus::Declined,
            "Removing" => ParticipantStatus::Removing,
            "Canceled" => ParticipantStatus::Canceled,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpaceAccessType {
    Private = 0,
    Personal = 1,
    Shared = 2,
}

impl Default for SpaceAccessType {
    fn default() -> Self {
        SpaceAccessType::Private
    }
}

impl From<i32> for SpaceAccessType {
    fn from(i: i32) -> Self {
        match i {
            0 => SpaceAccessType::Private,
            1 => SpaceAccessType::Personal,
            2 => SpaceAccessType::Shared,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SpaceAccessType {
    fn from(s: &'a str) -> Self {
        match s {
            "Private" => SpaceAccessType::Private,
            "Personal" => SpaceAccessType::Personal,
            "Shared" => SpaceAccessType::Shared,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpaceUxType {
    Chat = 0,
    Data = 1,
    Stream = 2,
}

impl Default for SpaceUxType {
    fn default() -> Self {
        SpaceUxType::Chat
    }
}

impl From<i32> for SpaceUxType {
    fn from(i: i32) -> Self {
        match i {
            0 => SpaceUxType::Chat,
            1 => SpaceUxType::Data,
            2 => SpaceUxType::Stream,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SpaceUxType {
    fn from(s: &'a str) -> Self {
        match s {
            "Chat" => SpaceUxType::Chat,
            "Data" => SpaceUxType::Data,
            "Stream" => SpaceUxType::Stream,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImageKind {
    Basic = 0,
    Cover = 1,
    Icon = 2,
    AutomaticallyAdded = 3,
}

impl Default for ImageKind {
    fn default() -> Self {
        ImageKind::Basic
    }
}

impl From<i32> for ImageKind {
    fn from(i: i32) -> Self {
        match i {
            0 => ImageKind::Basic,
            1 => ImageKind::Cover,
            2 => ImageKind::Icon,
            3 => ImageKind::AutomaticallyAdded,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ImageKind {
    fn from(s: &'a str) -> Self {
        match s {
            "Basic" => ImageKind::Basic,
            "Cover" => ImageKind::Cover,
            "Icon" => ImageKind::Icon,
            "AutomaticallyAdded" => ImageKind::AutomaticallyAdded,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileIndexingStatus {
    NotIndexed = 0,
    Indexed = 1,
    NotFound = 2,
}

impl Default for FileIndexingStatus {
    fn default() -> Self {
        FileIndexingStatus::NotIndexed
    }
}

impl From<i32> for FileIndexingStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => FileIndexingStatus::NotIndexed,
            1 => FileIndexingStatus::Indexed,
            2 => FileIndexingStatus::NotFound,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for FileIndexingStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "NotIndexed" => FileIndexingStatus::NotIndexed,
            "Indexed" => FileIndexingStatus::Indexed,
            "NotFound" => FileIndexingStatus::NotFound,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpaceShareableStatus {
    StatusUnknown = 0,
    StatusShareable = 1,
    StatusNotShareable = 2,
}

impl Default for SpaceShareableStatus {
    fn default() -> Self {
        SpaceShareableStatus::StatusUnknown
    }
}

impl From<i32> for SpaceShareableStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => SpaceShareableStatus::StatusUnknown,
            1 => SpaceShareableStatus::StatusShareable,
            2 => SpaceShareableStatus::StatusNotShareable,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SpaceShareableStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "StatusUnknown" => SpaceShareableStatus::StatusUnknown,
            "StatusShareable" => SpaceShareableStatus::StatusShareable,
            "StatusNotShareable" => SpaceShareableStatus::StatusNotShareable,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NameserviceNameType {
    AnyName = 0,
}

impl Default for NameserviceNameType {
    fn default() -> Self {
        NameserviceNameType::AnyName
    }
}

impl From<i32> for NameserviceNameType {
    fn from(i: i32) -> Self {
        match i {
            0 => NameserviceNameType::AnyName,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for NameserviceNameType {
    fn from(s: &'a str) -> Self {
        match s {
            "AnyName" => NameserviceNameType::AnyName,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeviceNetworkType {
    WIFI = 0,
    CELLULAR = 1,
    NOT_CONNECTED = 2,
}

impl Default for DeviceNetworkType {
    fn default() -> Self {
        DeviceNetworkType::WIFI
    }
}

impl From<i32> for DeviceNetworkType {
    fn from(i: i32) -> Self {
        match i {
            0 => DeviceNetworkType::WIFI,
            1 => DeviceNetworkType::CELLULAR,
            2 => DeviceNetworkType::NOT_CONNECTED,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DeviceNetworkType {
    fn from(s: &'a str) -> Self {
        match s {
            "WIFI" => DeviceNetworkType::WIFI,
            "CELLULAR" => DeviceNetworkType::CELLULAR,
            "NOT_CONNECTED" => DeviceNetworkType::NOT_CONNECTED,
            _ => Self::default(),
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SmartBlockSnapshotBase<'a> {
    pub blocks: Vec<model::Block<'a>>,
    pub details: Option<google::protobuf::Struct<'a>>,
    pub objectTypes: Vec<Cow<'a, str>>,
    pub collections: Option<google::protobuf::Struct<'a>>,
    pub removedCollectionKeys: Vec<Cow<'a, str>>,
    pub relationLinks: Vec<model::RelationLink<'a>>,
    pub key: Cow<'a, str>,
    pub originalCreatedTimestamp: i64,
    pub fileInfo: Option<model::FileInfo<'a>>,
}

impl<'a> MessageRead<'a> for SmartBlockSnapshotBase<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.blocks.push(r.read_message::<model::Block>(bytes)?),
                Ok(18) => msg.details = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(42) => msg.objectTypes.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.collections = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(66) => msg.removedCollectionKeys.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(58) => msg.relationLinks.push(r.read_message::<model::RelationLink>(bytes)?),
                Ok(74) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.originalCreatedTimestamp = r.read_int64(bytes)?,
                Ok(90) => msg.fileInfo = Some(r.read_message::<model::FileInfo>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SmartBlockSnapshotBase<'a> {
    fn get_size(&self) -> usize {
        0
        + self.blocks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.details.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.objectTypes.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.collections.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.removedCollectionKeys.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.originalCreatedTimestamp == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.originalCreatedTimestamp) as u64) }
        + self.fileInfo.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.blocks { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.details { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.objectTypes { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.collections { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.removedCollectionKeys { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        for s in &self.relationLinks { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.key != "" { w.write_with_tag(74, |w| w.write_string(&**&self.key))?; }
        if self.originalCreatedTimestamp != 0i64 { w.write_with_tag(80, |w| w.write_int64(*&self.originalCreatedTimestamp))?; }
        if let Some(ref s) = self.fileInfo { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Search {
}

impl<'a> MessageRead<'a> for Search {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Search {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Search {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Result_pb<'a> {
    pub objectId: Cow<'a, str>,
    pub details: Option<google::protobuf::Struct<'a>>,
    pub meta: Vec<model::mod_Search::Meta<'a>>,
}

impl<'a> MessageRead<'a> for Result_pb<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.objectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.details = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(26) => msg.meta.push(r.read_message::<model::mod_Search::Meta>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Result_pb<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.objectId == "" { 0 } else { 1 + sizeof_len((&self.objectId).len()) }
        + self.details.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.meta.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.objectId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.objectId))?; }
        if let Some(ref s) = self.details { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.meta { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Meta<'a> {
    pub highlight: Cow<'a, str>,
    pub highlightRanges: Vec<model::Range>,
    pub blockId: Cow<'a, str>,
    pub relationKey: Cow<'a, str>,
    pub relationDetails: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Meta<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.highlight = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.highlightRanges.push(r.read_message::<model::Range>(bytes)?),
                Ok(26) => msg.blockId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.relationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.relationDetails = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Meta<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.highlight == "" { 0 } else { 1 + sizeof_len((&self.highlight).len()) }
        + self.highlightRanges.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.blockId == "" { 0 } else { 1 + sizeof_len((&self.blockId).len()) }
        + if self.relationKey == "" { 0 } else { 1 + sizeof_len((&self.relationKey).len()) }
        + self.relationDetails.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.highlight != "" { w.write_with_tag(10, |w| w.write_string(&**&self.highlight))?; }
        for s in &self.highlightRanges { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.blockId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.blockId))?; }
        if self.relationKey != "" { w.write_with_tag(34, |w| w.write_string(&**&self.relationKey))?; }
        if let Some(ref s) = self.relationDetails { w.write_with_tag(42, |w| w.write_message(s))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone )]
pub struct Block<'a> {
    pub id: Cow<'a, str>,
    pub fields: Option<google::protobuf::Struct<'a>>,
    pub restrictions: Option<model::mod_Block::Restrictions>,
    pub childrenIds: Vec<Cow<'a, str>>,
    pub backgroundColor: Cow<'a, str>,
    pub align: model::mod_Block::Align,
    pub verticalAlign: model::mod_Block::VerticalAlign,
    pub content: model::mod_Block::OneOfcontent<'a>,
}

impl<'a> MessageRead<'a> for Block<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.fields = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(26) => msg.restrictions = Some(r.read_message::<model::mod_Block::Restrictions>(bytes)?),
                Ok(34) => msg.childrenIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.backgroundColor = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.align = r.read_enum(bytes)?,
                Ok(56) => msg.verticalAlign = r.read_enum(bytes)?,
                Ok(90) => msg.content = model::mod_Block::OneOfcontent::smartblock(r.read_message::<model::mod_Block::mod_Content::Smartblock>(bytes)?),
                Ok(114) => msg.content = model::mod_Block::OneOfcontent::text(r.read_message::<model::mod_Block::mod_Content::Text>(bytes)?),
                Ok(122) => msg.content = model::mod_Block::OneOfcontent::file(r.read_message::<model::mod_Block::mod_Content::File>(bytes)?),
                Ok(130) => msg.content = model::mod_Block::OneOfcontent::layout(r.read_message::<model::mod_Block::mod_Content::Layout>(bytes)?),
                Ok(138) => msg.content = model::mod_Block::OneOfcontent::div(r.read_message::<model::mod_Block::mod_Content::Div>(bytes)?),
                Ok(146) => msg.content = model::mod_Block::OneOfcontent::bookmark(r.read_message::<model::mod_Block::mod_Content::Bookmark>(bytes)?),
                Ok(154) => msg.content = model::mod_Block::OneOfcontent::icon(r.read_message::<model::mod_Block::mod_Content::Icon>(bytes)?),
                Ok(162) => msg.content = model::mod_Block::OneOfcontent::link(r.read_message::<model::mod_Block::mod_Content::Link>(bytes)?),
                Ok(170) => msg.content = model::mod_Block::OneOfcontent::dataview(r.read_message::<model::mod_Block::mod_Content::Dataview>(bytes)?),
                Ok(178) => msg.content = model::mod_Block::OneOfcontent::relation(r.read_message::<model::mod_Block::mod_Content::Relation>(bytes)?),
                Ok(186) => msg.content = model::mod_Block::OneOfcontent::featuredRelations(r.read_message::<model::mod_Block::mod_Content::FeaturedRelations>(bytes)?),
                Ok(194) => msg.content = model::mod_Block::OneOfcontent::latex(r.read_message::<model::mod_Block::mod_Content::Latex>(bytes)?),
                Ok(202) => msg.content = model::mod_Block::OneOfcontent::tableOfContents(r.read_message::<model::mod_Block::mod_Content::TableOfContents>(bytes)?),
                Ok(210) => msg.content = model::mod_Block::OneOfcontent::table(r.read_message::<model::mod_Block::mod_Content::Table>(bytes)?),
                Ok(218) => msg.content = model::mod_Block::OneOfcontent::tableColumn(r.read_message::<model::mod_Block::mod_Content::TableColumn>(bytes)?),
                Ok(226) => msg.content = model::mod_Block::OneOfcontent::tableRow(r.read_message::<model::mod_Block::mod_Content::TableRow>(bytes)?),
                Ok(234) => msg.content = model::mod_Block::OneOfcontent::widget(r.read_message::<model::mod_Block::mod_Content::Widget>(bytes)?),
                Ok(242) => msg.content = model::mod_Block::OneOfcontent::chat(r.read_message::<model::mod_Block::mod_Content::Chat>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Block<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.restrictions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.childrenIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.backgroundColor == "" { 0 } else { 1 + sizeof_len((&self.backgroundColor).len()) }
        + if self.align == anytype::model::mod_Block::Align::AlignLeft { 0 } else { 1 + sizeof_varint(*(&self.align) as u64) }
        + if self.verticalAlign == anytype::model::mod_Block::VerticalAlign::VerticalAlignTop { 0 } else { 1 + sizeof_varint(*(&self.verticalAlign) as u64) }
        + match self.content {
            model::mod_Block::OneOfcontent::smartblock(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::text(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::file(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::layout(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::div(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::bookmark(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::icon(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::link(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::dataview(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::relation(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::featuredRelations(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::latex(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::tableOfContents(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::table(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::tableColumn(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::tableRow(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::widget(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::chat(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Block::OneOfcontent::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.restrictions { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.childrenIds { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if self.backgroundColor != "" { w.write_with_tag(42, |w| w.write_string(&**&self.backgroundColor))?; }
        if self.align != anytype::model::mod_Block::Align::AlignLeft { w.write_with_tag(48, |w| w.write_enum(*&self.align as i32))?; }
        if self.verticalAlign != anytype::model::mod_Block::VerticalAlign::VerticalAlignTop { w.write_with_tag(56, |w| w.write_enum(*&self.verticalAlign as i32))?; }
        match self.content {            model::mod_Block::OneOfcontent::smartblock(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::text(ref m) => { w.write_with_tag(114, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::file(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::layout(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::div(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::bookmark(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::icon(ref m) => { w.write_with_tag(154, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::link(ref m) => { w.write_with_tag(162, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::dataview(ref m) => { w.write_with_tag(170, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::relation(ref m) => { w.write_with_tag(178, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::featuredRelations(ref m) => { w.write_with_tag(186, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::latex(ref m) => { w.write_with_tag(194, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::tableOfContents(ref m) => { w.write_with_tag(202, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::table(ref m) => { w.write_with_tag(210, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::tableColumn(ref m) => { w.write_with_tag(218, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::tableRow(ref m) => { w.write_with_tag(226, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::widget(ref m) => { w.write_with_tag(234, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::chat(ref m) => { w.write_with_tag(242, |w| w.write_message(m))? },
            model::mod_Block::OneOfcontent::None => {},
    }        Ok(())
    }
}

pub mod mod_Block {

use serde::{Deserialize, Serialize};

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Restrictions {
    pub read: bool,
    pub edit: bool,
    pub remove: bool,
    pub drag: bool,
    pub dropOn: bool,
}

impl<'a> MessageRead<'a> for Restrictions {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.read = r.read_bool(bytes)?,
                Ok(16) => msg.edit = r.read_bool(bytes)?,
                Ok(24) => msg.remove = r.read_bool(bytes)?,
                Ok(32) => msg.drag = r.read_bool(bytes)?,
                Ok(40) => msg.dropOn = r.read_bool(bytes)?,
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
        + if self.read == false { 0 } else { 1 + sizeof_varint(*(&self.read) as u64) }
        + if self.edit == false { 0 } else { 1 + sizeof_varint(*(&self.edit) as u64) }
        + if self.remove == false { 0 } else { 1 + sizeof_varint(*(&self.remove) as u64) }
        + if self.drag == false { 0 } else { 1 + sizeof_varint(*(&self.drag) as u64) }
        + if self.dropOn == false { 0 } else { 1 + sizeof_varint(*(&self.dropOn) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.read != false { w.write_with_tag(8, |w| w.write_bool(*&self.read))?; }
        if self.edit != false { w.write_with_tag(16, |w| w.write_bool(*&self.edit))?; }
        if self.remove != false { w.write_with_tag(24, |w| w.write_bool(*&self.remove))?; }
        if self.drag != false { w.write_with_tag(32, |w| w.write_bool(*&self.drag))?; }
        if self.dropOn != false { w.write_with_tag(40, |w| w.write_bool(*&self.dropOn))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Content {
}

impl<'a> MessageRead<'a> for Content {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Content {
    fn get_size(&self) -> usize {
        0
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        Ok(())
    }
}

pub mod mod_Content {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Layout {
    pub style: model::mod_Block::mod_Content::mod_Layout::Style,
}

impl<'a> MessageRead<'a> for Layout {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.style = r.read_enum(bytes)?,
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
        + if self.style == anytype::model::mod_Block::mod_Content::mod_Layout::Style::Row { 0 } else { 1 + sizeof_varint(*(&self.style) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.style != anytype::model::mod_Block::mod_Content::mod_Layout::Style::Row { w.write_with_tag(8, |w| w.write_enum(*&self.style as i32))?; }
        Ok(())
    }
}

pub mod mod_Layout {
    use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Style {
    Row = 0,
    Column = 1,
    Div = 2,
    Header = 3,
    TableRows = 4,
    TableColumns = 5,
}

impl Default for Style {
    fn default() -> Self {
        Style::Row
    }
}

impl From<i32> for Style {
    fn from(i: i32) -> Self {
        match i {
            0 => Style::Row,
            1 => Style::Column,
            2 => Style::Div,
            3 => Style::Header,
            4 => Style::TableRows,
            5 => Style::TableColumns,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
        match s {
            "Row" => Style::Row,
            "Column" => Style::Column,
            "Div" => Style::Div,
            "Header" => Style::Header,
            "TableRows" => Style::TableRows,
            "TableColumns" => Style::TableColumns,
            _ => Self::default(),
        }
    }
}
impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
             Style::Row => "Row",
             Style::Column => "Column",
             Style::Div => "Div",
             Style::Header => "Header",
             Style::TableRows => "TableRows",
             Style::TableColumns => "TableColumns",
        }.to_string()
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Link<'a> {
    pub targetBlockId: Cow<'a, str>,
    pub style: model::mod_Block::mod_Content::mod_Link::Style,
    pub fields: Option<google::protobuf::Struct<'a>>,
    pub iconSize: model::mod_Block::mod_Content::mod_Link::IconSize,
    pub cardStyle: model::mod_Block::mod_Content::mod_Link::CardStyle,
    pub description: model::mod_Block::mod_Content::mod_Link::Description,
    pub relations: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Link<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.targetBlockId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.style = r.read_enum(bytes)?,
                Ok(26) => msg.fields = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(32) => msg.iconSize = r.read_enum(bytes)?,
                Ok(40) => msg.cardStyle = r.read_enum(bytes)?,
                Ok(48) => msg.description = r.read_enum(bytes)?,
                Ok(58) => msg.relations.push(r.read_string(bytes).map(Cow::Borrowed)?),
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
        + if self.targetBlockId == "" { 0 } else { 1 + sizeof_len((&self.targetBlockId).len()) }
        + if self.style == anytype::model::mod_Block::mod_Content::mod_Link::Style::Page { 0 } else { 1 + sizeof_varint(*(&self.style) as u64) }
        + self.fields.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.iconSize == anytype::model::mod_Block::mod_Content::mod_Link::IconSize::SizeNone { 0 } else { 1 + sizeof_varint(*(&self.iconSize) as u64) }
        + if self.cardStyle == anytype::model::mod_Block::mod_Content::mod_Link::CardStyle::Text { 0 } else { 1 + sizeof_varint(*(&self.cardStyle) as u64) }
        + if self.description == anytype::model::mod_Block::mod_Content::mod_Link::Description::None { 0 } else { 1 + sizeof_varint(*(&self.description) as u64) }
        + self.relations.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.targetBlockId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.targetBlockId))?; }
        if self.style != anytype::model::mod_Block::mod_Content::mod_Link::Style::Page { w.write_with_tag(16, |w| w.write_enum(*&self.style as i32))?; }
        if let Some(ref s) = self.fields { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.iconSize != anytype::model::mod_Block::mod_Content::mod_Link::IconSize::SizeNone { w.write_with_tag(32, |w| w.write_enum(*&self.iconSize as i32))?; }
        if self.cardStyle != anytype::model::mod_Block::mod_Content::mod_Link::CardStyle::Text { w.write_with_tag(40, |w| w.write_enum(*&self.cardStyle as i32))?; }
        if self.description != anytype::model::mod_Block::mod_Content::mod_Link::Description::None { w.write_with_tag(48, |w| w.write_enum(*&self.description as i32))?; }
        for s in &self.relations { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

pub mod mod_Link {
    use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum IconSize {
    SizeNone = 0,
    SizeSmall = 1,
    SizeMedium = 2,
}

impl Default for IconSize {
    fn default() -> Self {
        IconSize::SizeNone
    }
}

impl From<i32> for IconSize {
    fn from(i: i32) -> Self {
        match i {
            0 => IconSize::SizeNone,
            1 => IconSize::SizeSmall,
            2 => IconSize::SizeMedium,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for IconSize {
    fn from(s: &'a str) -> Self {
        match s {
            "SizeNone" => IconSize::SizeNone,
            "SizeSmall" => IconSize::SizeSmall,
            "SizeMedium" => IconSize::SizeMedium,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Style {
    Page = 0,
    Dataview = 1,
    Dashboard = 2,
    Archive = 3,
}

impl Default for Style {
    fn default() -> Self {
        Style::Page
    }
}

impl From<i32> for Style {
    fn from(i: i32) -> Self {
        match i {
            0 => Style::Page,
            1 => Style::Dataview,
            2 => Style::Dashboard,
            3 => Style::Archive,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
        match s {
            "Page" => Style::Page,
            "Dataview" => Style::Dataview,
            "Dashboard" => Style::Dashboard,
            "Archive" => Style::Archive,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Description {
    None = 0,
    Added = 1,
    Content = 2,
}

impl Default for Description {
    fn default() -> Self {
        Description::None
    }
}

impl From<i32> for Description {
    fn from(i: i32) -> Self {
        match i {
            0 => Description::None,
            1 => Description::Added,
            2 => Description::Content,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Description {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => Description::None,
            "Added" => Description::Added,
            "Content" => Description::Content,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum CardStyle {
    Text = 0,
    Card = 1,
    Inline = 2,
}

impl Default for CardStyle {
    fn default() -> Self {
        CardStyle::Text
    }
}

impl From<i32> for CardStyle {
    fn from(i: i32) -> Self {
        match i {
            0 => CardStyle::Text,
            1 => CardStyle::Card,
            2 => CardStyle::Inline,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for CardStyle {
    fn from(s: &'a str) -> Self {
        match s {
            "Text" => CardStyle::Text,
            "Card" => CardStyle::Card,
            "Inline" => CardStyle::Inline,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Div {
    pub style: model::mod_Block::mod_Content::mod_Div::Style,
}

impl<'a> MessageRead<'a> for Div {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.style = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Div {
    fn get_size(&self) -> usize {
        0
        + if self.style == anytype::model::mod_Block::mod_Content::mod_Div::Style::Line { 0 } else { 1 + sizeof_varint(*(&self.style) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.style != anytype::model::mod_Block::mod_Content::mod_Div::Style::Line { w.write_with_tag(8, |w| w.write_enum(*&self.style as i32))?; }
        Ok(())
    }
}

pub mod mod_Div {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Style {
    Line = 0,
    Dots = 1,
}

impl Default for Style {
    fn default() -> Self {
        Style::Line
    }
}

impl From<i32> for Style {
    fn from(i: i32) -> Self {
        match i {
            0 => Style::Line,
            1 => Style::Dots,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
        match s {
            "Line" => Style::Line,
            "Dots" => Style::Dots,
            _ => Self::default(),
        }
    }
}
impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
            Style::Line =>"Line" ,
            Style::Dots =>"Dots" ,
        }.to_string()
    }
}
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Bookmark<'a> {
    pub url: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub imageHash: Cow<'a, str>,
    pub faviconHash: Cow<'a, str>,
    pub type_pb: model::mod_LinkPreview::Type,
    pub targetObjectId: Cow<'a, str>,
    pub state: model::mod_Block::mod_Content::mod_Bookmark::State,
}

impl<'a> MessageRead<'a> for Bookmark<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.imageHash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.faviconHash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.type_pb = r.read_enum(bytes)?,
                Ok(58) => msg.targetObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.state = r.read_enum(bytes)?,
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
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.imageHash == "" { 0 } else { 1 + sizeof_len((&self.imageHash).len()) }
        + if self.faviconHash == "" { 0 } else { 1 + sizeof_len((&self.faviconHash).len()) }
        + if self.type_pb == anytype::model::mod_LinkPreview::Type::Unknown { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.targetObjectId == "" { 0 } else { 1 + sizeof_len((&self.targetObjectId).len()) }
        + if self.state == anytype::model::mod_Block::mod_Content::mod_Bookmark::State::Empty { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.url != "" { w.write_with_tag(10, |w| w.write_string(&**&self.url))?; }
        if self.title != "" { w.write_with_tag(18, |w| w.write_string(&**&self.title))?; }
        if self.description != "" { w.write_with_tag(26, |w| w.write_string(&**&self.description))?; }
        if self.imageHash != "" { w.write_with_tag(34, |w| w.write_string(&**&self.imageHash))?; }
        if self.faviconHash != "" { w.write_with_tag(42, |w| w.write_string(&**&self.faviconHash))?; }
        if self.type_pb != anytype::model::mod_LinkPreview::Type::Unknown { w.write_with_tag(48, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.targetObjectId != "" { w.write_with_tag(58, |w| w.write_string(&**&self.targetObjectId))?; }
        if self.state != anytype::model::mod_Block::mod_Content::mod_Bookmark::State::Empty { w.write_with_tag(64, |w| w.write_enum(*&self.state as i32))?; }
        Ok(())
    }
}

pub mod mod_Bookmark {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Empty = 0,
    Fetching = 1,
    Done = 2,
    Error = 3,
}

impl Default for State {
    fn default() -> Self {
        State::Empty
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::Empty,
            1 => State::Fetching,
            2 => State::Done,
            3 => State::Error,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "Empty" => State::Empty,
            "Fetching" => State::Fetching,
            "Done" => State::Done,
            "Error" => State::Error,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Icon<'a> {
    pub name: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Icon<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Icon<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FeaturedRelations { }

impl<'a> MessageRead<'a> for FeaturedRelations {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for FeaturedRelations { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Text<'a> {
    pub text: Cow<'a, str>,
    pub style: model::mod_Block::mod_Content::mod_Text::Style,
    pub marks: Option<model::mod_Block::mod_Content::mod_Text::Marks<'a>>,
    pub checked: bool,
    pub color: Cow<'a, str>,
    pub iconEmoji: Cow<'a, str>,
    pub iconImage: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Text<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.style = r.read_enum(bytes)?,
                Ok(26) => msg.marks = Some(r.read_message::<model::mod_Block::mod_Content::mod_Text::Marks>(bytes)?),
                Ok(32) => msg.checked = r.read_bool(bytes)?,
                Ok(42) => msg.color = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.iconEmoji = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.iconImage = r.read_string(bytes).map(Cow::Borrowed)?,
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
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.style == anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { 0 } else { 1 + sizeof_varint(*(&self.style) as u64) }
        + self.marks.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.checked == false { 0 } else { 1 + sizeof_varint(*(&self.checked) as u64) }
        + if self.color == "" { 0 } else { 1 + sizeof_len((&self.color).len()) }
        + if self.iconEmoji == "" { 0 } else { 1 + sizeof_len((&self.iconEmoji).len()) }
        + if self.iconImage == "" { 0 } else { 1 + sizeof_len((&self.iconImage).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != "" { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        if self.style != anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { w.write_with_tag(16, |w| w.write_enum(*&self.style as i32))?; }
        if let Some(ref s) = self.marks { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.checked != false { w.write_with_tag(32, |w| w.write_bool(*&self.checked))?; }
        if self.color != "" { w.write_with_tag(42, |w| w.write_string(&**&self.color))?; }
        if self.iconEmoji != "" { w.write_with_tag(50, |w| w.write_string(&**&self.iconEmoji))?; }
        if self.iconImage != "" { w.write_with_tag(58, |w| w.write_string(&**&self.iconImage))?; }
        Ok(())
    }
}

pub mod mod_Text {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Marks<'a> {
    pub marks: Vec<model::mod_Block::mod_Content::mod_Text::Mark<'a>>,
}

impl<'a> MessageRead<'a> for Marks<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.marks.push(r.read_message::<model::mod_Block::mod_Content::mod_Text::Mark>(bytes)?),
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
        + self.marks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.marks { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Mark<'a> {
    pub range: Option<model::Range>,
    pub type_pb: model::mod_Block::mod_Content::mod_Text::mod_Mark::Type,
    pub param: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Mark<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.range = Some(r.read_message::<model::Range>(bytes)?),
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.param = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Mark<'a> {
    fn get_size(&self) -> usize {
        0
        + self.range.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.type_pb == anytype::model::mod_Block::mod_Content::mod_Text::mod_Mark::Type::Strikethrough { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.param == "" { 0 } else { 1 + sizeof_len((&self.param).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.range { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.type_pb != anytype::model::mod_Block::mod_Content::mod_Text::mod_Mark::Type::Strikethrough { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.param != "" { w.write_with_tag(26, |w| w.write_string(&**&self.param))?; }
        Ok(())
    }
}

pub mod mod_Mark {
    use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Strikethrough = 0,
    Keyboard = 1,
    Italic = 2,
    Bold = 3,
    Underscored = 4,
    Link = 5,
    TextColor = 6,
    BackgroundColor = 7,
    Mention = 8,
    Emoji = 9,
    Object = 10,
}

impl Default for Type {
    fn default() -> Self {
        Type::Strikethrough
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::Strikethrough,
            1 => Type::Keyboard,
            2 => Type::Italic,
            3 => Type::Bold,
            4 => Type::Underscored,
            5 => Type::Link,
            6 => Type::TextColor,
            7 => Type::BackgroundColor,
            8 => Type::Mention,
            9 => Type::Emoji,
            10 => Type::Object,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Strikethrough" => Type::Strikethrough,
            "Keyboard" => Type::Keyboard,
            "Italic" => Type::Italic,
            "Bold" => Type::Bold,
            "Underscored" => Type::Underscored,
            "Link" => Type::Link,
            "TextColor" => Type::TextColor,
            "BackgroundColor" => Type::BackgroundColor,
            "Mention" => Type::Mention,
            "Emoji" => Type::Emoji,
            "Object" => Type::Object,
            _ => Self::default(),
        }
    }
}


#[test]
fn test_to_serde_string(){
    let token_enum = Type::Link;
    println!("{:?}", serde_json::to_string(&token_enum));
}



}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Style {
    Paragraph = 0,
    Header1 = 1,
    Header2 = 2,
    Header3 = 3,
    Header4 = 4,
    Quote = 5,
    Code = 6,
    Title = 7,
    Checkbox = 8,
    Marked = 9,
    Numbered = 10,
    Toggle = 11,
    Description = 12,
    Callout = 13,
}

impl Default for Style {
    fn default() -> Self {
        Style::Paragraph
    }
}

impl From<i32> for Style {
    fn from(i: i32) -> Self {
        match i {
            0 => Style::Paragraph,
            1 => Style::Header1,
            2 => Style::Header2,
            3 => Style::Header3,
            4 => Style::Header4,
            5 => Style::Quote,
            6 => Style::Code,
            7 => Style::Title,
            8 => Style::Checkbox,
            9 => Style::Marked,
            10 => Style::Numbered,
            11 => Style::Toggle,
            12 => Style::Description,
            13 => Style::Callout,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
        match s {
            "Paragraph" => Style::Paragraph,
            "Header1" => Style::Header1,
            "Header2" => Style::Header2,
            "Header3" => Style::Header3,
            "Header4" => Style::Header4,
            "Quote" => Style::Quote,
            "Code" => Style::Code,
            "Title" => Style::Title,
            "Checkbox" => Style::Checkbox,
            "Marked" => Style::Marked,
            "Numbered" => Style::Numbered,
            "Toggle" => Style::Toggle,
            "Description" => Style::Description,
            "Callout" => Style::Callout,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct File<'a> {
    pub hash: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub type_pb: model::mod_Block::mod_Content::mod_File::Type,
    pub mime: Cow<'a, str>,
    pub size: i64,
    pub addedAt: i64,
    pub targetObjectId: Cow<'a, str>,
    pub state: model::mod_Block::mod_Content::mod_File::State,
    pub style: model::mod_Block::mod_Content::mod_File::Style,
}

impl<'a> MessageRead<'a> for File<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.hash = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.type_pb = r.read_enum(bytes)?,
                Ok(34) => msg.mime = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(40) => msg.size = r.read_int64(bytes)?,
                Ok(48) => msg.addedAt = r.read_int64(bytes)?,
                Ok(74) => msg.targetObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.state = r.read_enum(bytes)?,
                Ok(64) => msg.style = r.read_enum(bytes)?,
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
        + if self.hash == "" { 0 } else { 1 + sizeof_len((&self.hash).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.type_pb == anytype::model::mod_Block::mod_Content::mod_File::Type::None { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.mime == "" { 0 } else { 1 + sizeof_len((&self.mime).len()) }
        + if self.size == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.size) as u64) }
        + if self.addedAt == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.addedAt) as u64) }
        + if self.targetObjectId == "" { 0 } else { 1 + sizeof_len((&self.targetObjectId).len()) }
        + if self.state == anytype::model::mod_Block::mod_Content::mod_File::State::Empty { 0 } else { 1 + sizeof_varint(*(&self.state) as u64) }
        + if self.style == anytype::model::mod_Block::mod_Content::mod_File::Style::Auto { 0 } else { 1 + sizeof_varint(*(&self.style) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hash != "" { w.write_with_tag(10, |w| w.write_string(&**&self.hash))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.type_pb != anytype::model::mod_Block::mod_Content::mod_File::Type::None { w.write_with_tag(24, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.mime != "" { w.write_with_tag(34, |w| w.write_string(&**&self.mime))?; }
        if self.size != 0i64 { w.write_with_tag(40, |w| w.write_int64(*&self.size))?; }
        if self.addedAt != 0i64 { w.write_with_tag(48, |w| w.write_int64(*&self.addedAt))?; }
        if self.targetObjectId != "" { w.write_with_tag(74, |w| w.write_string(&**&self.targetObjectId))?; }
        if self.state != anytype::model::mod_Block::mod_Content::mod_File::State::Empty { w.write_with_tag(56, |w| w.write_enum(*&self.state as i32))?; }
        if self.style != anytype::model::mod_Block::mod_Content::mod_File::Style::Auto { w.write_with_tag(64, |w| w.write_enum(*&self.style as i32))?; }
        Ok(())
    }
}

pub mod mod_File {
    use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    None = 0,
    File = 1,
    Image = 2,
    Video = 3,
    Audio = 4,
    PDF = 5,
}

impl Default for Type {
    fn default() -> Self {
        Type::None
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::None,
            1 => Type::File,
            2 => Type::Image,
            3 => Type::Video,
            4 => Type::Audio,
            5 => Type::PDF,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => Type::None,
            "File" => Type::File,
            "Image" => Type::Image,
            "Video" => Type::Video,
            "Audio" => Type::Audio,
            "PDF" => Type::PDF,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Style {
    Auto = 0,
    Link = 1,
    Embed = 2,
}

impl Default for Style {
    fn default() -> Self {
        Style::Auto
    }
}

impl From<i32> for Style {
    fn from(i: i32) -> Self {
        match i {
            0 => Style::Auto,
            1 => Style::Link,
            2 => Style::Embed,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Style {
    fn from(s: &'a str) -> Self {
        match s {
            "Auto" => Style::Auto,
            "Link" => Style::Link,
            "Embed" => Style::Embed,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Empty = 0,
    Uploading = 1,
    Done = 2,
    Error = 3,
}

impl Default for State {
    fn default() -> Self {
        State::Empty
    }
}

impl From<i32> for State {
    fn from(i: i32) -> Self {
        match i {
            0 => State::Empty,
            1 => State::Uploading,
            2 => State::Done,
            3 => State::Error,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> Self {
        match s {
            "Empty" => State::Empty,
            "Uploading" => State::Uploading,
            "Done" => State::Done,
            "Error" => State::Error,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Smartblock { }

impl<'a> MessageRead<'a> for Smartblock {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Smartblock { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Dataview<'a> {
    pub source: Vec<Cow<'a, str>>,
    pub views: Vec<model::mod_Block::mod_Content::mod_Dataview::View<'a>>,
    pub activeView: Cow<'a, str>,
    pub relations: Vec<model::Relation<'a>>,
    pub groupOrders: Vec<model::mod_Block::mod_Content::mod_Dataview::GroupOrder<'a>>,
    pub objectOrders: Vec<model::mod_Block::mod_Content::mod_Dataview::ObjectOrder<'a>>,
    pub relationLinks: Vec<model::RelationLink<'a>>,
    pub TargetObjectId: Cow<'a, str>,
    pub isCollection: bool,
}

impl<'a> MessageRead<'a> for Dataview<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.source.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.views.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::View>(bytes)?),
                Ok(26) => msg.activeView = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.relations.push(r.read_message::<model::Relation>(bytes)?),
                Ok(98) => msg.groupOrders.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::GroupOrder>(bytes)?),
                Ok(106) => msg.objectOrders.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::ObjectOrder>(bytes)?),
                Ok(42) => msg.relationLinks.push(r.read_message::<model::RelationLink>(bytes)?),
                Ok(50) => msg.TargetObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(112) => msg.isCollection = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Dataview<'a> {
    fn get_size(&self) -> usize {
        0
        + self.source.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.views.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.activeView == "" { 0 } else { 1 + sizeof_len((&self.activeView).len()) }
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.groupOrders.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.objectOrders.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.TargetObjectId == "" { 0 } else { 1 + sizeof_len((&self.TargetObjectId).len()) }
        + if self.isCollection == false { 0 } else { 1 + sizeof_varint(*(&self.isCollection) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.source { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.views { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.activeView != "" { w.write_with_tag(26, |w| w.write_string(&**&self.activeView))?; }
        for s in &self.relations { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.groupOrders { w.write_with_tag(98, |w| w.write_message(s))?; }
        for s in &self.objectOrders { w.write_with_tag(106, |w| w.write_message(s))?; }
        for s in &self.relationLinks { w.write_with_tag(42, |w| w.write_message(s))?; }
        if self.TargetObjectId != "" { w.write_with_tag(50, |w| w.write_string(&**&self.TargetObjectId))?; }
        if self.isCollection != false { w.write_with_tag(112, |w| w.write_bool(*&self.isCollection))?; }
        Ok(())
    }
}

pub mod mod_Dataview {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct View<'a> {
    pub id: Cow<'a, str>,
    pub type_pb: model::mod_Block::mod_Content::mod_Dataview::mod_View::Type,
    pub name: Cow<'a, str>,
    pub sorts: Vec<model::mod_Block::mod_Content::mod_Dataview::Sort<'a>>,
    pub filters: Vec<model::mod_Block::mod_Content::mod_Dataview::Filter<'a>>,
    pub relations: Vec<model::mod_Block::mod_Content::mod_Dataview::Relation<'a>>,
    pub coverRelationKey: Cow<'a, str>,
    pub hideIcon: bool,
    pub cardSize: model::mod_Block::mod_Content::mod_Dataview::mod_View::Size,
    pub coverFit: bool,
    pub groupRelationKey: Cow<'a, str>,
    pub groupBackgroundColors: bool,
    pub pageLimit: i32,
    pub defaultTemplateId: Cow<'a, str>,
    pub defaultObjectTypeId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for View<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.sorts.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Sort>(bytes)?),
                Ok(42) => msg.filters.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Filter>(bytes)?),
                Ok(50) => msg.relations.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Relation>(bytes)?),
                Ok(58) => msg.coverRelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.hideIcon = r.read_bool(bytes)?,
                Ok(72) => msg.cardSize = r.read_enum(bytes)?,
                Ok(80) => msg.coverFit = r.read_bool(bytes)?,
                Ok(90) => msg.groupRelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(96) => msg.groupBackgroundColors = r.read_bool(bytes)?,
                Ok(104) => msg.pageLimit = r.read_int32(bytes)?,
                Ok(114) => msg.defaultTemplateId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.defaultObjectTypeId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for View<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.type_pb == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Type::Table { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.sorts.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.filters.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.coverRelationKey == "" { 0 } else { 1 + sizeof_len((&self.coverRelationKey).len()) }
        + if self.hideIcon == false { 0 } else { 1 + sizeof_varint(*(&self.hideIcon) as u64) }
        + if self.cardSize == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Size::Small { 0 } else { 1 + sizeof_varint(*(&self.cardSize) as u64) }
        + if self.coverFit == false { 0 } else { 1 + sizeof_varint(*(&self.coverFit) as u64) }
        + if self.groupRelationKey == "" { 0 } else { 1 + sizeof_len((&self.groupRelationKey).len()) }
        + if self.groupBackgroundColors == false { 0 } else { 1 + sizeof_varint(*(&self.groupBackgroundColors) as u64) }
        + if self.pageLimit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.pageLimit) as u64) }
        + if self.defaultTemplateId == "" { 0 } else { 1 + sizeof_len((&self.defaultTemplateId).len()) }
        + if self.defaultObjectTypeId == "" { 0 } else { 1 + sizeof_len((&self.defaultObjectTypeId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.type_pb != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Type::Table { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        if self.name != "" { w.write_with_tag(26, |w| w.write_string(&**&self.name))?; }
        for s in &self.sorts { w.write_with_tag(34, |w| w.write_message(s))?; }
        for s in &self.filters { w.write_with_tag(42, |w| w.write_message(s))?; }
        for s in &self.relations { w.write_with_tag(50, |w| w.write_message(s))?; }
        if self.coverRelationKey != "" { w.write_with_tag(58, |w| w.write_string(&**&self.coverRelationKey))?; }
        if self.hideIcon != false { w.write_with_tag(64, |w| w.write_bool(*&self.hideIcon))?; }
        if self.cardSize != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_View::Size::Small { w.write_with_tag(72, |w| w.write_enum(*&self.cardSize as i32))?; }
        if self.coverFit != false { w.write_with_tag(80, |w| w.write_bool(*&self.coverFit))?; }
        if self.groupRelationKey != "" { w.write_with_tag(90, |w| w.write_string(&**&self.groupRelationKey))?; }
        if self.groupBackgroundColors != false { w.write_with_tag(96, |w| w.write_bool(*&self.groupBackgroundColors))?; }
        if self.pageLimit != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.pageLimit))?; }
        if self.defaultTemplateId != "" { w.write_with_tag(114, |w| w.write_string(&**&self.defaultTemplateId))?; }
        if self.defaultObjectTypeId != "" { w.write_with_tag(122, |w| w.write_string(&**&self.defaultObjectTypeId))?; }
        Ok(())
    }
}

pub mod mod_View {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Table = 0,
    List = 1,
    Gallery = 2,
    Kanban = 3,
    Calendar = 4,
    Graph = 5,
}

impl Default for Type {
    fn default() -> Self {
        Type::Table
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::Table,
            1 => Type::List,
            2 => Type::Gallery,
            3 => Type::Kanban,
            4 => Type::Calendar,
            5 => Type::Graph,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Table" => Type::Table,
            "List" => Type::List,
            "Gallery" => Type::Gallery,
            "Kanban" => Type::Kanban,
            "Calendar" => Type::Calendar,
            "Graph" => Type::Graph,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Size {
    Small = 0,
    Medium = 1,
    Large = 2,
}

impl Default for Size {
    fn default() -> Self {
        Size::Small
    }
}

impl From<i32> for Size {
    fn from(i: i32) -> Self {
        match i {
            0 => Size::Small,
            1 => Size::Medium,
            2 => Size::Large,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Size {
    fn from(s: &'a str) -> Self {
        match s {
            "Small" => Size::Small,
            "Medium" => Size::Medium,
            "Large" => Size::Large,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation<'a> {
    pub key: Cow<'a, str>,
    pub isVisible: bool,
    pub width: i32,
    pub dateIncludeTime: bool,
    pub timeFormat: model::mod_Block::mod_Content::mod_Dataview::mod_Relation::TimeFormat,
    pub dateFormat: model::mod_Block::mod_Content::mod_Dataview::mod_Relation::DateFormat,
    pub formula: model::mod_Block::mod_Content::mod_Dataview::mod_Relation::FormulaType,
    pub align: model::mod_Block::Align,
}

impl<'a> MessageRead<'a> for Relation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.isVisible = r.read_bool(bytes)?,
                Ok(24) => msg.width = r.read_int32(bytes)?,
                Ok(40) => msg.dateIncludeTime = r.read_bool(bytes)?,
                Ok(48) => msg.timeFormat = r.read_enum(bytes)?,
                Ok(56) => msg.dateFormat = r.read_enum(bytes)?,
                Ok(64) => msg.formula = r.read_enum(bytes)?,
                Ok(72) => msg.align = r.read_enum(bytes)?,
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
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.isVisible == false { 0 } else { 1 + sizeof_varint(*(&self.isVisible) as u64) }
        + if self.width == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.width) as u64) }
        + if self.dateIncludeTime == false { 0 } else { 1 + sizeof_varint(*(&self.dateIncludeTime) as u64) }
        + if self.timeFormat == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Relation::TimeFormat::Format12 { 0 } else { 1 + sizeof_varint(*(&self.timeFormat) as u64) }
        + if self.dateFormat == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Relation::DateFormat::MonthAbbrBeforeDay { 0 } else { 1 + sizeof_varint(*(&self.dateFormat) as u64) }
        + if self.formula == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Relation::FormulaType::None { 0 } else { 1 + sizeof_varint(*(&self.formula) as u64) }
        + if self.align == anytype::model::mod_Block::Align::AlignLeft { 0 } else { 1 + sizeof_varint(*(&self.align) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        if self.isVisible != false { w.write_with_tag(16, |w| w.write_bool(*&self.isVisible))?; }
        if self.width != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.width))?; }
        if self.dateIncludeTime != false { w.write_with_tag(40, |w| w.write_bool(*&self.dateIncludeTime))?; }
        if self.timeFormat != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Relation::TimeFormat::Format12 { w.write_with_tag(48, |w| w.write_enum(*&self.timeFormat as i32))?; }
        if self.dateFormat != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Relation::DateFormat::MonthAbbrBeforeDay { w.write_with_tag(56, |w| w.write_enum(*&self.dateFormat as i32))?; }
        if self.formula != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Relation::FormulaType::None { w.write_with_tag(64, |w| w.write_enum(*&self.formula as i32))?; }
        if self.align != anytype::model::mod_Block::Align::AlignLeft { w.write_with_tag(72, |w| w.write_enum(*&self.align as i32))?; }
        Ok(())
    }
}

pub mod mod_Relation {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DateFormat {
    MonthAbbrBeforeDay = 0,
    MonthAbbrAfterDay = 1,
    Short = 2,
    ShortUS = 3,
    ISO = 4,
}

impl Default for DateFormat {
    fn default() -> Self {
        DateFormat::MonthAbbrBeforeDay
    }
}

impl From<i32> for DateFormat {
    fn from(i: i32) -> Self {
        match i {
            0 => DateFormat::MonthAbbrBeforeDay,
            1 => DateFormat::MonthAbbrAfterDay,
            2 => DateFormat::Short,
            3 => DateFormat::ShortUS,
            4 => DateFormat::ISO,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DateFormat {
    fn from(s: &'a str) -> Self {
        match s {
            "MonthAbbrBeforeDay" => DateFormat::MonthAbbrBeforeDay,
            "MonthAbbrAfterDay" => DateFormat::MonthAbbrAfterDay,
            "Short" => DateFormat::Short,
            "ShortUS" => DateFormat::ShortUS,
            "ISO" => DateFormat::ISO,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TimeFormat {
    Format12 = 0,
    Format24 = 1,
}

impl Default for TimeFormat {
    fn default() -> Self {
        TimeFormat::Format12
    }
}

impl From<i32> for TimeFormat {
    fn from(i: i32) -> Self {
        match i {
            0 => TimeFormat::Format12,
            1 => TimeFormat::Format24,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for TimeFormat {
    fn from(s: &'a str) -> Self {
        match s {
            "Format12" => TimeFormat::Format12,
            "Format24" => TimeFormat::Format24,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FormulaType {
    None = 0,
    Count = 1,
    CountValue = 2,
    CountDistinct = 3,
    CountEmpty = 4,
    CountNotEmpty = 5,
    PercentEmpty = 6,
    PercentNotEmpty = 7,
    MathSum = 8,
    MathAverage = 9,
    MathMedian = 10,
    MathMin = 11,
    MathMax = 12,
    Range = 13,
}

impl Default for FormulaType {
    fn default() -> Self {
        FormulaType::None
    }
}

impl From<i32> for FormulaType {
    fn from(i: i32) -> Self {
        match i {
            0 => FormulaType::None,
            1 => FormulaType::Count,
            2 => FormulaType::CountValue,
            3 => FormulaType::CountDistinct,
            4 => FormulaType::CountEmpty,
            5 => FormulaType::CountNotEmpty,
            6 => FormulaType::PercentEmpty,
            7 => FormulaType::PercentNotEmpty,
            8 => FormulaType::MathSum,
            9 => FormulaType::MathAverage,
            10 => FormulaType::MathMedian,
            11 => FormulaType::MathMin,
            12 => FormulaType::MathMax,
            13 => FormulaType::Range,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for FormulaType {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => FormulaType::None,
            "Count" => FormulaType::Count,
            "CountValue" => FormulaType::CountValue,
            "CountDistinct" => FormulaType::CountDistinct,
            "CountEmpty" => FormulaType::CountEmpty,
            "CountNotEmpty" => FormulaType::CountNotEmpty,
            "PercentEmpty" => FormulaType::PercentEmpty,
            "PercentNotEmpty" => FormulaType::PercentNotEmpty,
            "MathSum" => FormulaType::MathSum,
            "MathAverage" => FormulaType::MathAverage,
            "MathMedian" => FormulaType::MathMedian,
            "MathMin" => FormulaType::MathMin,
            "MathMax" => FormulaType::MathMax,
            "Range" => FormulaType::Range,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sort<'a> {
    pub RelationKey: Cow<'a, str>,
    pub type_pb: model::mod_Block::mod_Content::mod_Dataview::mod_Sort::Type,
    pub customOrder: Vec<google::protobuf::Value<'a>>,
    pub format: model::RelationFormat,
    pub includeTime: bool,
    pub id: Cow<'a, str>,
    pub emptyPlacement: model::mod_Block::mod_Content::mod_Dataview::mod_Sort::EmptyType,
    pub noCollate: bool,
}

impl<'a> MessageRead<'a> for Sort<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.RelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(26) => msg.customOrder.push(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(32) => msg.format = r.read_enum(bytes)?,
                Ok(40) => msg.includeTime = r.read_bool(bytes)?,
                Ok(50) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.emptyPlacement = r.read_enum(bytes)?,
                Ok(64) => msg.noCollate = r.read_bool(bytes)?,
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
        + if self.RelationKey == "" { 0 } else { 1 + sizeof_len((&self.RelationKey).len()) }
        + if self.type_pb == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Sort::Type::Asc { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + self.customOrder.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.format == anytype::model::RelationFormat::longtext { 0 } else { 1 + sizeof_varint(*(&self.format) as u64) }
        + if self.includeTime == false { 0 } else { 1 + sizeof_varint(*(&self.includeTime) as u64) }
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.emptyPlacement == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Sort::EmptyType::NotSpecified { 0 } else { 1 + sizeof_varint(*(&self.emptyPlacement) as u64) }
        + if self.noCollate == false { 0 } else { 1 + sizeof_varint(*(&self.noCollate) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.RelationKey != "" { w.write_with_tag(10, |w| w.write_string(&**&self.RelationKey))?; }
        if self.type_pb != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Sort::Type::Asc { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        for s in &self.customOrder { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.format != anytype::model::RelationFormat::longtext { w.write_with_tag(32, |w| w.write_enum(*&self.format as i32))?; }
        if self.includeTime != false { w.write_with_tag(40, |w| w.write_bool(*&self.includeTime))?; }
        if self.id != "" { w.write_with_tag(50, |w| w.write_string(&**&self.id))?; }
        if self.emptyPlacement != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Sort::EmptyType::NotSpecified { w.write_with_tag(56, |w| w.write_enum(*&self.emptyPlacement as i32))?; }
        if self.noCollate != false { w.write_with_tag(64, |w| w.write_bool(*&self.noCollate))?; }
        Ok(())
    }
}

pub mod mod_Sort {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Asc = 0,
    Desc = 1,
    Custom = 2,
}

impl Default for Type {
    fn default() -> Self {
        Type::Asc
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::Asc,
            1 => Type::Desc,
            2 => Type::Custom,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Asc" => Type::Asc,
            "Desc" => Type::Desc,
            "Custom" => Type::Custom,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EmptyType {
    NotSpecified = 0,
    Start = 1,
    End = 2,
}

impl Default for EmptyType {
    fn default() -> Self {
        EmptyType::NotSpecified
    }
}

impl From<i32> for EmptyType {
    fn from(i: i32) -> Self {
        match i {
            0 => EmptyType::NotSpecified,
            1 => EmptyType::Start,
            2 => EmptyType::End,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EmptyType {
    fn from(s: &'a str) -> Self {
        match s {
            "NotSpecified" => EmptyType::NotSpecified,
            "Start" => EmptyType::Start,
            "End" => EmptyType::End,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Filter<'a> {
    pub id: Cow<'a, str>,
    pub operator: model::mod_Block::mod_Content::mod_Dataview::mod_Filter::Operator,
    pub RelationKey: Cow<'a, str>,
    pub relationProperty: Cow<'a, str>,
    pub condition: model::mod_Block::mod_Content::mod_Dataview::mod_Filter::Condition,
    pub value: Option<google::protobuf::Value<'a>>,
    pub quickOption: model::mod_Block::mod_Content::mod_Dataview::mod_Filter::QuickOption,
    pub format: model::RelationFormat,
    pub includeTime: bool,
    pub nestedFilters: Vec<model::mod_Block::mod_Content::mod_Dataview::Filter<'a>>,
}

impl<'a> MessageRead<'a> for Filter<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(74) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(8) => msg.operator = r.read_enum(bytes)?,
                Ok(18) => msg.RelationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.relationProperty = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.condition = r.read_enum(bytes)?,
                Ok(34) => msg.value = Some(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(48) => msg.quickOption = r.read_enum(bytes)?,
                Ok(56) => msg.format = r.read_enum(bytes)?,
                Ok(64) => msg.includeTime = r.read_bool(bytes)?,
                Ok(82) => msg.nestedFilters.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Filter>(bytes)?),
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
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.operator == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Filter::Operator::No { 0 } else { 1 + sizeof_varint(*(&self.operator) as u64) }
        + if self.RelationKey == "" { 0 } else { 1 + sizeof_len((&self.RelationKey).len()) }
        + if self.relationProperty == "" { 0 } else { 1 + sizeof_len((&self.relationProperty).len()) }
        + if self.condition == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Filter::Condition::None { 0 } else { 1 + sizeof_varint(*(&self.condition) as u64) }
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.quickOption == anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Filter::QuickOption::ExactDate { 0 } else { 1 + sizeof_varint(*(&self.quickOption) as u64) }
        + if self.format == anytype::model::RelationFormat::longtext { 0 } else { 1 + sizeof_varint(*(&self.format) as u64) }
        + if self.includeTime == false { 0 } else { 1 + sizeof_varint(*(&self.includeTime) as u64) }
        + self.nestedFilters.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(74, |w| w.write_string(&**&self.id))?; }
        if self.operator != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Filter::Operator::No { w.write_with_tag(8, |w| w.write_enum(*&self.operator as i32))?; }
        if self.RelationKey != "" { w.write_with_tag(18, |w| w.write_string(&**&self.RelationKey))?; }
        if self.relationProperty != "" { w.write_with_tag(42, |w| w.write_string(&**&self.relationProperty))?; }
        if self.condition != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Filter::Condition::None { w.write_with_tag(24, |w| w.write_enum(*&self.condition as i32))?; }
        if let Some(ref s) = self.value { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.quickOption != anytype::model::mod_Block::mod_Content::mod_Dataview::mod_Filter::QuickOption::ExactDate { w.write_with_tag(48, |w| w.write_enum(*&self.quickOption as i32))?; }
        if self.format != anytype::model::RelationFormat::longtext { w.write_with_tag(56, |w| w.write_enum(*&self.format as i32))?; }
        if self.includeTime != false { w.write_with_tag(64, |w| w.write_bool(*&self.includeTime))?; }
        for s in &self.nestedFilters { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Filter {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Operator {
    No = 0,
    Or = 1,
    And = 2,
}

impl Default for Operator {
    fn default() -> Self {
        Operator::No
    }
}

impl From<i32> for Operator {
    fn from(i: i32) -> Self {
        match i {
            0 => Operator::No,
            1 => Operator::Or,
            2 => Operator::And,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Operator {
    fn from(s: &'a str) -> Self {
        match s {
            "No" => Operator::No,
            "Or" => Operator::Or,
            "And" => Operator::And,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Condition {
    None = 0,
    Equal = 1,
    NotEqual = 2,
    Greater = 3,
    Less = 4,
    GreaterOrEqual = 5,
    LessOrEqual = 6,
    Like = 7,
    NotLike = 8,
    In = 9,
    NotIn = 10,
    Empty = 11,
    NotEmpty = 12,
    AllIn = 13,
    NotAllIn = 14,
    ExactIn = 15,
    NotExactIn = 16,
    Exists = 17,
}

impl Default for Condition {
    fn default() -> Self {
        Condition::None
    }
}

impl From<i32> for Condition {
    fn from(i: i32) -> Self {
        match i {
            0 => Condition::None,
            1 => Condition::Equal,
            2 => Condition::NotEqual,
            3 => Condition::Greater,
            4 => Condition::Less,
            5 => Condition::GreaterOrEqual,
            6 => Condition::LessOrEqual,
            7 => Condition::Like,
            8 => Condition::NotLike,
            9 => Condition::In,
            10 => Condition::NotIn,
            11 => Condition::Empty,
            12 => Condition::NotEmpty,
            13 => Condition::AllIn,
            14 => Condition::NotAllIn,
            15 => Condition::ExactIn,
            16 => Condition::NotExactIn,
            17 => Condition::Exists,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Condition {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => Condition::None,
            "Equal" => Condition::Equal,
            "NotEqual" => Condition::NotEqual,
            "Greater" => Condition::Greater,
            "Less" => Condition::Less,
            "GreaterOrEqual" => Condition::GreaterOrEqual,
            "LessOrEqual" => Condition::LessOrEqual,
            "Like" => Condition::Like,
            "NotLike" => Condition::NotLike,
            "In" => Condition::In,
            "NotIn" => Condition::NotIn,
            "Empty" => Condition::Empty,
            "NotEmpty" => Condition::NotEmpty,
            "AllIn" => Condition::AllIn,
            "NotAllIn" => Condition::NotAllIn,
            "ExactIn" => Condition::ExactIn,
            "NotExactIn" => Condition::NotExactIn,
            "Exists" => Condition::Exists,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuickOption {
    ExactDate = 0,
    Yesterday = 1,
    Today = 2,
    Tomorrow = 3,
    LastWeek = 4,
    CurrentWeek = 5,
    NextWeek = 6,
    LastMonth = 7,
    CurrentMonth = 8,
    NextMonth = 9,
    NumberOfDaysAgo = 10,
    NumberOfDaysNow = 11,
}

impl Default for QuickOption {
    fn default() -> Self {
        QuickOption::ExactDate
    }
}

impl From<i32> for QuickOption {
    fn from(i: i32) -> Self {
        match i {
            0 => QuickOption::ExactDate,
            1 => QuickOption::Yesterday,
            2 => QuickOption::Today,
            3 => QuickOption::Tomorrow,
            4 => QuickOption::LastWeek,
            5 => QuickOption::CurrentWeek,
            6 => QuickOption::NextWeek,
            7 => QuickOption::LastMonth,
            8 => QuickOption::CurrentMonth,
            9 => QuickOption::NextMonth,
            10 => QuickOption::NumberOfDaysAgo,
            11 => QuickOption::NumberOfDaysNow,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for QuickOption {
    fn from(s: &'a str) -> Self {
        match s {
            "ExactDate" => QuickOption::ExactDate,
            "Yesterday" => QuickOption::Yesterday,
            "Today" => QuickOption::Today,
            "Tomorrow" => QuickOption::Tomorrow,
            "LastWeek" => QuickOption::LastWeek,
            "CurrentWeek" => QuickOption::CurrentWeek,
            "NextWeek" => QuickOption::NextWeek,
            "LastMonth" => QuickOption::LastMonth,
            "CurrentMonth" => QuickOption::CurrentMonth,
            "NextMonth" => QuickOption::NextMonth,
            "NumberOfDaysAgo" => QuickOption::NumberOfDaysAgo,
            "NumberOfDaysNow" => QuickOption::NumberOfDaysNow,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GroupOrder<'a> {
    pub viewId: Cow<'a, str>,
    pub viewGroups: Vec<model::mod_Block::mod_Content::mod_Dataview::ViewGroup<'a>>,
}

impl<'a> MessageRead<'a> for GroupOrder<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.viewGroups.push(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::ViewGroup>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GroupOrder<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
        + self.viewGroups.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.viewId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.viewId))?; }
        for s in &self.viewGroups { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ViewGroup<'a> {
    pub groupId: Cow<'a, str>,
    pub index: i32,
    pub hidden: bool,
    pub backgroundColor: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ViewGroup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.groupId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.index = r.read_int32(bytes)?,
                Ok(24) => msg.hidden = r.read_bool(bytes)?,
                Ok(34) => msg.backgroundColor = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ViewGroup<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.groupId == "" { 0 } else { 1 + sizeof_len((&self.groupId).len()) }
        + if self.index == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.index) as u64) }
        + if self.hidden == false { 0 } else { 1 + sizeof_varint(*(&self.hidden) as u64) }
        + if self.backgroundColor == "" { 0 } else { 1 + sizeof_len((&self.backgroundColor).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.groupId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.groupId))?; }
        if self.index != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.index))?; }
        if self.hidden != false { w.write_with_tag(24, |w| w.write_bool(*&self.hidden))?; }
        if self.backgroundColor != "" { w.write_with_tag(34, |w| w.write_string(&**&self.backgroundColor))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectOrder<'a> {
    pub viewId: Cow<'a, str>,
    pub groupId: Cow<'a, str>,
    pub objectIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ObjectOrder<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.groupId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.objectIds.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjectOrder<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
        + if self.groupId == "" { 0 } else { 1 + sizeof_len((&self.groupId).len()) }
        + self.objectIds.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.viewId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.viewId))?; }
        if self.groupId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.groupId))?; }
        for s in &self.objectIds { w.write_with_tag(26, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Group<'a> {
    pub id: Cow<'a, str>,
    pub Value: model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue<'a>,
}

impl<'a> MessageRead<'a> for Group<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.Value = model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::status(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Status>(bytes)?),
                Ok(26) => msg.Value = model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::tag(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Tag>(bytes)?),
                Ok(34) => msg.Value = model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::checkbox(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Checkbox>(bytes)?),
                Ok(42) => msg.Value = model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::date(r.read_message::<model::mod_Block::mod_Content::mod_Dataview::Date>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Group<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + match self.Value {
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::status(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::tag(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::checkbox(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::date(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        match self.Value {            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::status(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::tag(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::checkbox(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::date(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            model::mod_Block::mod_Content::mod_Dataview::mod_Group::OneOfValue::None => {},
    }        Ok(())
    }
}

pub mod mod_Group {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfValue<'a> {
    status(model::mod_Block::mod_Content::mod_Dataview::Status<'a>),
    tag(model::mod_Block::mod_Content::mod_Dataview::Tag<'a>),
    checkbox(model::mod_Block::mod_Content::mod_Dataview::Checkbox),
    date(model::mod_Block::mod_Content::mod_Dataview::Date),
    None,
}

impl<'a> Default for OneOfValue<'a> {
    fn default() -> Self {
        OneOfValue::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Status<'a> {
    pub id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Status<'a> {
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

impl<'a> MessageWrite for Status<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tag<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for Tag<'a> {
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

impl<'a> MessageWrite for Tag<'a> {
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
pub struct Checkbox {
    pub checked: bool,
}

impl<'a> MessageRead<'a> for Checkbox {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.checked = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Checkbox {
    fn get_size(&self) -> usize {
        0
        + if self.checked == false { 0 } else { 1 + sizeof_varint(*(&self.checked) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.checked != false { w.write_with_tag(8, |w| w.write_bool(*&self.checked))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Date { }

impl<'a> MessageRead<'a> for Date {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Date { }

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation<'a> {
    pub key: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Relation<'a> {
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

impl<'a> MessageWrite for Relation<'a> {
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
pub struct Latex<'a> {
    pub text: Cow<'a, str>,
    pub processor: model::mod_Block::mod_Content::mod_Latex::Processor,
}

impl<'a> MessageRead<'a> for Latex<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.processor = r.read_enum(bytes)?,
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
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.processor == anytype::model::mod_Block::mod_Content::mod_Latex::Processor::Latex { 0 } else { 1 + sizeof_varint(*(&self.processor) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != "" { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        if self.processor != anytype::model::mod_Block::mod_Content::mod_Latex::Processor::Latex { w.write_with_tag(16, |w| w.write_enum(*&self.processor as i32))?; }
        Ok(())
    }
}

pub mod mod_Latex {
    use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize,Deserialize)]
pub enum Processor {
    Latex = 0,
    Mermaid = 1,
    Chart = 2,
    Youtube = 3,
    Vimeo = 4,
    Soundcloud = 5,
    GoogleMaps = 6,
    Miro = 7,
    Figma = 8,
    Twitter = 9,
    OpenStreetMap = 10,
    Reddit = 11,
    Facebook = 12,
    Instagram = 13,
    Telegram = 14,
    GithubGist = 15,
    Codepen = 16,
    Bilibili = 17,
    Excalidraw = 18,
    Kroki = 19,
    Graphviz = 20,
    Sketchfab = 21,
    Image = 22,
}

impl Default for Processor {
    fn default() -> Self {
        Processor::Latex
    }
}

impl From<i32> for Processor {
    fn from(i: i32) -> Self {
        match i {
            0 => Processor::Latex,
            1 => Processor::Mermaid,
            2 => Processor::Chart,
            3 => Processor::Youtube,
            4 => Processor::Vimeo,
            5 => Processor::Soundcloud,
            6 => Processor::GoogleMaps,
            7 => Processor::Miro,
            8 => Processor::Figma,
            9 => Processor::Twitter,
            10 => Processor::OpenStreetMap,
            11 => Processor::Reddit,
            12 => Processor::Facebook,
            13 => Processor::Instagram,
            14 => Processor::Telegram,
            15 => Processor::GithubGist,
            16 => Processor::Codepen,
            17 => Processor::Bilibili,
            18 => Processor::Excalidraw,
            19 => Processor::Kroki,
            20 => Processor::Graphviz,
            21 => Processor::Sketchfab,
            22 => Processor::Image,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Processor {
    fn from(s: &'a str) -> Self {
        match s {
            "Latex" => Processor::Latex,
            "Mermaid" => Processor::Mermaid,
            "Chart" => Processor::Chart,
            "Youtube" => Processor::Youtube,
            "Vimeo" => Processor::Vimeo,
            "Soundcloud" => Processor::Soundcloud,
            "GoogleMaps" => Processor::GoogleMaps,
            "Miro" => Processor::Miro,
            "Figma" => Processor::Figma,
            "Twitter" => Processor::Twitter,
            "OpenStreetMap" => Processor::OpenStreetMap,
            "Reddit" => Processor::Reddit,
            "Facebook" => Processor::Facebook,
            "Instagram" => Processor::Instagram,
            "Telegram" => Processor::Telegram,
            "GithubGist" => Processor::GithubGist,
            "Codepen" => Processor::Codepen,
            "Bilibili" => Processor::Bilibili,
            "Excalidraw" => Processor::Excalidraw,
            "Kroki" => Processor::Kroki,
            "Graphviz" => Processor::Graphviz,
            "Sketchfab" => Processor::Sketchfab,
            "Image" => Processor::Image,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableOfContents { }

impl<'a> MessageRead<'a> for TableOfContents {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for TableOfContents { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Table { }

impl<'a> MessageRead<'a> for Table {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Table { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableColumn { }

impl<'a> MessageRead<'a> for TableColumn {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for TableColumn { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TableRow {
    pub isHeader: bool,
}

impl<'a> MessageRead<'a> for TableRow {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.isHeader = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TableRow {
    fn get_size(&self) -> usize {
        0
        + if self.isHeader == false { 0 } else { 1 + sizeof_varint(*(&self.isHeader) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.isHeader != false { w.write_with_tag(8, |w| w.write_bool(*&self.isHeader))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Widget<'a> {
    pub layout: model::mod_Block::mod_Content::mod_Widget::Layout,
    pub limit: i32,
    pub viewId: Cow<'a, str>,
    pub autoAdded: bool,
}

impl<'a> MessageRead<'a> for Widget<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.layout = r.read_enum(bytes)?,
                Ok(16) => msg.limit = r.read_int32(bytes)?,
                Ok(26) => msg.viewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.autoAdded = r.read_bool(bytes)?,
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
        + if self.layout == anytype::model::mod_Block::mod_Content::mod_Widget::Layout::Link { 0 } else { 1 + sizeof_varint(*(&self.layout) as u64) }
        + if self.limit == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.limit) as u64) }
        + if self.viewId == "" { 0 } else { 1 + sizeof_len((&self.viewId).len()) }
        + if self.autoAdded == false { 0 } else { 1 + sizeof_varint(*(&self.autoAdded) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.layout != anytype::model::mod_Block::mod_Content::mod_Widget::Layout::Link { w.write_with_tag(8, |w| w.write_enum(*&self.layout as i32))?; }
        if self.limit != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.limit))?; }
        if self.viewId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.viewId))?; }
        if self.autoAdded != false { w.write_with_tag(32, |w| w.write_bool(*&self.autoAdded))?; }
        Ok(())
    }
}

pub mod mod_Widget {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Layout {
    Link = 0,
    Tree = 1,
    List = 2,
    CompactList = 3,
    View = 4,
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Link
    }
}

impl From<i32> for Layout {
    fn from(i: i32) -> Self {
        match i {
            0 => Layout::Link,
            1 => Layout::Tree,
            2 => Layout::List,
            3 => Layout::CompactList,
            4 => Layout::View,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Layout {
    fn from(s: &'a str) -> Self {
        match s {
            "Link" => Layout::Link,
            "Tree" => Layout::Tree,
            "List" => Layout::List,
            "CompactList" => Layout::CompactList,
            "View" => Layout::View,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chat { }

impl<'a> MessageRead<'a> for Chat {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Chat { }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Position {
    None = 0,
    Top = 1,
    Bottom = 2,
    Left = 3,
    Right = 4,
    Inner = 5,
    Replace = 6,
    InnerFirst = 7,
}

impl Default for Position {
    fn default() -> Self {
        Position::None
    }
}

impl From<i32> for Position {
    fn from(i: i32) -> Self {
        match i {
            0 => Position::None,
            1 => Position::Top,
            2 => Position::Bottom,
            3 => Position::Left,
            4 => Position::Right,
            5 => Position::Inner,
            6 => Position::Replace,
            7 => Position::InnerFirst,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Position {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => Position::None,
            "Top" => Position::Top,
            "Bottom" => Position::Bottom,
            "Left" => Position::Left,
            "Right" => Position::Right,
            "Inner" => Position::Inner,
            "Replace" => Position::Replace,
            "InnerFirst" => Position::InnerFirst,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Align {
    AlignLeft = 0,
    AlignCenter = 1,
    AlignRight = 2,
    AlignJustify = 3,
}

impl Default for Align {
    fn default() -> Self {
        Align::AlignLeft
    }
}

impl From<i32> for Align {
    fn from(i: i32) -> Self {
        match i {
            0 => Align::AlignLeft,
            1 => Align::AlignCenter,
            2 => Align::AlignRight,
            3 => Align::AlignJustify,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Align {
    fn from(s: &'a str) -> Self {
        match s {
            "AlignLeft" => Align::AlignLeft,
            "AlignCenter" => Align::AlignCenter,
            "AlignRight" => Align::AlignRight,
            "AlignJustify" => Align::AlignJustify,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum VerticalAlign {
    VerticalAlignTop = 0,
    VerticalAlignMiddle = 1,
    VerticalAlignBottom = 2,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        VerticalAlign::VerticalAlignTop
    }
}

impl From<i32> for VerticalAlign {
    fn from(i: i32) -> Self {
        match i {
            0 => VerticalAlign::VerticalAlignTop,
            1 => VerticalAlign::VerticalAlignMiddle,
            2 => VerticalAlign::VerticalAlignBottom,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for VerticalAlign {
    fn from(s: &'a str) -> Self {
        match s {
            "VerticalAlignTop" => VerticalAlign::VerticalAlignTop,
            "VerticalAlignMiddle" => VerticalAlign::VerticalAlignMiddle,
            "VerticalAlignBottom" => VerticalAlign::VerticalAlignBottom,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfcontent<'a> {
    smartblock(model::mod_Block::mod_Content::Smartblock),
    text(model::mod_Block::mod_Content::Text<'a>),
    file(model::mod_Block::mod_Content::File<'a>),
    layout(model::mod_Block::mod_Content::Layout),
    div(model::mod_Block::mod_Content::Div),
    bookmark(model::mod_Block::mod_Content::Bookmark<'a>),
    icon(model::mod_Block::mod_Content::Icon<'a>),
    link(model::mod_Block::mod_Content::Link<'a>),
    dataview(model::mod_Block::mod_Content::Dataview<'a>),
    relation(model::mod_Block::mod_Content::Relation<'a>),
    featuredRelations(model::mod_Block::mod_Content::FeaturedRelations),
    latex(model::mod_Block::mod_Content::Latex<'a>),
    tableOfContents(model::mod_Block::mod_Content::TableOfContents),
    table(model::mod_Block::mod_Content::Table),
    tableColumn(model::mod_Block::mod_Content::TableColumn),
    tableRow(model::mod_Block::mod_Content::TableRow),
    widget(model::mod_Block::mod_Content::Widget<'a>),
    chat(model::mod_Block::mod_Content::Chat),
    None,
}

impl<'a> Default for OneOfcontent<'a> {
    fn default() -> Self {
        OneOfcontent::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockMetaOnly<'a> {
    pub id: Cow<'a, str>,
    pub fields: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for BlockMetaOnly<'a> {
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

impl<'a> MessageWrite for BlockMetaOnly<'a> {
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
pub struct Range {
    pub from: i32,
    pub to: i32,
}

impl<'a> MessageRead<'a> for Range {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.from = r.read_int32(bytes)?,
                Ok(16) => msg.to = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Range {
    fn get_size(&self) -> usize {
        0
        + if self.from == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.from) as u64) }
        + if self.to == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.to) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.from != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.from))?; }
        if self.to != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.to))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Account<'a> {
    pub id: Cow<'a, str>,
    pub config: Option<model::mod_Account::Config<'a>>,
    pub status: Option<model::mod_Account::Status>,
    pub info: Option<model::mod_Account::Info<'a>>,
}

impl<'a> MessageRead<'a> for Account<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.config = Some(r.read_message::<model::mod_Account::Config>(bytes)?),
                Ok(42) => msg.status = Some(r.read_message::<model::mod_Account::Status>(bytes)?),
                Ok(50) => msg.info = Some(r.read_message::<model::mod_Account::Info>(bytes)?),
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
        + self.config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.status.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.info.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if let Some(ref s) = self.config { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.status { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.info { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Account {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Config<'a> {
    pub enableDataview: bool,
    pub enableDebug: bool,
    pub enablePrereleaseChannel: bool,
    pub enableSpaces: bool,
    pub extra: Option<google::protobuf::Struct<'a>>,
}

impl<'a> MessageRead<'a> for Config<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.enableDataview = r.read_bool(bytes)?,
                Ok(16) => msg.enableDebug = r.read_bool(bytes)?,
                Ok(24) => msg.enablePrereleaseChannel = r.read_bool(bytes)?,
                Ok(32) => msg.enableSpaces = r.read_bool(bytes)?,
                Ok(802) => msg.extra = Some(r.read_message::<google::protobuf::Struct>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Config<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.enableDataview == false { 0 } else { 1 + sizeof_varint(*(&self.enableDataview) as u64) }
        + if self.enableDebug == false { 0 } else { 1 + sizeof_varint(*(&self.enableDebug) as u64) }
        + if self.enablePrereleaseChannel == false { 0 } else { 1 + sizeof_varint(*(&self.enablePrereleaseChannel) as u64) }
        + if self.enableSpaces == false { 0 } else { 1 + sizeof_varint(*(&self.enableSpaces) as u64) }
        + self.extra.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.enableDataview != false { w.write_with_tag(8, |w| w.write_bool(*&self.enableDataview))?; }
        if self.enableDebug != false { w.write_with_tag(16, |w| w.write_bool(*&self.enableDebug))?; }
        if self.enablePrereleaseChannel != false { w.write_with_tag(24, |w| w.write_bool(*&self.enablePrereleaseChannel))?; }
        if self.enableSpaces != false { w.write_with_tag(32, |w| w.write_bool(*&self.enableSpaces))?; }
        if let Some(ref s) = self.extra { w.write_with_tag(802, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Status {
    pub statusType: model::mod_Account::StatusType,
    pub deletionDate: i64,
}

impl<'a> MessageRead<'a> for Status {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.statusType = r.read_enum(bytes)?,
                Ok(16) => msg.deletionDate = r.read_int64(bytes)?,
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
        + if self.statusType == anytype::model::mod_Account::StatusType::Active { 0 } else { 1 + sizeof_varint(*(&self.statusType) as u64) }
        + if self.deletionDate == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.deletionDate) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.statusType != anytype::model::mod_Account::StatusType::Active { w.write_with_tag(8, |w| w.write_enum(*&self.statusType as i32))?; }
        if self.deletionDate != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.deletionDate))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Info<'a> {
    pub homeObjectId: Cow<'a, str>,
    pub archiveObjectId: Cow<'a, str>,
    pub profileObjectId: Cow<'a, str>,
    pub marketplaceWorkspaceId: Cow<'a, str>,
    pub workspaceObjectId: Cow<'a, str>,
    pub deviceId: Cow<'a, str>,
    pub accountSpaceId: Cow<'a, str>,
    pub widgetsId: Cow<'a, str>,
    pub spaceViewId: Cow<'a, str>,
    pub techSpaceId: Cow<'a, str>,
    pub gatewayUrl: Cow<'a, str>,
    pub localStoragePath: Cow<'a, str>,
    pub timeZone: Cow<'a, str>,
    pub analyticsId: Cow<'a, str>,
    pub networkId: Cow<'a, str>,
    pub ethereumAddress: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Info<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.homeObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.archiveObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.profileObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(90) => msg.marketplaceWorkspaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.workspaceObjectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.deviceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(74) => msg.accountSpaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(82) => msg.widgetsId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(106) => msg.spaceViewId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.techSpaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(810) => msg.gatewayUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(826) => msg.localStoragePath = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(834) => msg.timeZone = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(842) => msg.analyticsId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(850) => msg.networkId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(858) => msg.ethereumAddress = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Info<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.homeObjectId == "" { 0 } else { 1 + sizeof_len((&self.homeObjectId).len()) }
        + if self.archiveObjectId == "" { 0 } else { 1 + sizeof_len((&self.archiveObjectId).len()) }
        + if self.profileObjectId == "" { 0 } else { 1 + sizeof_len((&self.profileObjectId).len()) }
        + if self.marketplaceWorkspaceId == "" { 0 } else { 1 + sizeof_len((&self.marketplaceWorkspaceId).len()) }
        + if self.workspaceObjectId == "" { 0 } else { 1 + sizeof_len((&self.workspaceObjectId).len()) }
        + if self.deviceId == "" { 0 } else { 1 + sizeof_len((&self.deviceId).len()) }
        + if self.accountSpaceId == "" { 0 } else { 1 + sizeof_len((&self.accountSpaceId).len()) }
        + if self.widgetsId == "" { 0 } else { 1 + sizeof_len((&self.widgetsId).len()) }
        + if self.spaceViewId == "" { 0 } else { 1 + sizeof_len((&self.spaceViewId).len()) }
        + if self.techSpaceId == "" { 0 } else { 1 + sizeof_len((&self.techSpaceId).len()) }
        + if self.gatewayUrl == "" { 0 } else { 2 + sizeof_len((&self.gatewayUrl).len()) }
        + if self.localStoragePath == "" { 0 } else { 2 + sizeof_len((&self.localStoragePath).len()) }
        + if self.timeZone == "" { 0 } else { 2 + sizeof_len((&self.timeZone).len()) }
        + if self.analyticsId == "" { 0 } else { 2 + sizeof_len((&self.analyticsId).len()) }
        + if self.networkId == "" { 0 } else { 2 + sizeof_len((&self.networkId).len()) }
        + if self.ethereumAddress == "" { 0 } else { 2 + sizeof_len((&self.ethereumAddress).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.homeObjectId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.homeObjectId))?; }
        if self.archiveObjectId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.archiveObjectId))?; }
        if self.profileObjectId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.profileObjectId))?; }
        if self.marketplaceWorkspaceId != "" { w.write_with_tag(90, |w| w.write_string(&**&self.marketplaceWorkspaceId))?; }
        if self.workspaceObjectId != "" { w.write_with_tag(122, |w| w.write_string(&**&self.workspaceObjectId))?; }
        if self.deviceId != "" { w.write_with_tag(66, |w| w.write_string(&**&self.deviceId))?; }
        if self.accountSpaceId != "" { w.write_with_tag(74, |w| w.write_string(&**&self.accountSpaceId))?; }
        if self.widgetsId != "" { w.write_with_tag(82, |w| w.write_string(&**&self.widgetsId))?; }
        if self.spaceViewId != "" { w.write_with_tag(106, |w| w.write_string(&**&self.spaceViewId))?; }
        if self.techSpaceId != "" { w.write_with_tag(114, |w| w.write_string(&**&self.techSpaceId))?; }
        if self.gatewayUrl != "" { w.write_with_tag(810, |w| w.write_string(&**&self.gatewayUrl))?; }
        if self.localStoragePath != "" { w.write_with_tag(826, |w| w.write_string(&**&self.localStoragePath))?; }
        if self.timeZone != "" { w.write_with_tag(834, |w| w.write_string(&**&self.timeZone))?; }
        if self.analyticsId != "" { w.write_with_tag(842, |w| w.write_string(&**&self.analyticsId))?; }
        if self.networkId != "" { w.write_with_tag(850, |w| w.write_string(&**&self.networkId))?; }
        if self.ethereumAddress != "" { w.write_with_tag(858, |w| w.write_string(&**&self.ethereumAddress))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Auth { }

impl<'a> MessageRead<'a> for Auth {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Auth { }

pub mod mod_Auth {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LocalApiScope {
    Limited = 0,
    JsonAPI = 1,
    Full = 2,
}

impl Default for LocalApiScope {
    fn default() -> Self {
        LocalApiScope::Limited
    }
}

impl From<i32> for LocalApiScope {
    fn from(i: i32) -> Self {
        match i {
            0 => LocalApiScope::Limited,
            1 => LocalApiScope::JsonAPI,
            2 => LocalApiScope::Full,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for LocalApiScope {
    fn from(s: &'a str) -> Self {
        match s {
            "Limited" => LocalApiScope::Limited,
            "JsonAPI" => LocalApiScope::JsonAPI,
            "Full" => LocalApiScope::Full,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusType {
    Active = 0,
    PendingDeletion = 1,
    StartedDeletion = 2,
    Deleted = 3,
}

impl Default for StatusType {
    fn default() -> Self {
        StatusType::Active
    }
}

impl From<i32> for StatusType {
    fn from(i: i32) -> Self {
        match i {
            0 => StatusType::Active,
            1 => StatusType::PendingDeletion,
            2 => StatusType::StartedDeletion,
            3 => StatusType::Deleted,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for StatusType {
    fn from(s: &'a str) -> Self {
        match s {
            "Active" => StatusType::Active,
            "PendingDeletion" => StatusType::PendingDeletion,
            "StartedDeletion" => StatusType::StartedDeletion,
            "Deleted" => StatusType::Deleted,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LinkPreview<'a> {
    pub url: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub imageUrl: Cow<'a, str>,
    pub faviconUrl: Cow<'a, str>,
    pub type_pb: model::mod_LinkPreview::Type,
}

impl<'a> MessageRead<'a> for LinkPreview<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.imageUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.faviconUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(48) => msg.type_pb = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LinkPreview<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.imageUrl == "" { 0 } else { 1 + sizeof_len((&self.imageUrl).len()) }
        + if self.faviconUrl == "" { 0 } else { 1 + sizeof_len((&self.faviconUrl).len()) }
        + if self.type_pb == anytype::model::mod_LinkPreview::Type::Unknown { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.url != "" { w.write_with_tag(10, |w| w.write_string(&**&self.url))?; }
        if self.title != "" { w.write_with_tag(18, |w| w.write_string(&**&self.title))?; }
        if self.description != "" { w.write_with_tag(26, |w| w.write_string(&**&self.description))?; }
        if self.imageUrl != "" { w.write_with_tag(34, |w| w.write_string(&**&self.imageUrl))?; }
        if self.faviconUrl != "" { w.write_with_tag(42, |w| w.write_string(&**&self.faviconUrl))?; }
        if self.type_pb != anytype::model::mod_LinkPreview::Type::Unknown { w.write_with_tag(48, |w| w.write_enum(*&self.type_pb as i32))?; }
        Ok(())
    }
}

pub mod mod_LinkPreview {
    use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Type {
    Unknown = 0,
    Page = 1,
    Image = 2,
    Text = 3,
}

impl Default for Type {
    fn default() -> Self {
        Type::Unknown
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::Unknown,
            1 => Type::Page,
            2 => Type::Image,
            3 => Type::Text,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Unknown" => Type::Unknown,
            "Page" => Type::Page,
            "Image" => Type::Image,
            "Text" => Type::Text,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Restrictions<'a> {
    pub object: Vec<model::mod_Restrictions::ObjectRestriction>,
    pub dataview: Vec<model::mod_Restrictions::DataviewRestrictions<'a>>,
}

impl<'a> MessageRead<'a> for Restrictions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.object = r.read_packed(bytes, |r, bytes| Ok(r.read_enum(bytes)?))?,
                Ok(18) => msg.dataview.push(r.read_message::<model::mod_Restrictions::DataviewRestrictions>(bytes)?),
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
        + if self.object.is_empty() { 0 } else { 1 + sizeof_len(self.object.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + self.dataview.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.object, |w, m| w.write_enum(*m as i32), &|m| sizeof_varint(*(m) as u64))?;
        for s in &self.dataview { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_Restrictions {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DataviewRestrictions<'a> {
    pub blockId: Cow<'a, str>,
    pub restrictions: Vec<model::mod_Restrictions::DataviewRestriction>,
}

impl<'a> MessageRead<'a> for DataviewRestrictions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.blockId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.restrictions.push(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DataviewRestrictions<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.blockId == "" { 0 } else { 1 + sizeof_len((&self.blockId).len()) }
        + self.restrictions.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.blockId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.blockId))?; }
        for s in &self.restrictions { w.write_with_tag(16, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ObjectRestriction {
    None = 0,
    Delete = 1,
    Relations = 2,
    Blocks = 3,
    Details = 4,
    TypeChange = 5,
    LayoutChange = 6,
    Template = 7,
    Duplicate = 8,
    CreateObjectOfThisType = 9,
    Publish = 10,
}

impl Default for ObjectRestriction {
    fn default() -> Self {
        ObjectRestriction::None
    }
}

impl From<i32> for ObjectRestriction {
    fn from(i: i32) -> Self {
        match i {
            0 => ObjectRestriction::None,
            1 => ObjectRestriction::Delete,
            2 => ObjectRestriction::Relations,
            3 => ObjectRestriction::Blocks,
            4 => ObjectRestriction::Details,
            5 => ObjectRestriction::TypeChange,
            6 => ObjectRestriction::LayoutChange,
            7 => ObjectRestriction::Template,
            8 => ObjectRestriction::Duplicate,
            9 => ObjectRestriction::CreateObjectOfThisType,
            10 => ObjectRestriction::Publish,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ObjectRestriction {
    fn from(s: &'a str) -> Self {
        match s {
            "None" => ObjectRestriction::None,
            "Delete" => ObjectRestriction::Delete,
            "Relations" => ObjectRestriction::Relations,
            "Blocks" => ObjectRestriction::Blocks,
            "Details" => ObjectRestriction::Details,
            "TypeChange" => ObjectRestriction::TypeChange,
            "LayoutChange" => ObjectRestriction::LayoutChange,
            "Template" => ObjectRestriction::Template,
            "Duplicate" => ObjectRestriction::Duplicate,
            "CreateObjectOfThisType" => ObjectRestriction::CreateObjectOfThisType,
            "Publish" => ObjectRestriction::Publish,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataviewRestriction {
    DVNone = 0,
    DVRelation = 1,
    DVCreateObject = 2,
    DVViews = 3,
}

impl Default for DataviewRestriction {
    fn default() -> Self {
        DataviewRestriction::DVNone
    }
}

impl From<i32> for DataviewRestriction {
    fn from(i: i32) -> Self {
        match i {
            0 => DataviewRestriction::DVNone,
            1 => DataviewRestriction::DVRelation,
            2 => DataviewRestriction::DVCreateObject,
            3 => DataviewRestriction::DVViews,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DataviewRestriction {
    fn from(s: &'a str) -> Self {
        match s {
            "DVNone" => DataviewRestriction::DVNone,
            "DVRelation" => DataviewRestriction::DVRelation,
            "DVCreateObject" => DataviewRestriction::DVCreateObject,
            "DVViews" => DataviewRestriction::DVViews,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Object {
}

impl<'a> MessageRead<'a> for Object {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let msg = Self::default();
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
pub struct ChangePayload<'a> {
    pub smartBlockType: model::SmartBlockType,
    pub key: Cow<'a, str>,
    pub data: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for ChangePayload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.smartBlockType = r.read_enum(bytes)?,
                Ok(18) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.data = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChangePayload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.smartBlockType == anytype::model::SmartBlockType::AccountOld { 0 } else { 1 + sizeof_varint(*(&self.smartBlockType) as u64) }
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.data == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.data).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.smartBlockType != anytype::model::SmartBlockType::AccountOld { w.write_with_tag(8, |w| w.write_enum(*&self.smartBlockType as i32))?; }
        if self.key != "" { w.write_with_tag(18, |w| w.write_string(&**&self.key))?; }
        if self.data != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.data))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SpaceObjectHeader<'a> {
    pub spaceID: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SpaceObjectHeader<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceID = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SpaceObjectHeader<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceID == "" { 0 } else { 1 + sizeof_len((&self.spaceID).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceID != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceID))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectType<'a> {
    pub url: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub relationLinks: Vec<model::RelationLink<'a>>,
    pub layout: model::mod_ObjectType::Layout,
    pub iconEmoji: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub hidden: bool,
    pub readonly: bool,
    pub types: Vec<model::SmartBlockType>,
    pub isArchived: bool,
    pub installedByDefault: bool,
    pub key: Cow<'a, str>,
    pub revision: i64,
    pub restrictObjectCreation: bool,
    pub iconColor: i64,
    pub iconName: Cow<'a, str>,
    pub pluralName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ObjectType<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.relationLinks.push(r.read_message::<model::RelationLink>(bytes)?),
                Ok(32) => msg.layout = r.read_enum(bytes)?,
                Ok(42) => msg.iconEmoji = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(56) => msg.hidden = r.read_bool(bytes)?,
                Ok(80) => msg.readonly = r.read_bool(bytes)?,
                Ok(66) => msg.types = r.read_packed(bytes, |r, bytes| Ok(r.read_enum(bytes)?))?,
                Ok(72) => msg.isArchived = r.read_bool(bytes)?,
                Ok(88) => msg.installedByDefault = r.read_bool(bytes)?,
                Ok(98) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(104) => msg.revision = r.read_int64(bytes)?,
                Ok(112) => msg.restrictObjectCreation = r.read_bool(bytes)?,
                Ok(120) => msg.iconColor = r.read_int64(bytes)?,
                Ok(130) => msg.iconName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(138) => msg.pluralName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjectType<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.url == "" { 0 } else { 1 + sizeof_len((&self.url).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.layout == anytype::model::mod_ObjectType::Layout::basic { 0 } else { 1 + sizeof_varint(*(&self.layout) as u64) }
        + if self.iconEmoji == "" { 0 } else { 1 + sizeof_len((&self.iconEmoji).len()) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.hidden == false { 0 } else { 1 + sizeof_varint(*(&self.hidden) as u64) }
        + if self.readonly == false { 0 } else { 1 + sizeof_varint(*(&self.readonly) as u64) }
        + if self.types.is_empty() { 0 } else { 1 + sizeof_len(self.types.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.isArchived == false { 0 } else { 1 + sizeof_varint(*(&self.isArchived) as u64) }
        + if self.installedByDefault == false { 0 } else { 1 + sizeof_varint(*(&self.installedByDefault) as u64) }
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.revision == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.revision) as u64) }
        + if self.restrictObjectCreation == false { 0 } else { 1 + sizeof_varint(*(&self.restrictObjectCreation) as u64) }
        + if self.iconColor == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.iconColor) as u64) }
        + if self.iconName == "" { 0 } else { 2 + sizeof_len((&self.iconName).len()) }
        + if self.pluralName == "" { 0 } else { 2 + sizeof_len((&self.pluralName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.url != "" { w.write_with_tag(10, |w| w.write_string(&**&self.url))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        for s in &self.relationLinks { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.layout != anytype::model::mod_ObjectType::Layout::basic { w.write_with_tag(32, |w| w.write_enum(*&self.layout as i32))?; }
        if self.iconEmoji != "" { w.write_with_tag(42, |w| w.write_string(&**&self.iconEmoji))?; }
        if self.description != "" { w.write_with_tag(50, |w| w.write_string(&**&self.description))?; }
        if self.hidden != false { w.write_with_tag(56, |w| w.write_bool(*&self.hidden))?; }
        if self.readonly != false { w.write_with_tag(80, |w| w.write_bool(*&self.readonly))?; }
        w.write_packed_with_tag(66, &self.types, |w, m| w.write_enum(*m as i32), &|m| sizeof_varint(*(m) as u64))?;
        if self.isArchived != false { w.write_with_tag(72, |w| w.write_bool(*&self.isArchived))?; }
        if self.installedByDefault != false { w.write_with_tag(88, |w| w.write_bool(*&self.installedByDefault))?; }
        if self.key != "" { w.write_with_tag(98, |w| w.write_string(&**&self.key))?; }
        if self.revision != 0i64 { w.write_with_tag(104, |w| w.write_int64(*&self.revision))?; }
        if self.restrictObjectCreation != false { w.write_with_tag(112, |w| w.write_bool(*&self.restrictObjectCreation))?; }
        if self.iconColor != 0i64 { w.write_with_tag(120, |w| w.write_int64(*&self.iconColor))?; }
        if self.iconName != "" { w.write_with_tag(130, |w| w.write_string(&**&self.iconName))?; }
        if self.pluralName != "" { w.write_with_tag(138, |w| w.write_string(&**&self.pluralName))?; }
        Ok(())
    }
}

pub mod mod_ObjectType {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Layout {
    basic = 0,
    profile = 1,
    todo = 2,
    set = 3,
    objectType = 4,
    relation = 5,
    file = 6,
    dashboard = 7,
    image = 8,
    note = 9,
    space = 10,
    bookmark = 11,
    relationOptionsList = 12,
    relationOption = 13,
    collection = 14,
    audio = 15,
    video = 16,
    date = 17,
    spaceView = 18,
    participant = 19,
    pdf = 20,
    chat = 21,
    chatDerived = 22,
    tag = 23,
}

impl Default for Layout {
    fn default() -> Self {
        Layout::basic
    }
}

impl From<i32> for Layout {
    fn from(i: i32) -> Self {
        match i {
            0 => Layout::basic,
            1 => Layout::profile,
            2 => Layout::todo,
            3 => Layout::set,
            4 => Layout::objectType,
            5 => Layout::relation,
            6 => Layout::file,
            7 => Layout::dashboard,
            8 => Layout::image,
            9 => Layout::note,
            10 => Layout::space,
            11 => Layout::bookmark,
            12 => Layout::relationOptionsList,
            13 => Layout::relationOption,
            14 => Layout::collection,
            15 => Layout::audio,
            16 => Layout::video,
            17 => Layout::date,
            18 => Layout::spaceView,
            19 => Layout::participant,
            20 => Layout::pdf,
            21 => Layout::chat,
            22 => Layout::chatDerived,
            23 => Layout::tag,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Layout {
    fn from(s: &'a str) -> Self {
        match s {
            "basic" => Layout::basic,
            "profile" => Layout::profile,
            "todo" => Layout::todo,
            "set" => Layout::set,
            "objectType" => Layout::objectType,
            "relation" => Layout::relation,
            "file" => Layout::file,
            "dashboard" => Layout::dashboard,
            "image" => Layout::image,
            "note" => Layout::note,
            "space" => Layout::space,
            "bookmark" => Layout::bookmark,
            "relationOptionsList" => Layout::relationOptionsList,
            "relationOption" => Layout::relationOption,
            "collection" => Layout::collection,
            "audio" => Layout::audio,
            "video" => Layout::video,
            "date" => Layout::date,
            "spaceView" => Layout::spaceView,
            "participant" => Layout::participant,
            "pdf" => Layout::pdf,
            "chat" => Layout::chat,
            "chatDerived" => Layout::chatDerived,
            "tag" => Layout::tag,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Layout<'a> {
    pub id: model::mod_ObjectType::Layout,
    pub name: Cow<'a, str>,
    pub requiredRelations: Vec<model::Relation<'a>>,
}

impl<'a> MessageRead<'a> for Layout<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_enum(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.requiredRelations.push(r.read_message::<model::Relation>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Layout<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == anytype::model::mod_ObjectType::Layout::basic { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.requiredRelations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != anytype::model::mod_ObjectType::Layout::basic { w.write_with_tag(8, |w| w.write_enum(*&self.id as i32))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        for s in &self.requiredRelations { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationWithValue<'a> {
    pub relation: Option<model::Relation<'a>>,
    pub value: Option<google::protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for RelationWithValue<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.relation = Some(r.read_message::<model::Relation>(bytes)?),
                Ok(18) => msg.value = Some(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationWithValue<'a> {
    fn get_size(&self) -> usize {
        0
        + self.relation.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.value.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.relation { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.value { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relation<'a> {
    pub id: Cow<'a, str>,
    pub key: Cow<'a, str>,
    pub format: model::RelationFormat,
    pub name: Cow<'a, str>,
    pub defaultValue: Option<google::protobuf::Value<'a>>,
    pub dataSource: model::mod_Relation::DataSource,
    pub hidden: bool,
    pub readOnly: bool,
    pub readOnlyRelation: bool,
    pub multi: bool,
    pub objectTypes: Vec<Cow<'a, str>>,
    pub selectDict: Vec<model::mod_Relation::Option_pb<'a>>,
    pub maxCount: i32,
    pub description: Cow<'a, str>,
    pub scope: model::mod_Relation::Scope,
    pub creator: Cow<'a, str>,
    pub revision: i64,
}

impl<'a> MessageRead<'a> for Relation<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(802) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.format = r.read_enum(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.defaultValue = Some(r.read_message::<google::protobuf::Value>(bytes)?),
                Ok(40) => msg.dataSource = r.read_enum(bytes)?,
                Ok(48) => msg.hidden = r.read_bool(bytes)?,
                Ok(56) => msg.readOnly = r.read_bool(bytes)?,
                Ok(120) => msg.readOnlyRelation = r.read_bool(bytes)?,
                Ok(64) => msg.multi = r.read_bool(bytes)?,
                Ok(74) => msg.objectTypes.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.selectDict.push(r.read_message::<model::mod_Relation::Option_pb>(bytes)?),
                Ok(104) => msg.maxCount = r.read_int32(bytes)?,
                Ok(114) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(160) => msg.scope = r.read_enum(bytes)?,
                Ok(170) => msg.creator = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(176) => msg.revision = r.read_int64(bytes)?,
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
        + if self.id == "" { 0 } else { 2 + sizeof_len((&self.id).len()) }
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.format == anytype::model::RelationFormat::longtext { 0 } else { 1 + sizeof_varint(*(&self.format) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.defaultValue.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.dataSource == anytype::model::mod_Relation::DataSource::details { 0 } else { 1 + sizeof_varint(*(&self.dataSource) as u64) }
        + if self.hidden == false { 0 } else { 1 + sizeof_varint(*(&self.hidden) as u64) }
        + if self.readOnly == false { 0 } else { 1 + sizeof_varint(*(&self.readOnly) as u64) }
        + if self.readOnlyRelation == false { 0 } else { 1 + sizeof_varint(*(&self.readOnlyRelation) as u64) }
        + if self.multi == false { 0 } else { 1 + sizeof_varint(*(&self.multi) as u64) }
        + self.objectTypes.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + self.selectDict.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.maxCount == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.maxCount) as u64) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.scope == anytype::model::mod_Relation::Scope::object { 0 } else { 2 + sizeof_varint(*(&self.scope) as u64) }
        + if self.creator == "" { 0 } else { 2 + sizeof_len((&self.creator).len()) }
        + if self.revision == 0i64 { 0 } else { 2 + sizeof_varint(*(&self.revision) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(802, |w| w.write_string(&**&self.id))?; }
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        if self.format != anytype::model::RelationFormat::longtext { w.write_with_tag(16, |w| w.write_enum(*&self.format as i32))?; }
        if self.name != "" { w.write_with_tag(26, |w| w.write_string(&**&self.name))?; }
        if let Some(ref s) = self.defaultValue { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.dataSource != anytype::model::mod_Relation::DataSource::details { w.write_with_tag(40, |w| w.write_enum(*&self.dataSource as i32))?; }
        if self.hidden != false { w.write_with_tag(48, |w| w.write_bool(*&self.hidden))?; }
        if self.readOnly != false { w.write_with_tag(56, |w| w.write_bool(*&self.readOnly))?; }
        if self.readOnlyRelation != false { w.write_with_tag(120, |w| w.write_bool(*&self.readOnlyRelation))?; }
        if self.multi != false { w.write_with_tag(64, |w| w.write_bool(*&self.multi))?; }
        for s in &self.objectTypes { w.write_with_tag(74, |w| w.write_string(&**s))?; }
        for s in &self.selectDict { w.write_with_tag(98, |w| w.write_message(s))?; }
        if self.maxCount != 0i32 { w.write_with_tag(104, |w| w.write_int32(*&self.maxCount))?; }
        if self.description != "" { w.write_with_tag(114, |w| w.write_string(&**&self.description))?; }
        if self.scope != anytype::model::mod_Relation::Scope::object { w.write_with_tag(160, |w| w.write_enum(*&self.scope as i32))?; }
        if self.creator != "" { w.write_with_tag(170, |w| w.write_string(&**&self.creator))?; }
        if self.revision != 0i64 { w.write_with_tag(176, |w| w.write_int64(*&self.revision))?; }
        Ok(())
    }
}

pub mod mod_Relation {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Option_pb<'a> {
    pub id: Cow<'a, str>,
    pub text: Cow<'a, str>,
    pub color: Cow<'a, str>,
    pub relationKey: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Option_pb<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.color = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.relationKey = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Option_pb<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.color == "" { 0 } else { 1 + sizeof_len((&self.color).len()) }
        + if self.relationKey == "" { 0 } else { 1 + sizeof_len((&self.relationKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.text != "" { w.write_with_tag(18, |w| w.write_string(&**&self.text))?; }
        if self.color != "" { w.write_with_tag(26, |w| w.write_string(&**&self.color))?; }
        if self.relationKey != "" { w.write_with_tag(42, |w| w.write_string(&**&self.relationKey))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Scope {
    object = 0,
    type_pb = 1,
    setOfTheSameType = 2,
    objectsOfTheSameType = 3,
    library = 4,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::object
    }
}

impl From<i32> for Scope {
    fn from(i: i32) -> Self {
        match i {
            0 => Scope::object,
            1 => Scope::type_pb,
            2 => Scope::setOfTheSameType,
            3 => Scope::objectsOfTheSameType,
            4 => Scope::library,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Scope {
    fn from(s: &'a str) -> Self {
        match s {
            "object" => Scope::object,
            "type_pb" => Scope::type_pb,
            "setOfTheSameType" => Scope::setOfTheSameType,
            "objectsOfTheSameType" => Scope::objectsOfTheSameType,
            "library" => Scope::library,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DataSource {
    details = 0,
    derived = 1,
    account = 2,
    local = 3,
}

impl Default for DataSource {
    fn default() -> Self {
        DataSource::details
    }
}

impl From<i32> for DataSource {
    fn from(i: i32) -> Self {
        match i {
            0 => DataSource::details,
            1 => DataSource::derived,
            2 => DataSource::account,
            3 => DataSource::local,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for DataSource {
    fn from(s: &'a str) -> Self {
        match s {
            "details" => DataSource::details,
            "derived" => DataSource::derived,
            "account" => DataSource::account,
            "local" => DataSource::local,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationLink<'a> {
    pub key: Cow<'a, str>,
    pub format: model::RelationFormat,
}

impl<'a> MessageRead<'a> for RelationLink<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.format = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationLink<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
        + if self.format == anytype::model::RelationFormat::longtext { 0 } else { 1 + sizeof_varint(*(&self.format) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        if self.format != anytype::model::RelationFormat::longtext { w.write_with_tag(16, |w| w.write_enum(*&self.format as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Relations<'a> {
    pub relations: Vec<model::Relation<'a>>,
}

impl<'a> MessageRead<'a> for Relations<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.relations.push(r.read_message::<model::Relation>(bytes)?),
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
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.relations { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RelationOptions<'a> {
    pub options: Vec<model::mod_Relation::Option_pb<'a>>,
}

impl<'a> MessageRead<'a> for RelationOptions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.options.push(r.read_message::<model::mod_Relation::Option_pb>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationOptions<'a> {
    fn get_size(&self) -> usize {
        0
        + self.options.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.options { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct InternalFlag {
    pub value: model::mod_InternalFlag::Value,
}

impl<'a> MessageRead<'a> for InternalFlag {
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

impl MessageWrite for InternalFlag {
    fn get_size(&self) -> usize {
        0
        + if self.value == anytype::model::mod_InternalFlag::Value::editorDeleteEmpty { 0 } else { 1 + sizeof_varint(*(&self.value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.value != anytype::model::mod_InternalFlag::Value::editorDeleteEmpty { w.write_with_tag(8, |w| w.write_enum(*&self.value as i32))?; }
        Ok(())
    }
}

pub mod mod_InternalFlag {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Value {
    editorDeleteEmpty = 0,
    editorSelectType = 1,
    editorSelectTemplate = 2,
    collectionDontIndexLinks = 3,
}

impl Default for Value {
    fn default() -> Self {
        Value::editorDeleteEmpty
    }
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        match i {
            0 => Value::editorDeleteEmpty,
            1 => Value::editorSelectType,
            2 => Value::editorSelectTemplate,
            3 => Value::collectionDontIndexLinks,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        match s {
            "editorDeleteEmpty" => Value::editorDeleteEmpty,
            "editorSelectType" => Value::editorSelectType,
            "editorSelectTemplate" => Value::editorSelectTemplate,
            "collectionDontIndexLinks" => Value::collectionDontIndexLinks,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ObjectView<'a> {
    pub rootId: Cow<'a, str>,
    pub blocks: Vec<model::Block<'a>>,
    pub details: Vec<model::mod_ObjectView::DetailsSet<'a>>,
    pub type_pb: model::SmartBlockType,
    pub relations: Vec<model::Relation<'a>>,
    pub relationLinks: Vec<model::RelationLink<'a>>,
    pub restrictions: Option<model::Restrictions<'a>>,
    pub history: Option<model::mod_ObjectView::HistorySize>,
    pub blockParticipants: Vec<model::mod_ObjectView::BlockParticipant<'a>>,
}

impl<'a> MessageRead<'a> for ObjectView<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rootId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.blocks.push(r.read_message::<model::Block>(bytes)?),
                Ok(26) => msg.details.push(r.read_message::<model::mod_ObjectView::DetailsSet>(bytes)?),
                Ok(32) => msg.type_pb = r.read_enum(bytes)?,
                Ok(58) => msg.relations.push(r.read_message::<model::Relation>(bytes)?),
                Ok(82) => msg.relationLinks.push(r.read_message::<model::RelationLink>(bytes)?),
                Ok(66) => msg.restrictions = Some(r.read_message::<model::Restrictions>(bytes)?),
                Ok(74) => msg.history = Some(r.read_message::<model::mod_ObjectView::HistorySize>(bytes)?),
                Ok(90) => msg.blockParticipants.push(r.read_message::<model::mod_ObjectView::BlockParticipant>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ObjectView<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.rootId == "" { 0 } else { 1 + sizeof_len((&self.rootId).len()) }
        + self.blocks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.details.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.type_pb == anytype::model::SmartBlockType::AccountOld { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.relationLinks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.restrictions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.history.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.blockParticipants.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.rootId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.rootId))?; }
        for s in &self.blocks { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.details { w.write_with_tag(26, |w| w.write_message(s))?; }
        if self.type_pb != anytype::model::SmartBlockType::AccountOld { w.write_with_tag(32, |w| w.write_enum(*&self.type_pb as i32))?; }
        for s in &self.relations { w.write_with_tag(58, |w| w.write_message(s))?; }
        for s in &self.relationLinks { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.restrictions { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.history { w.write_with_tag(74, |w| w.write_message(s))?; }
        for s in &self.blockParticipants { w.write_with_tag(90, |w| w.write_message(s))?; }
        Ok(())
    }
}

pub mod mod_ObjectView {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct DetailsSet<'a> {
    pub id: Cow<'a, str>,
    pub details: Option<google::protobuf::Struct<'a>>,
    pub subIds: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for DetailsSet<'a> {
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

impl<'a> MessageWrite for DetailsSet<'a> {
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
pub struct RelationWithValuePerObject<'a> {
    pub objectId: Cow<'a, str>,
    pub relations: Vec<model::RelationWithValue<'a>>,
}

impl<'a> MessageRead<'a> for RelationWithValuePerObject<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.objectId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.relations.push(r.read_message::<model::RelationWithValue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RelationWithValuePerObject<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.objectId == "" { 0 } else { 1 + sizeof_len((&self.objectId).len()) }
        + self.relations.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.objectId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.objectId))?; }
        for s in &self.relations { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct HistorySize {
    pub undo: i32,
    pub redo: i32,
}

impl<'a> MessageRead<'a> for HistorySize {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.undo = r.read_int32(bytes)?,
                Ok(16) => msg.redo = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HistorySize {
    fn get_size(&self) -> usize {
        0
        + if self.undo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.undo) as u64) }
        + if self.redo == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.redo) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.undo != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.undo))?; }
        if self.redo != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.redo))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BlockParticipant<'a> {
    pub blockId: Cow<'a, str>,
    pub participantId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for BlockParticipant<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.blockId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.participantId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BlockParticipant<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.blockId == "" { 0 } else { 1 + sizeof_len((&self.blockId).len()) }
        + if self.participantId == "" { 0 } else { 1 + sizeof_len((&self.participantId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.blockId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.blockId))?; }
        if self.participantId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.participantId))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParticipantPermissionChange<'a> {
    pub identity: Cow<'a, str>,
    pub perms: model::ParticipantPermissions,
}

impl<'a> MessageRead<'a> for ParticipantPermissionChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.identity = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.perms = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ParticipantPermissionChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.identity == "" { 0 } else { 1 + sizeof_len((&self.identity).len()) }
        + if self.perms == anytype::model::ParticipantPermissions::Reader { 0 } else { 1 + sizeof_varint(*(&self.perms) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.identity != "" { w.write_with_tag(10, |w| w.write_string(&**&self.identity))?; }
        if self.perms != anytype::model::ParticipantPermissions::Reader { w.write_with_tag(16, |w| w.write_enum(*&self.perms as i32))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Metadata<'a> {
    pub payload: model::mod_Metadata::OneOfpayload<'a>,
}

impl<'a> MessageRead<'a> for Metadata<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.payload = model::mod_Metadata::OneOfpayload::identity(r.read_message::<model::mod_Metadata::mod_Payload::IdentityPayload>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Metadata<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.payload {
            model::mod_Metadata::OneOfpayload::identity(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Metadata::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.payload {            model::mod_Metadata::OneOfpayload::identity(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            model::mod_Metadata::OneOfpayload::None => {},
    }        Ok(())
    }
}

pub mod mod_Metadata {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Payload {
}

impl<'a> MessageRead<'a> for Payload {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let msg = Self::default();
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
pub struct IdentityPayload<'a> {
    pub profileSymKey: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for IdentityPayload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.profileSymKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IdentityPayload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.profileSymKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.profileSymKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.profileSymKey != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.profileSymKey))?; }
        Ok(())
    }
}

}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload<'a> {
    identity(model::mod_Metadata::mod_Payload::IdentityPayload<'a>),
    None,
}

impl<'a> Default for OneOfpayload<'a> {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Notification<'a> {
    pub id: Cow<'a, str>,
    pub createTime: i64,
    pub status: model::mod_Notification::Status,
    pub isLocal: bool,
    pub space: Cow<'a, str>,
    pub aclHeadId: Cow<'a, str>,
    pub payload: model::mod_Notification::OneOfpayload<'a>,
}

impl<'a> MessageRead<'a> for Notification<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.createTime = r.read_int64(bytes)?,
                Ok(32) => msg.status = r.read_enum(bytes)?,
                Ok(40) => msg.isLocal = r.read_bool(bytes)?,
                Ok(58) => msg.space = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(114) => msg.aclHeadId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.payload = model::mod_Notification::OneOfpayload::import(r.read_message::<model::mod_Notification::Import>(bytes)?),
                Ok(66) => msg.payload = model::mod_Notification::OneOfpayload::export(r.read_message::<model::mod_Notification::Export>(bytes)?),
                Ok(74) => msg.payload = model::mod_Notification::OneOfpayload::galleryImport(r.read_message::<model::mod_Notification::GalleryImport>(bytes)?),
                Ok(82) => msg.payload = model::mod_Notification::OneOfpayload::requestToJoin(r.read_message::<model::mod_Notification::RequestToJoin>(bytes)?),
                Ok(90) => msg.payload = model::mod_Notification::OneOfpayload::test(r.read_message::<model::mod_Notification::Test>(bytes)?),
                Ok(106) => msg.payload = model::mod_Notification::OneOfpayload::participantRequestApproved(r.read_message::<model::mod_Notification::ParticipantRequestApproved>(bytes)?),
                Ok(122) => msg.payload = model::mod_Notification::OneOfpayload::requestToLeave(r.read_message::<model::mod_Notification::RequestToLeave>(bytes)?),
                Ok(130) => msg.payload = model::mod_Notification::OneOfpayload::participantRemove(r.read_message::<model::mod_Notification::ParticipantRemove>(bytes)?),
                Ok(138) => msg.payload = model::mod_Notification::OneOfpayload::participantRequestDecline(r.read_message::<model::mod_Notification::ParticipantRequestDecline>(bytes)?),
                Ok(146) => msg.payload = model::mod_Notification::OneOfpayload::participantPermissionsChange(r.read_message::<model::mod_Notification::ParticipantPermissionsChange>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Notification<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.createTime == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.createTime) as u64) }
        + if self.status == anytype::model::mod_Notification::Status::Created { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.isLocal == false { 0 } else { 1 + sizeof_varint(*(&self.isLocal) as u64) }
        + if self.space == "" { 0 } else { 1 + sizeof_len((&self.space).len()) }
        + if self.aclHeadId == "" { 0 } else { 1 + sizeof_len((&self.aclHeadId).len()) }
        + match self.payload {
            model::mod_Notification::OneOfpayload::import(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::export(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::galleryImport(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::requestToJoin(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::test(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::participantRequestApproved(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::requestToLeave(ref m) => 1 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::participantRemove(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::participantRequestDecline(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::participantPermissionsChange(ref m) => 2 + sizeof_len((m).get_size()),
            model::mod_Notification::OneOfpayload::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.createTime != 0i64 { w.write_with_tag(16, |w| w.write_int64(*&self.createTime))?; }
        if self.status != anytype::model::mod_Notification::Status::Created { w.write_with_tag(32, |w| w.write_enum(*&self.status as i32))?; }
        if self.isLocal != false { w.write_with_tag(40, |w| w.write_bool(*&self.isLocal))?; }
        if self.space != "" { w.write_with_tag(58, |w| w.write_string(&**&self.space))?; }
        if self.aclHeadId != "" { w.write_with_tag(114, |w| w.write_string(&**&self.aclHeadId))?; }
        match self.payload {            model::mod_Notification::OneOfpayload::import(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::export(ref m) => { w.write_with_tag(66, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::galleryImport(ref m) => { w.write_with_tag(74, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::requestToJoin(ref m) => { w.write_with_tag(82, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::test(ref m) => { w.write_with_tag(90, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::participantRequestApproved(ref m) => { w.write_with_tag(106, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::requestToLeave(ref m) => { w.write_with_tag(122, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::participantRemove(ref m) => { w.write_with_tag(130, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::participantRequestDecline(ref m) => { w.write_with_tag(138, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::participantPermissionsChange(ref m) => { w.write_with_tag(146, |w| w.write_message(m))? },
            model::mod_Notification::OneOfpayload::None => {},
    }        Ok(())
    }
}

pub mod mod_Notification {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Import<'a> {
    pub processId: Cow<'a, str>,
    pub errorCode: model::mod_Import::ErrorCode,
    pub importType: model::mod_Import::Type,
    pub spaceId: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Import<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.processId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.errorCode = r.read_enum(bytes)?,
                Ok(24) => msg.importType = r.read_enum(bytes)?,
                Ok(34) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Import<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.processId == "" { 0 } else { 1 + sizeof_len((&self.processId).len()) }
        + if self.errorCode == anytype::model::mod_Import::ErrorCode::NULL { 0 } else { 1 + sizeof_varint(*(&self.errorCode) as u64) }
        + if self.importType == anytype::model::mod_Import::Type::Notion { 0 } else { 1 + sizeof_varint(*(&self.importType) as u64) }
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.processId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.processId))?; }
        if self.errorCode != anytype::model::mod_Import::ErrorCode::NULL { w.write_with_tag(16, |w| w.write_enum(*&self.errorCode as i32))?; }
        if self.importType != anytype::model::mod_Import::Type::Notion { w.write_with_tag(24, |w| w.write_enum(*&self.importType as i32))?; }
        if self.spaceId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.spaceId))?; }
        if self.name != "" { w.write_with_tag(42, |w| w.write_string(&**&self.name))?; }
        if self.spaceName != "" { w.write_with_tag(50, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Export {
    pub errorCode: model::mod_Notification::mod_Export::Code,
    pub exportType: model::mod_Export::Format,
}

impl<'a> MessageRead<'a> for Export {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(16) => msg.errorCode = r.read_enum(bytes)?,
                Ok(24) => msg.exportType = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Export {
    fn get_size(&self) -> usize {
        0
        + if self.errorCode == anytype::model::mod_Notification::mod_Export::Code::NULL { 0 } else { 1 + sizeof_varint(*(&self.errorCode) as u64) }
        + if self.exportType == anytype::model::mod_Export::Format::Markdown { 0 } else { 1 + sizeof_varint(*(&self.exportType) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.errorCode != anytype::model::mod_Notification::mod_Export::Code::NULL { w.write_with_tag(16, |w| w.write_enum(*&self.errorCode as i32))?; }
        if self.exportType != anytype::model::mod_Export::Format::Markdown { w.write_with_tag(24, |w| w.write_enum(*&self.exportType as i32))?; }
        Ok(())
    }
}

pub mod mod_Export {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Code {
    NULL = 0,
    UNKNOWN_ERROR = 1,
    BAD_INPUT = 2,
}

impl Default for Code {
    fn default() -> Self {
        Code::NULL
    }
}

impl From<i32> for Code {
    fn from(i: i32) -> Self {
        match i {
            0 => Code::NULL,
            1 => Code::UNKNOWN_ERROR,
            2 => Code::BAD_INPUT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Code {
    fn from(s: &'a str) -> Self {
        match s {
            "NULL" => Code::NULL,
            "UNKNOWN_ERROR" => Code::UNKNOWN_ERROR,
            "BAD_INPUT" => Code::BAD_INPUT,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct GalleryImport<'a> {
    pub processId: Cow<'a, str>,
    pub errorCode: model::mod_Import::ErrorCode,
    pub spaceId: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for GalleryImport<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.processId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.errorCode = r.read_enum(bytes)?,
                Ok(26) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for GalleryImport<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.processId == "" { 0 } else { 1 + sizeof_len((&self.processId).len()) }
        + if self.errorCode == anytype::model::mod_Import::ErrorCode::NULL { 0 } else { 1 + sizeof_varint(*(&self.errorCode) as u64) }
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.processId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.processId))?; }
        if self.errorCode != anytype::model::mod_Import::ErrorCode::NULL { w.write_with_tag(16, |w| w.write_enum(*&self.errorCode as i32))?; }
        if self.spaceId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.spaceId))?; }
        if self.name != "" { w.write_with_tag(34, |w| w.write_string(&**&self.name))?; }
        if self.spaceName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestToJoin<'a> {
    pub spaceId: Cow<'a, str>,
    pub identity: Cow<'a, str>,
    pub identityName: Cow<'a, str>,
    pub identityIcon: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RequestToJoin<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.identity = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.identityName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.identityIcon = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestToJoin<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.identity == "" { 0 } else { 1 + sizeof_len((&self.identity).len()) }
        + if self.identityName == "" { 0 } else { 1 + sizeof_len((&self.identityName).len()) }
        + if self.identityIcon == "" { 0 } else { 1 + sizeof_len((&self.identityIcon).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.identity != "" { w.write_with_tag(18, |w| w.write_string(&**&self.identity))?; }
        if self.identityName != "" { w.write_with_tag(26, |w| w.write_string(&**&self.identityName))?; }
        if self.identityIcon != "" { w.write_with_tag(34, |w| w.write_string(&**&self.identityIcon))?; }
        if self.spaceName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Test { }

impl<'a> MessageRead<'a> for Test {
    fn from_reader(r: &mut BytesReader, _: &[u8]) -> Result<Self> {
        r.read_to_end();
        Ok(Self::default())
    }
}

impl MessageWrite for Test { }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParticipantRequestApproved<'a> {
    pub spaceId: Cow<'a, str>,
    pub permissions: model::ParticipantPermissions,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ParticipantRequestApproved<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.permissions = r.read_enum(bytes)?,
                Ok(42) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ParticipantRequestApproved<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.permissions == anytype::model::ParticipantPermissions::Reader { 0 } else { 1 + sizeof_varint(*(&self.permissions) as u64) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.permissions != anytype::model::ParticipantPermissions::Reader { w.write_with_tag(16, |w| w.write_enum(*&self.permissions as i32))?; }
        if self.spaceName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RequestToLeave<'a> {
    pub spaceId: Cow<'a, str>,
    pub identity: Cow<'a, str>,
    pub identityName: Cow<'a, str>,
    pub identityIcon: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RequestToLeave<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.identity = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.identityName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.identityIcon = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RequestToLeave<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.identity == "" { 0 } else { 1 + sizeof_len((&self.identity).len()) }
        + if self.identityName == "" { 0 } else { 1 + sizeof_len((&self.identityName).len()) }
        + if self.identityIcon == "" { 0 } else { 1 + sizeof_len((&self.identityIcon).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.identity != "" { w.write_with_tag(18, |w| w.write_string(&**&self.identity))?; }
        if self.identityName != "" { w.write_with_tag(26, |w| w.write_string(&**&self.identityName))?; }
        if self.identityIcon != "" { w.write_with_tag(34, |w| w.write_string(&**&self.identityIcon))?; }
        if self.spaceName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParticipantRemove<'a> {
    pub identity: Cow<'a, str>,
    pub identityName: Cow<'a, str>,
    pub identityIcon: Cow<'a, str>,
    pub spaceId: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ParticipantRemove<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.identity = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.identityName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.identityIcon = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ParticipantRemove<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.identity == "" { 0 } else { 1 + sizeof_len((&self.identity).len()) }
        + if self.identityName == "" { 0 } else { 1 + sizeof_len((&self.identityName).len()) }
        + if self.identityIcon == "" { 0 } else { 1 + sizeof_len((&self.identityIcon).len()) }
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.identity != "" { w.write_with_tag(10, |w| w.write_string(&**&self.identity))?; }
        if self.identityName != "" { w.write_with_tag(18, |w| w.write_string(&**&self.identityName))?; }
        if self.identityIcon != "" { w.write_with_tag(26, |w| w.write_string(&**&self.identityIcon))?; }
        if self.spaceId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.spaceId))?; }
        if self.spaceName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParticipantRequestDecline<'a> {
    pub spaceId: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ParticipantRequestDecline<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ParticipantRequestDecline<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.spaceName != "" { w.write_with_tag(26, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ParticipantPermissionsChange<'a> {
    pub spaceId: Cow<'a, str>,
    pub permissions: model::ParticipantPermissions,
    pub spaceName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ParticipantPermissionsChange<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.permissions = r.read_enum(bytes)?,
                Ok(26) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ParticipantPermissionsChange<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.permissions == anytype::model::ParticipantPermissions::Reader { 0 } else { 1 + sizeof_varint(*(&self.permissions) as u64) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.spaceId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.spaceId))?; }
        if self.permissions != anytype::model::ParticipantPermissions::Reader { w.write_with_tag(16, |w| w.write_enum(*&self.permissions as i32))?; }
        if self.spaceName != "" { w.write_with_tag(26, |w| w.write_string(&**&self.spaceName))?; }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    Created = 0,
    Shown = 1,
    Read = 2,
    Replied = 3,
}

impl Default for Status {
    fn default() -> Self {
        Status::Created
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            0 => Status::Created,
            1 => Status::Shown,
            2 => Status::Read,
            3 => Status::Replied,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "Created" => Status::Created,
            "Shown" => Status::Shown,
            "Read" => Status::Read,
            "Replied" => Status::Replied,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActionType {
    CLOSE = 0,
}

impl Default for ActionType {
    fn default() -> Self {
        ActionType::CLOSE
    }
}

impl From<i32> for ActionType {
    fn from(i: i32) -> Self {
        match i {
            0 => ActionType::CLOSE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ActionType {
    fn from(s: &'a str) -> Self {
        match s {
            "CLOSE" => ActionType::CLOSE,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfpayload<'a> {
    import(model::mod_Notification::Import<'a>),
    export(model::mod_Notification::Export),
    galleryImport(model::mod_Notification::GalleryImport<'a>),
    requestToJoin(model::mod_Notification::RequestToJoin<'a>),
    test(model::mod_Notification::Test),
    participantRequestApproved(model::mod_Notification::ParticipantRequestApproved<'a>),
    requestToLeave(model::mod_Notification::RequestToLeave<'a>),
    participantRemove(model::mod_Notification::ParticipantRemove<'a>),
    participantRequestDecline(model::mod_Notification::ParticipantRequestDecline<'a>),
    participantPermissionsChange(model::mod_Notification::ParticipantPermissionsChange<'a>),
    None,
}

impl<'a> Default for OneOfpayload<'a> {
    fn default() -> Self {
        OneOfpayload::None
    }
}

}

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

pub mod mod_Export {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Format {
    Markdown = 0,
    Protobuf = 1,
    JSON = 2,
    DOT = 3,
    SVG = 4,
    GRAPH_JSON = 5,
}

impl Default for Format {
    fn default() -> Self {
        Format::Markdown
    }
}

impl From<i32> for Format {
    fn from(i: i32) -> Self {
        match i {
            0 => Format::Markdown,
            1 => Format::Protobuf,
            2 => Format::JSON,
            3 => Format::DOT,
            4 => Format::SVG,
            5 => Format::GRAPH_JSON,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Format {
    fn from(s: &'a str) -> Self {
        match s {
            "Markdown" => Format::Markdown,
            "Protobuf" => Format::Protobuf,
            "JSON" => Format::JSON,
            "DOT" => Format::DOT,
            "SVG" => Format::SVG,
            "GRAPH_JSON" => Format::GRAPH_JSON,
            _ => Self::default(),
        }
    }
}

}

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

pub mod mod_Import {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Notion = 0,
    Markdown = 1,
    External = 2,
    Pb = 3,
    Html = 4,
    Txt = 5,
    Csv = 6,
}

impl Default for Type {
    fn default() -> Self {
        Type::Notion
    }
}

impl From<i32> for Type {
    fn from(i: i32) -> Self {
        match i {
            0 => Type::Notion,
            1 => Type::Markdown,
            2 => Type::External,
            3 => Type::Pb,
            4 => Type::Html,
            5 => Type::Txt,
            6 => Type::Csv,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Type {
    fn from(s: &'a str) -> Self {
        match s {
            "Notion" => Type::Notion,
            "Markdown" => Type::Markdown,
            "External" => Type::External,
            "Pb" => Type::Pb,
            "Html" => Type::Html,
            "Txt" => Type::Txt,
            "Csv" => Type::Csv,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorCode {
    NULL = 0,
    UNKNOWN_ERROR = 1,
    BAD_INPUT = 2,
    INTERNAL_ERROR = 3,
    FILE_LOAD_ERROR = 8,
    IMPORT_IS_CANCELED = 6,
    NOTION_NO_OBJECTS_IN_INTEGRATION = 5,
    NOTION_SERVER_IS_UNAVAILABLE = 12,
    NOTION_RATE_LIMIT_EXCEEDED = 13,
    FILE_IMPORT_NO_OBJECTS_IN_ZIP_ARCHIVE = 14,
    FILE_IMPORT_NO_OBJECTS_IN_DIRECTORY = 17,
    HTML_WRONG_HTML_STRUCTURE = 10,
    PB_NOT_ANYBLOCK_FORMAT = 11,
    CSV_LIMIT_OF_ROWS_OR_RELATIONS_EXCEEDED = 7,
    INSUFFICIENT_PERMISSIONS = 9,
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::NULL
    }
}

impl From<i32> for ErrorCode {
    fn from(i: i32) -> Self {
        match i {
            0 => ErrorCode::NULL,
            1 => ErrorCode::UNKNOWN_ERROR,
            2 => ErrorCode::BAD_INPUT,
            3 => ErrorCode::INTERNAL_ERROR,
            8 => ErrorCode::FILE_LOAD_ERROR,
            6 => ErrorCode::IMPORT_IS_CANCELED,
            5 => ErrorCode::NOTION_NO_OBJECTS_IN_INTEGRATION,
            12 => ErrorCode::NOTION_SERVER_IS_UNAVAILABLE,
            13 => ErrorCode::NOTION_RATE_LIMIT_EXCEEDED,
            14 => ErrorCode::FILE_IMPORT_NO_OBJECTS_IN_ZIP_ARCHIVE,
            17 => ErrorCode::FILE_IMPORT_NO_OBJECTS_IN_DIRECTORY,
            10 => ErrorCode::HTML_WRONG_HTML_STRUCTURE,
            11 => ErrorCode::PB_NOT_ANYBLOCK_FORMAT,
            7 => ErrorCode::CSV_LIMIT_OF_ROWS_OR_RELATIONS_EXCEEDED,
            9 => ErrorCode::INSUFFICIENT_PERMISSIONS,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for ErrorCode {
    fn from(s: &'a str) -> Self {
        match s {
            "NULL" => ErrorCode::NULL,
            "UNKNOWN_ERROR" => ErrorCode::UNKNOWN_ERROR,
            "BAD_INPUT" => ErrorCode::BAD_INPUT,
            "INTERNAL_ERROR" => ErrorCode::INTERNAL_ERROR,
            "FILE_LOAD_ERROR" => ErrorCode::FILE_LOAD_ERROR,
            "IMPORT_IS_CANCELED" => ErrorCode::IMPORT_IS_CANCELED,
            "NOTION_NO_OBJECTS_IN_INTEGRATION" => ErrorCode::NOTION_NO_OBJECTS_IN_INTEGRATION,
            "NOTION_SERVER_IS_UNAVAILABLE" => ErrorCode::NOTION_SERVER_IS_UNAVAILABLE,
            "NOTION_RATE_LIMIT_EXCEEDED" => ErrorCode::NOTION_RATE_LIMIT_EXCEEDED,
            "FILE_IMPORT_NO_OBJECTS_IN_ZIP_ARCHIVE" => ErrorCode::FILE_IMPORT_NO_OBJECTS_IN_ZIP_ARCHIVE,
            "FILE_IMPORT_NO_OBJECTS_IN_DIRECTORY" => ErrorCode::FILE_IMPORT_NO_OBJECTS_IN_DIRECTORY,
            "HTML_WRONG_HTML_STRUCTURE" => ErrorCode::HTML_WRONG_HTML_STRUCTURE,
            "PB_NOT_ANYBLOCK_FORMAT" => ErrorCode::PB_NOT_ANYBLOCK_FORMAT,
            "CSV_LIMIT_OF_ROWS_OR_RELATIONS_EXCEEDED" => ErrorCode::CSV_LIMIT_OF_ROWS_OR_RELATIONS_EXCEEDED,
            "INSUFFICIENT_PERMISSIONS" => ErrorCode::INSUFFICIENT_PERMISSIONS,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Invite<'a> {
    pub payload: Cow<'a, [u8]>,
    pub signature: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for Invite<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.payload = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.signature = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Invite<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.payload == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.payload).len()) }
        + if self.signature == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.signature).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.payload != Cow::Borrowed(b"") { w.write_with_tag(10, |w| w.write_bytes(&**&self.payload))?; }
        if self.signature != Cow::Borrowed(b"") { w.write_with_tag(18, |w| w.write_bytes(&**&self.signature))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct InvitePayload<'a> {
    pub creatorIdentity: Cow<'a, str>,
    pub creatorName: Cow<'a, str>,
    pub aclKey: Cow<'a, [u8]>,
    pub spaceId: Cow<'a, str>,
    pub spaceName: Cow<'a, str>,
    pub spaceIconCid: Cow<'a, str>,
    pub spaceIconEncryptionKeys: Vec<model::FileEncryptionKey<'a>>,
    pub inviteType: model::mod_InvitePayload::InviteType,
    pub guestKey: Cow<'a, [u8]>,
}

impl<'a> MessageRead<'a> for InvitePayload<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.creatorIdentity = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.creatorName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.aclKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.spaceId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.spaceName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.spaceIconCid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.spaceIconEncryptionKeys.push(r.read_message::<model::FileEncryptionKey>(bytes)?),
                Ok(64) => msg.inviteType = r.read_enum(bytes)?,
                Ok(74) => msg.guestKey = r.read_bytes(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for InvitePayload<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.creatorIdentity == "" { 0 } else { 1 + sizeof_len((&self.creatorIdentity).len()) }
        + if self.creatorName == "" { 0 } else { 1 + sizeof_len((&self.creatorName).len()) }
        + if self.aclKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.aclKey).len()) }
        + if self.spaceId == "" { 0 } else { 1 + sizeof_len((&self.spaceId).len()) }
        + if self.spaceName == "" { 0 } else { 1 + sizeof_len((&self.spaceName).len()) }
        + if self.spaceIconCid == "" { 0 } else { 1 + sizeof_len((&self.spaceIconCid).len()) }
        + self.spaceIconEncryptionKeys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.inviteType == anytype::model::mod_InvitePayload::InviteType::JoinAsMember { 0 } else { 1 + sizeof_varint(*(&self.inviteType) as u64) }
        + if self.guestKey == Cow::Borrowed(b"") { 0 } else { 1 + sizeof_len((&self.guestKey).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.creatorIdentity != "" { w.write_with_tag(10, |w| w.write_string(&**&self.creatorIdentity))?; }
        if self.creatorName != "" { w.write_with_tag(18, |w| w.write_string(&**&self.creatorName))?; }
        if self.aclKey != Cow::Borrowed(b"") { w.write_with_tag(26, |w| w.write_bytes(&**&self.aclKey))?; }
        if self.spaceId != "" { w.write_with_tag(34, |w| w.write_string(&**&self.spaceId))?; }
        if self.spaceName != "" { w.write_with_tag(42, |w| w.write_string(&**&self.spaceName))?; }
        if self.spaceIconCid != "" { w.write_with_tag(50, |w| w.write_string(&**&self.spaceIconCid))?; }
        for s in &self.spaceIconEncryptionKeys { w.write_with_tag(58, |w| w.write_message(s))?; }
        if self.inviteType != anytype::model::mod_InvitePayload::InviteType::JoinAsMember { w.write_with_tag(64, |w| w.write_enum(*&self.inviteType as i32))?; }
        if self.guestKey != Cow::Borrowed(b"") { w.write_with_tag(74, |w| w.write_bytes(&**&self.guestKey))?; }
        Ok(())
    }
}

pub mod mod_InvitePayload {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InviteType {
    JoinAsMember = 0,
    JoinAsGuest = 1,
}

impl Default for InviteType {
    fn default() -> Self {
        InviteType::JoinAsMember
    }
}

impl From<i32> for InviteType {
    fn from(i: i32) -> Self {
        match i {
            0 => InviteType::JoinAsMember,
            1 => InviteType::JoinAsGuest,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for InviteType {
    fn from(s: &'a str) -> Self {
        match s {
            "JoinAsMember" => InviteType::JoinAsMember,
            "JoinAsGuest" => InviteType::JoinAsGuest,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IdentityProfile<'a> {
    pub identity: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub iconCid: Cow<'a, str>,
    pub iconEncryptionKeys: Vec<model::FileEncryptionKey<'a>>,
    pub description: Cow<'a, str>,
    pub globalName: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for IdentityProfile<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.identity = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.iconCid = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.iconEncryptionKeys.push(r.read_message::<model::FileEncryptionKey>(bytes)?),
                Ok(42) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.globalName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for IdentityProfile<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.identity == "" { 0 } else { 1 + sizeof_len((&self.identity).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.iconCid == "" { 0 } else { 1 + sizeof_len((&self.iconCid).len()) }
        + self.iconEncryptionKeys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.globalName == "" { 0 } else { 1 + sizeof_len((&self.globalName).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.identity != "" { w.write_with_tag(10, |w| w.write_string(&**&self.identity))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.iconCid != "" { w.write_with_tag(26, |w| w.write_string(&**&self.iconCid))?; }
        for s in &self.iconEncryptionKeys { w.write_with_tag(34, |w| w.write_message(s))?; }
        if self.description != "" { w.write_with_tag(42, |w| w.write_string(&**&self.description))?; }
        if self.globalName != "" { w.write_with_tag(50, |w| w.write_string(&**&self.globalName))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileInfo<'a> {
    pub fileId: Cow<'a, str>,
    pub encryptionKeys: Vec<model::FileEncryptionKey<'a>>,
}

impl<'a> MessageRead<'a> for FileInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fileId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.encryptionKeys.push(r.read_message::<model::FileEncryptionKey>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.fileId == "" { 0 } else { 1 + sizeof_len((&self.fileId).len()) }
        + self.encryptionKeys.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.fileId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.fileId))?; }
        for s in &self.encryptionKeys { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FileEncryptionKey<'a> {
    pub path: Cow<'a, str>,
    pub key: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for FileEncryptionKey<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.path = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FileEncryptionKey<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.path == "" { 0 } else { 1 + sizeof_len((&self.path).len()) }
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.path != "" { w.write_with_tag(10, |w| w.write_string(&**&self.path))?; }
        if self.key != "" { w.write_with_tag(18, |w| w.write_string(&**&self.key))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ManifestInfo<'a> {
    pub schema: Cow<'a, str>,
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub author: Cow<'a, str>,
    pub license: Cow<'a, str>,
    pub title: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub screenshots: Vec<Cow<'a, str>>,
    pub downloadLink: Cow<'a, str>,
    pub fileSize: i32,
    pub categories: Vec<Cow<'a, str>>,
    pub language: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ManifestInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.schema = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.author = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.license = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.title = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(58) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(66) => msg.screenshots.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(74) => msg.downloadLink = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.fileSize = r.read_int32(bytes)?,
                Ok(90) => msg.categories.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(98) => msg.language = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ManifestInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.schema == "" { 0 } else { 1 + sizeof_len((&self.schema).len()) }
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.author == "" { 0 } else { 1 + sizeof_len((&self.author).len()) }
        + if self.license == "" { 0 } else { 1 + sizeof_len((&self.license).len()) }
        + if self.title == "" { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + self.screenshots.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.downloadLink == "" { 0 } else { 1 + sizeof_len((&self.downloadLink).len()) }
        + if self.fileSize == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fileSize) as u64) }
        + self.categories.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.language == "" { 0 } else { 1 + sizeof_len((&self.language).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.schema != "" { w.write_with_tag(10, |w| w.write_string(&**&self.schema))?; }
        if self.id != "" { w.write_with_tag(18, |w| w.write_string(&**&self.id))?; }
        if self.name != "" { w.write_with_tag(26, |w| w.write_string(&**&self.name))?; }
        if self.author != "" { w.write_with_tag(34, |w| w.write_string(&**&self.author))?; }
        if self.license != "" { w.write_with_tag(42, |w| w.write_string(&**&self.license))?; }
        if self.title != "" { w.write_with_tag(50, |w| w.write_string(&**&self.title))?; }
        if self.description != "" { w.write_with_tag(58, |w| w.write_string(&**&self.description))?; }
        for s in &self.screenshots { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        if self.downloadLink != "" { w.write_with_tag(74, |w| w.write_string(&**&self.downloadLink))?; }
        if self.fileSize != 0i32 { w.write_with_tag(80, |w| w.write_int32(*&self.fileSize))?; }
        for s in &self.categories { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if self.language != "" { w.write_with_tag(98, |w| w.write_string(&**&self.language))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Membership<'a> {
    pub tier: u32,
    pub status: model::mod_Membership::Status,
    pub dateStarted: u64,
    pub dateEnds: u64,
    pub isAutoRenew: bool,
    pub paymentMethod: model::mod_Membership::PaymentMethod,
    pub nsName: Cow<'a, str>,
    pub nsNameType: model::NameserviceNameType,
    pub userEmail: Cow<'a, str>,
    pub subscribeToNewsletter: bool,
}

impl<'a> MessageRead<'a> for Membership<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.tier = r.read_uint32(bytes)?,
                Ok(16) => msg.status = r.read_enum(bytes)?,
                Ok(24) => msg.dateStarted = r.read_uint64(bytes)?,
                Ok(32) => msg.dateEnds = r.read_uint64(bytes)?,
                Ok(40) => msg.isAutoRenew = r.read_bool(bytes)?,
                Ok(48) => msg.paymentMethod = r.read_enum(bytes)?,
                Ok(58) => msg.nsName = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(64) => msg.nsNameType = r.read_enum(bytes)?,
                Ok(74) => msg.userEmail = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(80) => msg.subscribeToNewsletter = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Membership<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.tier == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.tier) as u64) }
        + if self.status == anytype::model::mod_Membership::Status::StatusUnknown { 0 } else { 1 + sizeof_varint(*(&self.status) as u64) }
        + if self.dateStarted == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.dateStarted) as u64) }
        + if self.dateEnds == 0u64 { 0 } else { 1 + sizeof_varint(*(&self.dateEnds) as u64) }
        + if self.isAutoRenew == false { 0 } else { 1 + sizeof_varint(*(&self.isAutoRenew) as u64) }
        + if self.paymentMethod == anytype::model::mod_Membership::PaymentMethod::MethodNone { 0 } else { 1 + sizeof_varint(*(&self.paymentMethod) as u64) }
        + if self.nsName == "" { 0 } else { 1 + sizeof_len((&self.nsName).len()) }
        + if self.nsNameType == anytype::model::NameserviceNameType::AnyName { 0 } else { 1 + sizeof_varint(*(&self.nsNameType) as u64) }
        + if self.userEmail == "" { 0 } else { 1 + sizeof_len((&self.userEmail).len()) }
        + if self.subscribeToNewsletter == false { 0 } else { 1 + sizeof_varint(*(&self.subscribeToNewsletter) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.tier != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.tier))?; }
        if self.status != anytype::model::mod_Membership::Status::StatusUnknown { w.write_with_tag(16, |w| w.write_enum(*&self.status as i32))?; }
        if self.dateStarted != 0u64 { w.write_with_tag(24, |w| w.write_uint64(*&self.dateStarted))?; }
        if self.dateEnds != 0u64 { w.write_with_tag(32, |w| w.write_uint64(*&self.dateEnds))?; }
        if self.isAutoRenew != false { w.write_with_tag(40, |w| w.write_bool(*&self.isAutoRenew))?; }
        if self.paymentMethod != anytype::model::mod_Membership::PaymentMethod::MethodNone { w.write_with_tag(48, |w| w.write_enum(*&self.paymentMethod as i32))?; }
        if self.nsName != "" { w.write_with_tag(58, |w| w.write_string(&**&self.nsName))?; }
        if self.nsNameType != anytype::model::NameserviceNameType::AnyName { w.write_with_tag(64, |w| w.write_enum(*&self.nsNameType as i32))?; }
        if self.userEmail != "" { w.write_with_tag(74, |w| w.write_string(&**&self.userEmail))?; }
        if self.subscribeToNewsletter != false { w.write_with_tag(80, |w| w.write_bool(*&self.subscribeToNewsletter))?; }
        Ok(())
    }
}

pub mod mod_Membership {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    StatusUnknown = 0,
    StatusPending = 1,
    StatusActive = 2,
    StatusPendingRequiresFinalization = 3,
}

impl Default for Status {
    fn default() -> Self {
        Status::StatusUnknown
    }
}

impl From<i32> for Status {
    fn from(i: i32) -> Self {
        match i {
            0 => Status::StatusUnknown,
            1 => Status::StatusPending,
            2 => Status::StatusActive,
            3 => Status::StatusPendingRequiresFinalization,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Self {
        match s {
            "StatusUnknown" => Status::StatusUnknown,
            "StatusPending" => Status::StatusPending,
            "StatusActive" => Status::StatusActive,
            "StatusPendingRequiresFinalization" => Status::StatusPendingRequiresFinalization,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PaymentMethod {
    MethodNone = 0,
    MethodStripe = 1,
    MethodCrypto = 2,
    MethodInappApple = 3,
    MethodInappGoogle = 4,
}

impl Default for PaymentMethod {
    fn default() -> Self {
        PaymentMethod::MethodNone
    }
}

impl From<i32> for PaymentMethod {
    fn from(i: i32) -> Self {
        match i {
            0 => PaymentMethod::MethodNone,
            1 => PaymentMethod::MethodStripe,
            2 => PaymentMethod::MethodCrypto,
            3 => PaymentMethod::MethodInappApple,
            4 => PaymentMethod::MethodInappGoogle,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PaymentMethod {
    fn from(s: &'a str) -> Self {
        match s {
            "MethodNone" => PaymentMethod::MethodNone,
            "MethodStripe" => PaymentMethod::MethodStripe,
            "MethodCrypto" => PaymentMethod::MethodCrypto,
            "MethodInappApple" => PaymentMethod::MethodInappApple,
            "MethodInappGoogle" => PaymentMethod::MethodInappGoogle,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EmailVerificationStatus {
    StatusNotVerified = 0,
    StatusCodeSent = 1,
    StatusVerified = 2,
}

impl Default for EmailVerificationStatus {
    fn default() -> Self {
        EmailVerificationStatus::StatusNotVerified
    }
}

impl From<i32> for EmailVerificationStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => EmailVerificationStatus::StatusNotVerified,
            1 => EmailVerificationStatus::StatusCodeSent,
            2 => EmailVerificationStatus::StatusVerified,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for EmailVerificationStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "StatusNotVerified" => EmailVerificationStatus::StatusNotVerified,
            "StatusCodeSent" => EmailVerificationStatus::StatusCodeSent,
            "StatusVerified" => EmailVerificationStatus::StatusVerified,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MembershipTierData<'a> {
    pub id: u32,
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub isTest: bool,
    pub periodType: model::mod_MembershipTierData::PeriodType,
    pub periodValue: u32,
    pub priceStripeUsdCents: u32,
    pub anyNamesCountIncluded: u32,
    pub anyNameMinLength: u32,
    pub features: Vec<Cow<'a, str>>,
    pub colorStr: Cow<'a, str>,
    pub stripeProductId: Cow<'a, str>,
    pub stripeManageUrl: Cow<'a, str>,
    pub iosProductId: Cow<'a, str>,
    pub iosManageUrl: Cow<'a, str>,
    pub androidProductId: Cow<'a, str>,
    pub androidManageUrl: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for MembershipTierData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.description = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.isTest = r.read_bool(bytes)?,
                Ok(40) => msg.periodType = r.read_enum(bytes)?,
                Ok(48) => msg.periodValue = r.read_uint32(bytes)?,
                Ok(56) => msg.priceStripeUsdCents = r.read_uint32(bytes)?,
                Ok(64) => msg.anyNamesCountIncluded = r.read_uint32(bytes)?,
                Ok(72) => msg.anyNameMinLength = r.read_uint32(bytes)?,
                Ok(82) => msg.features.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(90) => msg.colorStr = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(98) => msg.stripeProductId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(106) => msg.stripeManageUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(122) => msg.iosProductId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(130) => msg.iosManageUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(138) => msg.androidProductId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(146) => msg.androidManageUrl = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MembershipTierData<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.description == "" { 0 } else { 1 + sizeof_len((&self.description).len()) }
        + if self.isTest == false { 0 } else { 1 + sizeof_varint(*(&self.isTest) as u64) }
        + if self.periodType == anytype::model::mod_MembershipTierData::PeriodType::PeriodTypeUnknown { 0 } else { 1 + sizeof_varint(*(&self.periodType) as u64) }
        + if self.periodValue == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.periodValue) as u64) }
        + if self.priceStripeUsdCents == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.priceStripeUsdCents) as u64) }
        + if self.anyNamesCountIncluded == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.anyNamesCountIncluded) as u64) }
        + if self.anyNameMinLength == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.anyNameMinLength) as u64) }
        + self.features.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
        + if self.colorStr == "" { 0 } else { 1 + sizeof_len((&self.colorStr).len()) }
        + if self.stripeProductId == "" { 0 } else { 1 + sizeof_len((&self.stripeProductId).len()) }
        + if self.stripeManageUrl == "" { 0 } else { 1 + sizeof_len((&self.stripeManageUrl).len()) }
        + if self.iosProductId == "" { 0 } else { 1 + sizeof_len((&self.iosProductId).len()) }
        + if self.iosManageUrl == "" { 0 } else { 2 + sizeof_len((&self.iosManageUrl).len()) }
        + if self.androidProductId == "" { 0 } else { 2 + sizeof_len((&self.androidProductId).len()) }
        + if self.androidManageUrl == "" { 0 } else { 2 + sizeof_len((&self.androidManageUrl).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.description != "" { w.write_with_tag(26, |w| w.write_string(&**&self.description))?; }
        if self.isTest != false { w.write_with_tag(32, |w| w.write_bool(*&self.isTest))?; }
        if self.periodType != anytype::model::mod_MembershipTierData::PeriodType::PeriodTypeUnknown { w.write_with_tag(40, |w| w.write_enum(*&self.periodType as i32))?; }
        if self.periodValue != 0u32 { w.write_with_tag(48, |w| w.write_uint32(*&self.periodValue))?; }
        if self.priceStripeUsdCents != 0u32 { w.write_with_tag(56, |w| w.write_uint32(*&self.priceStripeUsdCents))?; }
        if self.anyNamesCountIncluded != 0u32 { w.write_with_tag(64, |w| w.write_uint32(*&self.anyNamesCountIncluded))?; }
        if self.anyNameMinLength != 0u32 { w.write_with_tag(72, |w| w.write_uint32(*&self.anyNameMinLength))?; }
        for s in &self.features { w.write_with_tag(82, |w| w.write_string(&**s))?; }
        if self.colorStr != "" { w.write_with_tag(90, |w| w.write_string(&**&self.colorStr))?; }
        if self.stripeProductId != "" { w.write_with_tag(98, |w| w.write_string(&**&self.stripeProductId))?; }
        if self.stripeManageUrl != "" { w.write_with_tag(106, |w| w.write_string(&**&self.stripeManageUrl))?; }
        if self.iosProductId != "" { w.write_with_tag(122, |w| w.write_string(&**&self.iosProductId))?; }
        if self.iosManageUrl != "" { w.write_with_tag(130, |w| w.write_string(&**&self.iosManageUrl))?; }
        if self.androidProductId != "" { w.write_with_tag(138, |w| w.write_string(&**&self.androidProductId))?; }
        if self.androidManageUrl != "" { w.write_with_tag(146, |w| w.write_string(&**&self.androidManageUrl))?; }
        Ok(())
    }
}

pub mod mod_MembershipTierData {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PeriodType {
    PeriodTypeUnknown = 0,
    PeriodTypeUnlimited = 1,
    PeriodTypeDays = 2,
    PeriodTypeWeeks = 3,
    PeriodTypeMonths = 4,
    PeriodTypeYears = 5,
}

impl Default for PeriodType {
    fn default() -> Self {
        PeriodType::PeriodTypeUnknown
    }
}

impl From<i32> for PeriodType {
    fn from(i: i32) -> Self {
        match i {
            0 => PeriodType::PeriodTypeUnknown,
            1 => PeriodType::PeriodTypeUnlimited,
            2 => PeriodType::PeriodTypeDays,
            3 => PeriodType::PeriodTypeWeeks,
            4 => PeriodType::PeriodTypeMonths,
            5 => PeriodType::PeriodTypeYears,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for PeriodType {
    fn from(s: &'a str) -> Self {
        match s {
            "PeriodTypeUnknown" => PeriodType::PeriodTypeUnknown,
            "PeriodTypeUnlimited" => PeriodType::PeriodTypeUnlimited,
            "PeriodTypeDays" => PeriodType::PeriodTypeDays,
            "PeriodTypeWeeks" => PeriodType::PeriodTypeWeeks,
            "PeriodTypeMonths" => PeriodType::PeriodTypeMonths,
            "PeriodTypeYears" => PeriodType::PeriodTypeYears,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Detail<'a> {
    pub key: Cow<'a, str>,
    pub value: Option<google::protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for Detail<'a> {
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

impl<'a> MessageWrite for Detail<'a> {
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
pub struct DeviceInfo<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub addDate: i64,
    pub archived: bool,
    pub isConnected: bool,
}

impl<'a> MessageRead<'a> for DeviceInfo<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(24) => msg.addDate = r.read_int64(bytes)?,
                Ok(32) => msg.archived = r.read_bool(bytes)?,
                Ok(40) => msg.isConnected = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for DeviceInfo<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.name == "" { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.addDate == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.addDate) as u64) }
        + if self.archived == false { 0 } else { 1 + sizeof_varint(*(&self.archived) as u64) }
        + if self.isConnected == false { 0 } else { 1 + sizeof_varint(*(&self.isConnected) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.name != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.addDate != 0i64 { w.write_with_tag(24, |w| w.write_int64(*&self.addDate))?; }
        if self.archived != false { w.write_with_tag(32, |w| w.write_bool(*&self.archived))?; }
        if self.isConnected != false { w.write_with_tag(40, |w| w.write_bool(*&self.isConnected))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatState<'a> {
    pub messages: Option<model::mod_ChatState::UnreadState<'a>>,
    pub mentions: Option<model::mod_ChatState::UnreadState<'a>>,
    pub lastStateId: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ChatState<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.messages = Some(r.read_message::<model::mod_ChatState::UnreadState>(bytes)?),
                Ok(18) => msg.mentions = Some(r.read_message::<model::mod_ChatState::UnreadState>(bytes)?),
                Ok(26) => msg.lastStateId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatState<'a> {
    fn get_size(&self) -> usize {
        0
        + self.messages.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mentions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.lastStateId == "" { 0 } else { 1 + sizeof_len((&self.lastStateId).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.messages { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mentions { w.write_with_tag(18, |w| w.write_message(s))?; }
        if self.lastStateId != "" { w.write_with_tag(26, |w| w.write_string(&**&self.lastStateId))?; }
        Ok(())
    }
}

pub mod mod_ChatState {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UnreadState<'a> {
    pub oldestOrderId: Cow<'a, str>,
    pub counter: i32,
}

impl<'a> MessageRead<'a> for UnreadState<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.oldestOrderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.counter = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UnreadState<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.oldestOrderId == "" { 0 } else { 1 + sizeof_len((&self.oldestOrderId).len()) }
        + if self.counter == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.counter) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.oldestOrderId != "" { w.write_with_tag(10, |w| w.write_string(&**&self.oldestOrderId))?; }
        if self.counter != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.counter))?; }
        Ok(())
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChatMessage<'a> {
    pub id: Cow<'a, str>,
    pub orderId: Cow<'a, str>,
    pub creator: Cow<'a, str>,
    pub createdAt: i64,
    pub modifiedAt: i64,
    pub stateId: Cow<'a, str>,
    pub replyToMessageId: Cow<'a, str>,
    pub message: Option<model::mod_ChatMessage::MessageContent<'a>>,
    pub attachments: Vec<model::mod_ChatMessage::Attachment<'a>>,
    pub reactions: Option<model::mod_ChatMessage::Reactions<'a>>,
    pub read: bool,
    pub mentionRead: bool,
}

impl<'a> MessageRead<'a> for ChatMessage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.orderId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.creator = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(32) => msg.createdAt = r.read_int64(bytes)?,
                Ok(72) => msg.modifiedAt = r.read_int64(bytes)?,
                Ok(90) => msg.stateId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.replyToMessageId = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(50) => msg.message = Some(r.read_message::<model::mod_ChatMessage::MessageContent>(bytes)?),
                Ok(58) => msg.attachments.push(r.read_message::<model::mod_ChatMessage::Attachment>(bytes)?),
                Ok(66) => msg.reactions = Some(r.read_message::<model::mod_ChatMessage::Reactions>(bytes)?),
                Ok(80) => msg.read = r.read_bool(bytes)?,
                Ok(96) => msg.mentionRead = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ChatMessage<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.id == "" { 0 } else { 1 + sizeof_len((&self.id).len()) }
        + if self.orderId == "" { 0 } else { 1 + sizeof_len((&self.orderId).len()) }
        + if self.creator == "" { 0 } else { 1 + sizeof_len((&self.creator).len()) }
        + if self.createdAt == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.createdAt) as u64) }
        + if self.modifiedAt == 0i64 { 0 } else { 1 + sizeof_varint(*(&self.modifiedAt) as u64) }
        + if self.stateId == "" { 0 } else { 1 + sizeof_len((&self.stateId).len()) }
        + if self.replyToMessageId == "" { 0 } else { 1 + sizeof_len((&self.replyToMessageId).len()) }
        + self.message.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.attachments.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.reactions.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + if self.read == false { 0 } else { 1 + sizeof_varint(*(&self.read) as u64) }
        + if self.mentionRead == false { 0 } else { 1 + sizeof_varint(*(&self.mentionRead) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != "" { w.write_with_tag(10, |w| w.write_string(&**&self.id))?; }
        if self.orderId != "" { w.write_with_tag(18, |w| w.write_string(&**&self.orderId))?; }
        if self.creator != "" { w.write_with_tag(26, |w| w.write_string(&**&self.creator))?; }
        if self.createdAt != 0i64 { w.write_with_tag(32, |w| w.write_int64(*&self.createdAt))?; }
        if self.modifiedAt != 0i64 { w.write_with_tag(72, |w| w.write_int64(*&self.modifiedAt))?; }
        if self.stateId != "" { w.write_with_tag(90, |w| w.write_string(&**&self.stateId))?; }
        if self.replyToMessageId != "" { w.write_with_tag(42, |w| w.write_string(&**&self.replyToMessageId))?; }
        if let Some(ref s) = self.message { w.write_with_tag(50, |w| w.write_message(s))?; }
        for s in &self.attachments { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.reactions { w.write_with_tag(66, |w| w.write_message(s))?; }
        if self.read != false { w.write_with_tag(80, |w| w.write_bool(*&self.read))?; }
        if self.mentionRead != false { w.write_with_tag(96, |w| w.write_bool(*&self.mentionRead))?; }
        Ok(())
    }
}

pub mod mod_ChatMessage {

use std::borrow::Cow;
use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageContent<'a> {
    pub text: Cow<'a, str>,
    pub style: model::mod_Block::mod_Content::mod_Text::Style,
    pub marks: Vec<model::mod_Block::mod_Content::mod_Text::Mark<'a>>,
}

impl<'a> MessageRead<'a> for MessageContent<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.style = r.read_enum(bytes)?,
                Ok(26) => msg.marks.push(r.read_message::<model::mod_Block::mod_Content::mod_Text::Mark>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MessageContent<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
        + if self.style == anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { 0 } else { 1 + sizeof_varint(*(&self.style) as u64) }
        + self.marks.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != "" { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        if self.style != anytype::model::mod_Block::mod_Content::mod_Text::Style::Paragraph { w.write_with_tag(16, |w| w.write_enum(*&self.style as i32))?; }
        for s in &self.marks { w.write_with_tag(26, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Attachment<'a> {
    pub target: Cow<'a, str>,
    pub type_pb: model::mod_ChatMessage::mod_Attachment::AttachmentType,
}

impl<'a> MessageRead<'a> for Attachment<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.target = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Attachment<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.target == "" { 0 } else { 1 + sizeof_len((&self.target).len()) }
        + if self.type_pb == anytype::model::mod_ChatMessage::mod_Attachment::AttachmentType::FILE { 0 } else { 1 + sizeof_varint(*(&self.type_pb) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.target != "" { w.write_with_tag(10, |w| w.write_string(&**&self.target))?; }
        if self.type_pb != anytype::model::mod_ChatMessage::mod_Attachment::AttachmentType::FILE { w.write_with_tag(16, |w| w.write_enum(*&self.type_pb as i32))?; }
        Ok(())
    }
}

pub mod mod_Attachment {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttachmentType {
    FILE = 0,
    IMAGE = 1,
    LINK = 2,
}

impl Default for AttachmentType {
    fn default() -> Self {
        AttachmentType::FILE
    }
}

impl From<i32> for AttachmentType {
    fn from(i: i32) -> Self {
        match i {
            0 => AttachmentType::FILE,
            1 => AttachmentType::IMAGE,
            2 => AttachmentType::LINK,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for AttachmentType {
    fn from(s: &'a str) -> Self {
        match s {
            "FILE" => AttachmentType::FILE,
            "IMAGE" => AttachmentType::IMAGE,
            "LINK" => AttachmentType::LINK,
            _ => Self::default(),
        }
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Reactions<'a> {
    pub reactions: KVMap<Cow<'a, str>, model::mod_ChatMessage::mod_Reactions::IdentityList<'a>>,
}

impl<'a> MessageRead<'a> for Reactions<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_message::<model::mod_ChatMessage::mod_Reactions::IdentityList>(bytes)?))?;
                    msg.reactions.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Reactions<'a> {
    fn get_size(&self) -> usize {
        0
        + self.reactions.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).get_size()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.reactions.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).get_size()), 10, |w| w.write_string(&**k), 18, |w| w.write_message(v)))?; }
        Ok(())
    }
}

pub mod mod_Reactions {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct IdentityList<'a> {
    pub ids: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for IdentityList<'a> {
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

impl<'a> MessageWrite for IdentityList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ids.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ids { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

}

}
