use std::collections::BTreeMap;
pub type AttributeMap = BTreeMap<String, String>;

pub mod get_style;

fn merge_attributes(left: &AttributeMap, right: &AttributeMap) -> AttributeMap {
    let mut result = left.clone();
    for (key, value) in right {
        result.insert(key.clone(), value.clone());
    }

    result
}
