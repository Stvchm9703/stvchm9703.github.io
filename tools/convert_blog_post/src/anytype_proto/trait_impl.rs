// use super::models;

// use models::

// extern crate prost_types;
extern crate serde;

use std::collections::BTreeMap;

use anyhow::{Ok, Result, anyhow};
// use prost_types::value::Kind::{BoolValue, ListValue, NumberValue, StringValue, StructValue};
use super::google_protobuf as prost_types;

use super::anytype_model::block;

// use crate::anytype::trait_impl::ToTokenString;

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

// pub struct ProstTypeStruct(pub prost_types::Struct);

// impl serde::Serialize for ProstTypeStruct {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(self.0.fields.len()))?;
//         for (key, value) in &self.0.fields {
//             map.serialize_entry(key, &ProstTypeValue(value.clone()))?;
//         }
//         map.end()
//     }
// }

// pub struct ProstTypeList(pub prost_types::ListValue);

// impl serde::Serialize for ProstTypeList {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(self.0.values.len()))?;
//         for value in &self.0.values {
//             seq.serialize_element(&ProstTypeValue(value.clone()))?;
//         }
//         seq.end()
//     }
// }

// pub struct ProstTypeValue(pub prost_types::Value);

// impl serde::Serialize for ProstTypeValue {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self.0.kind.as_ref() {
//             Some(NumberValue(i)) => serializer.serialize_f64(*i),
//             Some(BoolValue(i)) => serializer.serialize_bool(*i),
//             Some(StringValue(i)) => serializer.serialize_str(i),
//             Some(StructValue(i)) => ProstTypeStruct(i.clone()).serialize(serializer),
//             Some(ListValue(i)) => ProstTypeList(i.clone()).serialize(serializer),
//             Some(NullValue) => serializer.serialize_none(),
//             None => serializer.serialize_none(),

//             _ => Err(serde::ser::Error::custom("Unsupported type")),
//         }
//     }
// }

pub fn to_val_string(value: &prost_types::Value) -> Result<String, anyhow::Error> {
    let str = serde_json::to_value(value.to_owned())?.to_string();
    Ok(str.replace("\"", ""))
}

pub fn to_val_number(value: &prost_types::Value) -> Result<f64, anyhow::Error> {
    let str = serde_json::to_value(value.to_owned())?;
    let number = str
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("Invalid number"))?;
    Ok(number)
}
// pub fn to_val_bool(value: &prost_types::Value) -> Result<bool, anyhow::Error> {
//     let str = serde_json::to_value(value.to_owned())?;
//         .as_bool()
//         .ok_or_else(|| anyhow::anyhow!("Invalid boolean"))?;
//     Ok(boolean)
// }

pub fn to_val_map(value: &prost_types::Struct) -> Result<BTreeMap<String, String>, anyhow::Error> {
    let mut result = BTreeMap::new();
    for (key, value) in value.fields.iter() {
        let key = key.replace("\"", "");
        let value = to_val_string(&value)?;
        result.insert(key, value);
    }
    Ok(result)
}

pub fn get_field_value(
    field_map: &prost_types::Struct,
    field_name: &str,
) -> Result<String, anyhow::Error> {
    match field_map.fields.get(field_name) {
        Some(value) => to_val_string(value),
        None => {
            println!("field name: <{}> not found", field_name);
            println!(
                "hint: check the field name list and fix the field name : {:?}",
                field_map.fields.keys()
            );
            Err(anyhow!("field name: {} not found", field_name))
        }
    }
}

pub fn get_field_value_list(
    field_map: &prost_types::Struct,
    field_name: &str,
) -> Result<Vec<String>, anyhow::Error> {
    match field_map.fields.get(field_name) {
        Some(value) => match value.kind.as_ref() {
            Some(prost_types::value::Kind::ListValue(list)) => {
                let mut result = Vec::new();
                for item in list.values.iter() {
                    result.push(to_val_string(item)?);
                }
                Ok(result)
            }
            _ => Err(anyhow!("field name: {} not found", field_name)),
        },
        None => {
            println!("field name: <{}> not found", field_name);
            println!(
                "hint: check the field name list and fix the field name : {:?}",
                field_map.fields.keys()
            );
            Err(anyhow!("field name: {} not found", field_name))
        }
    }
}

mod test {
    use std::collections::{BTreeMap, HashMap};

    use crate::anytype_proto::{
        google_protobuf::{Struct, Value, value},
        trait_impl::{to_val_map, to_val_number, to_val_string},
    };

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

    #[test]
    fn test_to_val_map() {
        let mut value = Struct {
            fields: HashMap::new(),
        };
        value.fields.insert(
            "test".to_string(),
            Value {
                kind: Some(value::Kind::StringValue("test".to_string())),
            },
        );
        value.fields.insert(
            "number".to_string(),
            Value {
                kind: Some(value::Kind::NumberValue(42.0)),
            },
        );

        let result = to_val_map(&value);
        assert!(result.is_ok());
        let expected = BTreeMap::from([
            ("test".to_string(), "test".to_string()),
            ("number".to_string(), "42.0".to_string()),
        ]);
        println!("outcome {:?}", result);
        assert_eq!(result.unwrap(), expected);
    }
}
// ----------
pub trait ToTokenString {
    fn to_token_string(&self) -> String;
}

impl ToTokenString for block::Content {
    fn to_token_string(&self) -> String {
        match self {
            block::Content::Smartblock(_) => "Smartblock".to_string(),
            block::Content::Text(_) => "Text".to_string(),
            block::Content::File(_) => "File".to_string(),
            block::Content::Layout(_) => "Layout".to_string(),
            block::Content::Div(_) => "Div".to_string(),
            block::Content::Bookmark(_) => "Bookmark".to_string(),
            block::Content::Icon(_) => "Icon".to_string(),
            block::Content::Link(_) => "Link".to_string(),
            block::Content::Dataview(_) => "Dataview".to_string(),
            block::Content::Relation(_) => "Relation".to_string(),
            block::Content::FeaturedRelations(_) => "FeaturedRelations".to_string(),
            block::Content::Latex(_) => "Latex".to_string(),
            block::Content::TableOfContents(_) => "TableOfContents".to_string(),
            block::Content::Table(_) => "Table".to_string(),
            block::Content::TableColumn(_) => "TableColumn".to_string(),
            block::Content::TableRow(_) => "TableRow".to_string(),
            block::Content::Widget(_) => "Widget".to_string(),
            block::Content::Chat(_) => "Chat".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
