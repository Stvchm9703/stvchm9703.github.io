



// impl Serialize for Smartblock {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(1))?;
//         map.serialize_entry("$type", "Smartblock")?;
//         map.end()
//     }
// }

// impl Serialize for Text {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut map = serializer.serialize_map(Some(2))?;
//         map.serialize_entry(
//             "$type",
//             text::Style::from_i32(self.style).unwrap().as_str_name(),
//         )?;

//         let simp = BTreeMap::from(self);
//         map.serialize_entry("text", &self.text)?;
//         map.end()
//     }
// }
