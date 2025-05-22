use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::{
    export_model::trait_impl::FromRaw,
    proto::anytype::model::{
        Range as RawRange,
        mod_Block::mod_Content::mod_Text::{Mark as RawMark, mod_Mark},
    },
};

// text span make-up
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mark {
    pub range: MarkRange,
    #[serde(rename = "type")]
    pub mark_type: MarkType,
    pub param: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkRange {
    pub from: usize,
    pub to: usize,
}
impl FromRaw<RawRange> for MarkRange {
    fn from_raw(raw: &RawRange) -> Result<MarkRange, Error> {
        return Ok(MarkRange {
            from: raw.from as usize,
            to: raw.to as usize,
            ..MarkRange::default()
        });
    }
}
pub type MarkType = mod_Mark::Type;

impl<'a> FromRaw<RawMark<'a>> for Mark {
    fn from_raw(raw: &RawMark) -> Result<Mark, Error> {
        let mut tmp = Mark {
            mark_type: raw.type_pb.to_owned(),
            ..Default::default()
        };

        if let Some(rangee) = &raw.range {
            tmp.range = MarkRange::from_raw(rangee)?;
        }

        if raw.param.is_empty() == false {
            tmp.param = Some(raw.param.to_string());
        }
        return Ok(tmp);
    }
}
