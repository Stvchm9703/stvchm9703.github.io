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

impl<'life> FromBlockContent<RawBookmark<'life>, BookmarkComponentAttr> for BookmarkComponentAttr {
    fn from_block_content(
        raw: &RawBookmark<'life>,
    ) -> Result<BookmarkComponentAttr, anyhow::Error> {
        let raw_clone = raw.clone();
        let tmp = BookmarkComponentAttr {
            url: raw_clone.url.into_owned(),
            title: raw_clone.title.into_owned(),
            description: raw_clone.description.into_owned(),
            image_hash: raw_clone.imageHash.into_owned(),
            favicon_hash: raw_clone.faviconHash.into_owned(),
            target_object_id: raw_clone.targetObjectId.into_owned(),
            bookmark_type: raw_clone.type_pb.clone(),
            ..BookmarkComponentAttr::default()
        };
        return Ok(tmp);
    }
}
