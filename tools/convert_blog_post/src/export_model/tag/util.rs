use crate::export_model::{
    common::set_relation_name_map,
    // option::TagOption,
    page::{Page, ext::PageExternalLink},
    // tag::Tag,
};
use lo_::chunk;
// use option::TagOption;
use serde::{Deserialize, Serialize};

use super::{Tag, option::TagOption};
pub fn resolve_tag_option<'a>(
    tag_list: &'a mut Vec<Tag>,
    tag_opt_list: &'a Vec<TagOption>,
) -> &'a Vec<Tag> {
    for tag in tag_list.iter_mut() {
        let tag_rel: Vec<TagOption> = tag_opt_list
            .into_iter()
            .filter(|x| x.relation_key == tag.relation_key)
            .map(|x| x.to_owned())
            .collect();

        tag.options = tag_rel.to_owned();

        set_relation_name_map(tag);
    }

    return tag_list;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagIndexList {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub name: String,
    pub description: String,
    pub total_count: i32,
    pub total_page_number: Option<i32>,
    pub page_index: Option<i32>,
    pub result_list: Vec<PageExternalLink>,
}

const PAGE_SIZE: usize = 50;

pub fn generate_tag_index(tag_set: &Tag, page_list: &Vec<Page>) -> Vec<TagIndexList> {
    let mut output: Vec<TagIndexList> = vec![];

    let mut tag_option_index = tag_set
        .options
        .iter()
        .map(|x| TagIndexList {
            id: x.id.to_string(),
            sid: x.sid.to_string(),
            name: x.name.to_string(),
            description: x.description.to_string(),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    println!("total option in tag : {:?}", tag_option_index.len());
    println!("review page :{:?}", page_list.len());

    for tag_item in tag_option_index.iter_mut() {
        let mut included = page_list
            .iter()
            .filter(|x| x.raw_tag_list.contains(&tag_item.id))
            .collect::<Vec<_>>();
        included.sort_by(|a, b| b.publish_date.cmp(&a.publish_date));
        tag_item.total_count = included.len() as i32;
        let paged = chunk(included, PAGE_SIZE);

        let total_page_number: i32 = paged.len() as i32;
        for (page_idx, sub_page_list) in paged.iter().enumerate() {
            let mut tag_item_page = tag_item.clone();
            tag_item_page.total_page_number = Some(total_page_number.to_owned());
            tag_item_page.page_index = Some(page_idx as i32);
            tag_item_page.result_list = sub_page_list
                .iter()
                .map(|x| x.to_post_card_link())
                .collect::<Vec<_>>();

            output.push(tag_item_page);
        }
        // tag_item.
        // chunk(untaggedSerie, 150);
    }
    println!("total option pages in tag : {:?}", output.len());

    return output;
}

pub fn generate_serie_index(tag_set: &Tag, page_list: &Vec<Page>) -> Vec<TagIndexList> {
    let mut output: Vec<TagIndexList> = vec![];

    let mut tag_option_index = tag_set
        .options
        .iter()
        .map(|x| TagIndexList {
            id: x.id.to_string(),
            sid: x.sid.to_string(),
            name: x.name.to_string(),
            description: x.description.to_string(),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    println!("total option in tag : {:?}", tag_option_index.len());
    println!("review page :{:?}", page_list.len());

    for tag_item in tag_option_index.iter_mut() {
        let mut included = page_list
            .iter()
            .filter(|&x| x.serie.as_ref().is_some_and(|y| y.id == tag_item.id))
            .collect::<Vec<_>>();
        included.sort_by(|a, b| b.publish_date.cmp(&a.publish_date));
        tag_item.total_count = included.len() as i32;
        let paged = chunk(included, PAGE_SIZE);

        let total_page_number: i32 = paged.len() as i32;
        for (page_idx, sub_page_list) in paged.iter().enumerate() {
            let mut tag_item_page = tag_item.clone();
            tag_item_page.total_page_number = Some(total_page_number.to_owned());
            tag_item_page.page_index = Some(page_idx as i32);
            tag_item_page.result_list = sub_page_list
                .iter()
                .map(|x| x.to_post_card_link())
                .collect::<Vec<_>>();

            output.push(tag_item_page);
        }
        // tag_item.
        // chunk(untaggedSerie, 150);
    }
    println!("total option pages in tag : {:?}", output.len());

    return output;
}
