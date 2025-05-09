// Automatically generated rust module for 'struct.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use std::collections::HashMap;
type KVMap<K, V> = HashMap<K, V>;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;

use serde::ser::{Serialize, SerializeMap, SerializeSeq,SerializeStruct,Serializer};

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NullValue {
    NULL_VALUE = 0,
}

impl Default for NullValue {
    fn default() -> Self {
        NullValue::NULL_VALUE
    }
}

impl From<i32> for NullValue {
    fn from(i: i32) -> Self {
        match i {
            0 => NullValue::NULL_VALUE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for NullValue {
    fn from(s: &'a str) -> Self {
        match s {
            "NULL_VALUE" => NullValue::NULL_VALUE,
            _ => Self::default(),
        }
    }
}

impl serde::Serialize for NullValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer {
            serializer.serialize_none()
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Struct<'a> {
    pub fields: KVMap<Cow<'a, str>, protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for Struct<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => {
                    let (key, value) = r.read_map(bytes, |r, bytes| Ok(r.read_string(bytes).map(Cow::Borrowed)?), |r, bytes| Ok(r.read_message::<protobuf::Value>(bytes)?))?;
                    msg.fields.insert(key, value);
                }
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Struct<'a> {
    fn get_size(&self) -> usize {
        0
        + self.fields.iter().map(|(k, v)| 1 + sizeof_len(2 + sizeof_len((k).len()) + sizeof_len((v).get_size()))).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for (k, v) in self.fields.iter() { w.write_with_tag(10, |w| w.write_map(2 + sizeof_len((k).len()) + sizeof_len((v).get_size()), 10, |w| w.write_string(&**k), 18, |w| w.write_message(v)))?; }
        Ok(())
    }
}

impl<'a> serde::Serialize for Struct<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer
    {
            let mut mapper = serializer.serialize_map(Some(self.fields.len()))?;
            for (k, v) in self.fields.clone() {
                mapper.serialize_entry(&k.to_owned(), &v)?;
            }
            mapper.end()

        }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Value<'a> {
    pub kind: protobuf::mod_Value::OneOfkind<'a>,
}

impl<'a> MessageRead<'a> for Value<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.kind = protobuf::mod_Value::OneOfkind::null_value(r.read_enum(bytes)?),
                Ok(17) => msg.kind = protobuf::mod_Value::OneOfkind::number_value(r.read_double(bytes)?),
                Ok(26) => msg.kind = protobuf::mod_Value::OneOfkind::string_value(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(32) => msg.kind = protobuf::mod_Value::OneOfkind::bool_value(r.read_bool(bytes)?),
                Ok(42) => msg.kind = protobuf::mod_Value::OneOfkind::struct_value(r.read_message::<protobuf::Struct>(bytes)?),
                Ok(50) => msg.kind = protobuf::mod_Value::OneOfkind::list_value(r.read_message::<protobuf::ListValue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Value<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.kind {
            protobuf::mod_Value::OneOfkind::null_value(ref m) => 1 + sizeof_varint(*(m) as u64),
            protobuf::mod_Value::OneOfkind::number_value(_) => 1 + 8,
            protobuf::mod_Value::OneOfkind::string_value(ref m) => 1 + sizeof_len((m).len()),
            protobuf::mod_Value::OneOfkind::bool_value(ref m) => 1 + sizeof_varint(*(m) as u64),
            protobuf::mod_Value::OneOfkind::struct_value(ref m) => 1 + sizeof_len((m).get_size()),
            protobuf::mod_Value::OneOfkind::list_value(ref m) => 1 + sizeof_len((m).get_size()),
            protobuf::mod_Value::OneOfkind::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.kind {            protobuf::mod_Value::OneOfkind::null_value(ref m) => { w.write_with_tag(8, |w| w.write_enum(*m as i32))? },
            protobuf::mod_Value::OneOfkind::number_value(ref m) => { w.write_with_tag(17, |w| w.write_double(*m))? },
            protobuf::mod_Value::OneOfkind::string_value(ref m) => { w.write_with_tag(26, |w| w.write_string(&**m))? },
            protobuf::mod_Value::OneOfkind::bool_value(ref m) => { w.write_with_tag(32, |w| w.write_bool(*m))? },
            protobuf::mod_Value::OneOfkind::struct_value(ref m) => { w.write_with_tag(42, |w| w.write_message(m))? },
            protobuf::mod_Value::OneOfkind::list_value(ref m) => { w.write_with_tag(50, |w| w.write_message(m))? },
            protobuf::mod_Value::OneOfkind::None => {},
    }        Ok(())
    }
}

impl<'a> serde::Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer {
            match self.kind.to_owned() {
                mod_Value::OneOfkind::null_value(_) => serializer.serialize_none(),
                mod_Value::OneOfkind::number_value(v) => serializer.serialize_f64(v),
                mod_Value::OneOfkind::string_value(cow) => serializer.serialize_str(cow.to_string().as_str()),
                mod_Value::OneOfkind::bool_value(b) => serializer.serialize_bool(b),
                mod_Value::OneOfkind::struct_value(sv) => sv.serialize(serializer),
                mod_Value::OneOfkind::list_value(list_value) => list_value.serialize(serializer),
                mod_Value::OneOfkind::None => serializer.serialize_none(),
            }
        }
}


pub mod mod_Value {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfkind<'a> {
    null_value(protobuf::NullValue),
    number_value(f64),
    string_value(Cow<'a, str>),
    bool_value(bool),
    struct_value(protobuf::Struct<'a>),
    list_value(protobuf::ListValue<'a>),
    None,
}

impl<'a> Default for OneOfkind<'a> {
    fn default() -> Self {
        OneOfkind::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ListValue<'a> {
    pub values: Vec<protobuf::Value<'a>>,
}

impl<'a> MessageRead<'a> for ListValue<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.values.push(r.read_message::<protobuf::Value>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ListValue<'a> {
    fn get_size(&self) -> usize {
        0
        + self.values.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.values { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

impl<'a> serde::Serialize for ListValue<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer {
            self.values.serialize(serializer)
        }

}
