use super::FromBlockContent;
use crate::proto::anytype::model;
use crate::proto::anytype::model::mod_Block::mod_Content::Bookmark as RawBookmark;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkComponentAttr {
    pub url: String,
    pub title: String,
    pub description: String,
    pub image_hash: String,
    pub favicon_hash: String,
    #[serde(rename = "type")]
    pub bookmark_type: BookmarkType,
    pub target_object_id: String,
}

pub type BookmarkType = model::mod_LinkPreview::Type;

impl<'life> FromBlockContent<RawBookmark<'life>> for BookmarkComponentAttr {
    fn from_block_content(
        raw: &RawBookmark<'life>,
    ) -> Result<BookmarkComponentAttr, anyhow::Error> {
        // let raw_clone = raw.clone();
        let tmp = BookmarkComponentAttr {
            url: raw.url.to_string(),
            title: raw.title.to_string(),
            description: raw.description.to_string(),
            image_hash: raw.imageHash.to_string(),
            favicon_hash: raw.faviconHash.to_string(),
            target_object_id: raw.targetObjectId.to_string(),
            bookmark_type: raw.type_pb.clone(),
            ..BookmarkComponentAttr::default()
        };
        return Ok(tmp);
    }
}
