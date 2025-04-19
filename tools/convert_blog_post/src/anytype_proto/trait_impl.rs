// use super::models;

// use models::

extern crate prost_types;
extern crate serde;

use std::collections::BTreeMap;

use anyhow::Result;
use prost_types::value::Kind::{BoolValue, ListValue, NumberValue, StringValue, StructValue};
use serde::{
    // Deserialize, Serialize,
    ser::{SerializeMap, SerializeSeq, SerializeStruct, Serializer},
};

// impl serde::Serialize for prost_types::Value {

//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         if self.kind.is_none() {
//             serializer.serialize_none();
//         }
//         match self.kind.unwrap() {
//             NumberValue(i) => serializer.serialize_f64(*i),
//             BoolValue(i) => serializer.serialize_bool(i),
//             StructValue(i) => serializer.serialize_map(Some(i.fields.len())),
//             ListValue(i) => serializer.serialize_seq(Some(i.values.len())),
//             NullValue => serializer.serialize_none(),
//             _ => Err(serde::ser::Error::custom("Unsupported type")),
//         }
//     }
// }

pub struct ProstTypeStruct(pub prost_types::Struct);

impl serde::Serialize for ProstTypeStruct {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.fields.len()))?;
        for (key, value) in &self.0.fields {
            map.serialize_entry(key, &ProstTypeValue(value.clone()))?;
        }
        map.end()
    }
}

pub struct ProstTypeList(pub prost_types::ListValue);

impl serde::Serialize for ProstTypeList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.values.len()))?;
        for value in &self.0.values {
            seq.serialize_element(&ProstTypeValue(value.clone()))?;
        }
        seq.end()
    }
}

pub struct ProstTypeValue(pub prost_types::Value);

impl serde::Serialize for ProstTypeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self.0.kind.as_ref() {
            Some(NumberValue(i)) => serializer.serialize_f64(*i),
            Some(BoolValue(i)) => serializer.serialize_bool(*i),
            Some(StringValue(i)) => serializer.serialize_str(i),
            Some(StructValue(i)) => ProstTypeStruct(i.clone()).serialize(serializer),
            Some(ListValue(i)) => ProstTypeList(i.clone()).serialize(serializer),
            Some(NullValue) => serializer.serialize_none(),
            None => serializer.serialize_none(),
            _ => Err(serde::ser::Error::custom("Unsupported type")),
        }
    }
}

pub fn to_val_string(value: &prost_types::Value) -> Result<String, anyhow::Error> {
    let str = serde_json::to_value(ProstTypeValue(value.to_owned()))?.to_string();
    Ok(str.replace("\"", ""))
}

pub fn to_val_number(value: &prost_types::Value) -> Result<f64, anyhow::Error> {
    let str = serde_json::to_value(ProstTypeValue(value.to_owned()))?;
    let number = str
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("Invalid number"))?;
    Ok(number)
}
pub fn to_val_bool(value: &prost_types::Value) -> Result<bool, anyhow::Error> {
    let str = serde_json::to_value(ProstTypeValue(value.to_owned()))?;
    let boolean = str
        .as_bool()
        .ok_or_else(|| anyhow::anyhow!("Invalid boolean"))?;
    Ok(boolean)
}

pub fn to_val_map(value: &prost_types::Struct) -> Result<BTreeMap<String, String>, anyhow::Error> {
    let map = serde_json::to_value(ProstTypeStruct(value.to_owned()))?;
    let map = map
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Invalid map"))?;
    let mut result = BTreeMap::new();
    for (key, value) in map {
        let key = key.replace("\"", "");
        let value = value
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid value"))?;
        result.insert(key, value.to_string());
    }
    Ok(result)
}

mod test {
    use super::*;
    

    #[test]
    fn test_to_val_string() {
        let value = Value {
            kind: Some(value::Kind::StringValue("test".to_string())),
        };
        let result = to_val_string(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test".to_string());
    }

    #[test]
    fn test_to_val_number() {
        let value = Value {
            kind: Some(value::Kind::NumberValue(42.0)),
        };
        let result = to_val_number(&value);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42.0);
    }
}
// ----------
