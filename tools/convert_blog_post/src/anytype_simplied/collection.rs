use std::collections::BTreeMap;

use anyhow::{Error, Ok};

use crate::{
    anytype::object::AnytypeObject,
    anytype_proto::{anytype::SnapshotWithType, trait_impl::to_val_string},
};

use super::page::{Page, PageId, PageMap};

// use super::article::Article;

pub type CollectionId = String;
pub type CollectionMap = BTreeMap<CollectionId, Box<Collection>>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Collection {
    pub id: CollectionId,
    pub name: String,
    pub description: String,
    pub cover: String,
    pub styles: Vec<String>,

    pub articale_id_list: Vec<PageId>,
    pub articles: PageMap,
}

impl Collection {
    #[deprecated]
    pub fn from_anytype(raw: &AnytypeObject) -> Self {
        let data = raw.snapshot.data.details.clone();

        let mut articale_id_list: Vec<String> = vec![];
        if let Some(collections) = raw.snapshot.data.collections.clone() {
            articale_id_list = collections.objects.unwrap_or_default();
        }
        Self {
            id: data.id.clone(),
            name: data.name.unwrap_or_default().clone(),
            description: data.description.unwrap_or_default().clone(),
            // cover: data.cover.unwrap_or_default().clone(),
            // styles: data.styles.unwrap_or_default().clone(),
            // articale_id_list: raw.snapshot.data.collections.unwrap_or_default().objects
            articale_id_list,
            ..Default::default()
        }
    }

    pub fn from_anytype_proto(raw: &SnapshotWithType) -> Result<Self, Error> {
        let mut tmp = Self::default();
        // let data = raw.snapshot.data.details.clone();
        let ref data_map = raw
            .snapshot
            .as_ref()
            .unwrap()
            .data
            .as_ref()
            .unwrap()
            .details
            .as_ref()
            .unwrap()
            .fields;

        if let Some(id) = data_map.get("id") {
            tmp.id = to_val_string(&id)?;
        }
        if let Some(name) = data_map.get("name") {
            tmp.name = to_val_string(&name)?;
        }
        if let Some(description) = data_map.get("description") {
            tmp.description = to_val_string(&description)?;
        }

        // let mut articale_id_list: Vec<String> = vec![];
        // if let Some(collections) = raw.snapshot.data.collections.clone() {
        //     articale_id_list = collections.objects.unwrap_or_default();
        // }
        Ok(tmp)
    }

    pub fn insert_articles<T>(&mut self, files: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = Page>,
    {
        for file in files.into_iter() {
            if file.collection_id == self.id {
                self.articale_id_list.push(file.id.to_owned());
                self.articles.insert(file.id.to_owned(), Box::new(file));
            }
        }
        Ok(())
    }
}
