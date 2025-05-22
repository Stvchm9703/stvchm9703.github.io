pub mod attr;
pub mod ext;
pub mod meta;
pub mod util;
use super::{
    common::{
        AttributeMap,
        // DEFAULT_TAG,
        get_field_value,
        get_shorten_id,
        get_snapshot_shorthanded,
        is_release,
        path_resolver,
    },
    content_block::{
        ComponentAttrType, ContentBlock,
        layout::{LayoutComponentAttr, LayoutItem},
        text::{TextComponentAttr, TextStyle},
    },
    // external_link, file_object,
    trait_impl::{FromBlock, FromRaw, FromSnapshotList},
};
use crate::{
    export_model::{
        common::{GLOBAL_RELATION_NAMEMAP, GLOBAL_RELATION_NAMEMAP_STR},
        // content_block::text::TextItem,
        // content_block::text::TextItemComponentAttr,
    },
    proto::anytype::SnapshotWithType,
};

use std::{
    // borrow::{Borrow, BorrowMut},
    collections::BTreeMap,
};

use anyhow::{Error, anyhow};
use attr::get_page_details;
use convert_blog_post_marco::set_field_value;
use ext::PageExternalLink;
// use lo_::intersection;
use meta::PageMeta;
use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    #[serde(rename = "_sid")]
    pub sid: String,
    pub title: String,
    // pub description: String,
    pub snippet: String,
    pub collection_id: String,
    pub workspace_id: String,
    // pub attributes: PageAttributes,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_attributes: AttributeMap,
    pub creator: String,
    pub publish_date: i64,
    pub tags: Vec<PageExternalLink>,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_tag_list: Vec<String>,
    pub styles: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "is_release")]
    pub raw_series: Option<Vec<String>>,
    pub serie: Option<PageExternalLink>,
    // page other relation / attr
    pub meta: PageMeta,
    pub table_of_contents: Vec<PageExternalLink>,
    pub related_chapters: Vec<PageExternalLink>,
    pub related_articles: Vec<PageExternalLink>,
    // content-block
    #[serde(skip)]
    pub cache_contents: BTreeMap<String, ContentBlock>,
    #[serde(skip_serializing_if = "is_release")]
    pub check_block_order: Vec<String>,

    pub contents: Vec<ContentBlock>,
}

impl<'a> FromRaw<SnapshotWithType<'a>> for Page {
    fn from_raw(input: &SnapshotWithType) -> Result<Page, anyhow::Error> {
        let mut tmp = Page::default();
        let instance = get_snapshot_shorthanded(input);
        if let Err(e) = instance {
            return Err(anyhow!("fail field map : {:?}", e));
        }
        let (data_wrap, field_map) = instance.unwrap();
        tmp.raw_attributes = field_map.clone();
        // let field_map_ref = field_map;
        let field_try_map = get_page_details(&data_wrap);
        if let Err(e) = field_try_map {
            // eprintln!("{:?}", e);
            return Err(anyhow!("fail page-attributes : {:?}", e));
        }
        let field_attr = field_try_map.unwrap();

        // if let Ok(id) = get_field_value::<String>(field_map_ref, "id") {
        tmp.id = field_attr.id.to_owned();
        tmp.sid = get_shorten_id(&field_attr.id);
        // };
        // if let Ok(title) = get_field_value::<String>(field_map_ref, "name") {
        tmp.title = field_attr.name.to_owned();
        // }
        // tmp.attributes = field_attr.to_owned();
        // if let Ok(backlink) =  get_field_value::<Vec<String>>>(field_map_ref, "backlinks"){
        if field_attr.backlinks.len() > 0 {
            tmp.collection_id = field_attr.backlinks.into_iter().last().unwrap().to_owned();
        }

        // tmp.workspace_id = getFieldValue(detailMap, "spaceId") ?? "";
        tmp.workspace_id = field_attr.space_id;

        // tmp.creator = getFieldValue(detailMap, "creator") ?? "";
        tmp.creator = field_attr.creator;
        // tmp.snippet = getFieldValue(detailMap, "snippet") ?? "";
        tmp.snippet = field_attr.snippet;
        // tmp.raw_tag_list = getFieldValue(detailMap, "tag") ?? [];
        if let Some(tag) = field_attr.tag {
            tmp.raw_tag_list = tag;
        }
        // tmp.publish_date = getFieldValue<number>(detailMap, "publish date");
        if let Ok(publish_date) = get_field_value::<i64>(&field_map, "publish date") {
            tmp.publish_date = publish_date.to_owned();
        }

        for (idx, block) in data_wrap.blocks.iter().enumerate() {
            let block_tmp = ContentBlock::from_block_with_idx(block, idx);
            if let Ok(blk) = block_tmp {
                tmp.cache_contents.insert(blk.id.to_owned(), blk.clone());
            } else {
                println!("error in page-> content-block : {:?}", block_tmp.err());
            }
        }

        return Ok(tmp);
    }
}

impl FromSnapshotList for Page {
    fn from_snapshot_list(raw: Vec<SnapshotWithType>) -> Result<Vec<Page>, anyhow::Error> {
        let mut tmp_ls: Vec<Page> = vec![];

        for y in raw.into_iter() {
            let tmp = Page::from_raw(&y);
            if let Err(e) = tmp {
                return Err(anyhow!("list init error : {:?} , {:?}", e, &y));
            }
            tmp_ls.push(tmp.unwrap());
        }

        return Ok(tmp_ls);
    }
}

#[derive(Debug, Default)]
struct PageTextListSet {
    pub style: TextStyle,
    pub start: usize,
    pub end: usize,
    pub is_header: bool,
    pub start_id: String,
    pub sebering_id_list: Vec<String>,
    pub start_child: Vec<String>,
    pub layer: usize,
}

impl PageTextListSet {
    pub fn is_sebering(&self, style: &TextStyle, order: &usize, layer: &usize) -> bool {
        return &self.style == style && order > &self.end && &self.layer == layer;
    }

    pub fn push_next(&mut self, blk: &ContentBlock) -> Result<(), Error> {
        if blk.order - self.start < self.sebering_id_list.len() + 1 {
            return Err(anyhow!("unresovled"));
        }

        self.end = blk.order.to_owned();
        self.sebering_id_list.push(blk.id.to_owned());
        // self.4 = layer;
        //
        return Ok(());
    }
}

type PageTextListMainSet = SmallVec<[PageTextListSet; 128]>;
impl Page {
    pub fn recheck_fields(&mut self) {
        let mut publish_date: f64 = 0.0;
        set_field_value!(publish_date, self.raw_attributes, "publish date");
        self.publish_date = publish_date as i64;
        let mut series: Vec<String> = Vec::<String>::new();
        set_field_value!(series, self.raw_attributes, "Series");
        self.raw_series = Some(series.to_owned());
        if let Some(rel_key) = GLOBAL_RELATION_NAMEMAP_STR.lock().unwrap().get("Series") {
            if let Some(serie_id) = series.first() {
                if let Some(tag) = GLOBAL_RELATION_NAMEMAP.lock().unwrap().get(rel_key) {
                    self.serie = tag.get_option_page_link(serie_id);
                }
            }
        }

        // GLOBAL_RELATION_NAMEMAP.

        // println!("GLOBAL_RELATION_IDMAP {:?}", GLOBAL_RELATION_IDMAP);
        // println!("GLOBAL_RELATION_NAMEMAP {:?}", GLOBAL_RELATION_NAMEMAP);
        // println!( "GLOBAL_RELATION_NAMEMAP_STR {:?}", GLOBAL_RELATION_NAMEMAP_STR );
    }

    pub fn resolve_content_block(&mut self) {
        let root_block = self.cache_contents.get(&self.id);
        if root_block.is_none() {
            return;
        }
        let check_id = self.id.to_owned();
        println!("page-id: {:?}", check_id);
        let mut order_list_ptr = vec![];
        self.resolve_children_ids_order(&check_id, &mut order_list_ptr);

        self.resolve_nest_children(&order_list_ptr);
        // self.transform_text_list(current_block, text_sebering, &current_layer);

        return;
    }
    // #[deprecated]
    fn resolve_children_ids_order(&mut self, check_id: &str, order_list_ptr: &mut Vec<String>) {
        let root_block = self.cache_contents.get(check_id);
        if root_block.is_none() {
            return;
        }

        let root_blk = root_block.unwrap().to_owned();

        if root_blk.id == "header" || root_blk.id == "title" || root_blk.id == "featuredrelations" {
            return;
        }

        // layer check and filter children
        let mut layout_comp_is_div: bool = false;
        let mut layout_comp_is_layered: bool = false;
        let mut is_layout_comp: bool = false;
        if let ComponentAttrType::Layout(l) = &root_blk.component_attr {
            is_layout_comp == true;
            layout_comp_is_div = (l.layout_style == "Div");
            layout_comp_is_layered = (l.layout_style != "Div");
        }
        let mut text_comp_is_toggle: bool = false;
        let mut text_comp_is_layer_child: bool = false;
        if let ComponentAttrType::Text(t) = &root_blk.component_attr {
            text_comp_is_toggle = t.style == TextStyle::Toggle;
            if t.style == TextStyle::Marked || t.style == TextStyle::Numbered {
                text_comp_is_layer_child =
                    root_blk.children_ids.as_ref().is_some_and(|f| f.len() > 0);
            }
        }
        // let default_skip_component

        if root_blk.debug_type != "div"
            && root_blk.debug_type != "relation"
            && root_blk.debug_type != "table_of_contents"
            && root_blk.debug_type != "smartblock"
            && (root_blk.debug_type == "layout" && layout_comp_is_div == true) == false
        {
            order_list_ptr.push(check_id.to_string());
        }

        // skip the children
        if root_blk.debug_type == "table"
            || (root_blk.debug_type == "layout" && layout_comp_is_layered)
            || text_comp_is_toggle
            || text_comp_is_layer_child
        {
            return;
        }

        let chil_ids = root_blk.children_ids.to_owned().unwrap_or_default();
        for elm in chil_ids {
            self.resolve_children_ids_order(&elm, order_list_ptr);
        }

        if (root_blk.debug_type == "layout" && layout_comp_is_div) || root_blk.debug_type == "div" {
            //     // order_list.push(check_id.to_owned());
            order_list_ptr.push(check_id.to_string());
        }

        // drop(root_blk);

        return;
    }

    // type TmpTextListSizer = ;

    fn resolve_nest_children(&mut self, order_list: &Vec<String>) {
        let mut tmp_cache_contents = self.cache_contents.to_owned();
        let mut tmp_text_mark_and_number: PageTextListMainSet = smallvec![];

        if let Some(root_blk) = self.cache_contents.get(&self.id) {
            // add default layer
            tmp_text_mark_and_number.push(PageTextListSet {
                style: TextStyle::Title,
                is_header: true,
                start_id: root_blk.id.to_owned(),
                start_child: root_blk.children_ids.to_owned().unwrap_or_default(),
                ..PageTextListSet::default()
            });
        }

        for id in order_list {
            let ct_blk = tmp_cache_contents.get_mut(id);

            if let Some(blk) = ct_blk {
                self.resolve_internal_children_pipeline(blk, &mut tmp_text_mark_and_number, 1);
                self.contents.push(blk.to_owned());
            }
        }
        self.transform_text_list(&mut tmp_text_mark_and_number);
    }

    fn resolve_internal_children_pipeline(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        self.resolve_layout_nest_children(current_block, text_sebering, current_layer);
        self.resolve_text_nest_children(current_block, text_sebering, current_layer);
    }

    fn resolve_text_nest_children(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        let attr: &mut TextComponentAttr;
        match &mut current_block.component_attr {
            ComponentAttrType::Text(e) => attr = e,
            _ => return,
        }
        if [TextStyle::Marked, TextStyle::Numbered, TextStyle::Toggle].contains(&attr.style)
            == false
        {
            return;
        }
        // println!(in );

        let lk_children_ids = current_block.children_ids.as_ref().unwrap();
        let mut cached = self.cache_contents.to_owned();
        // println!("attr : {:#?}", attr);

        // if attr.style == TextStyle::Toggle {
        for child_id in lk_children_ids.iter() {
            if let Some(child_blk) = cached.get_mut(child_id) {
                if child_blk.children_ids.is_some() {
                    self.resolve_internal_children_pipeline(
                        child_blk,
                        text_sebering,
                        current_layer + 1,
                    );
                }
            }
        }
        // self.transform_text_list(current_block, text_sebering);

        // return;
        // }

        if attr.style == TextStyle::Numbered || attr.style == TextStyle::Marked {
            let mut sebering = text_sebering
                .iter_mut()
                .find(|p| p.is_sebering(&attr.style, &current_block.order, &current_layer));
            if let Some(last_item) = sebering {
                // if last_item.is_sebering(&attr.style, &current_block.order, &current_layer) {
                // find next?

                last_item.push_next(&current_block);
            } else {
                text_sebering.push(PageTextListSet {
                    style: attr.style,
                    is_header: false,
                    start: current_block.order,
                    start_id: current_block.id.to_owned(),
                    start_child: lk_children_ids.to_vec(),
                    end: current_block.order,
                    sebering_id_list: vec![current_block.id.to_owned()],
                    layer: current_layer,
                });
            }
            // }
        }
        // println!("text_sebering - snapshot: {:?}", text_sebering);
    }
    fn transform_text_list(&mut self, text_sebering: &mut PageTextListMainSet) {
        if text_sebering.len() == 0 {
            return;
        }
        // println!("text_sebering : {:#?}", text_sebering);
        // do the insertion
        if self.id != "bafyreialni2kwfzmrbytsbg3xl4zwug4mp4vbxxri5tcpdddtrfraexhha" {
            return;
        }

        println!("page_set : ");
        text_sebering.sort_by(|a, b| b.layer.cmp(&a.layer));
        // println!("layer : {}", current_layer);
        // println!("{}", current_layer);
        for page_set in text_sebering.iter() {
            // let id_list = page_set.3;

            // if let Some(original_text_item) = self.cache_contents.get(&page_set.start_id) {
            //     // create parent
            //     let mut tmp = TextItem::from_content_block(&original_text_item);

            //     // for subitem in page_set.start_child.iter() {
            //     //     if let Some(original__item) = self.cache_contents.get(&subitem) {}
            //     // }
            // }
            println!(" {:#?}", page_set);
        }

        // text_sebering.clear();
    }
    fn resolve_layout_nest_children(
        &mut self,
        current_block: &mut ContentBlock,
        text_sebering: &mut PageTextListMainSet,
        current_layer: usize,
    ) {
        let attr: &mut LayoutComponentAttr;
        match &mut current_block.component_attr {
            ComponentAttrType::Layout(e) | ComponentAttrType::Table(e) => attr = e,
            _ => {
                // println!("skip resolve_layout_nest_children");
                return;
            }
        }

        let lk_children_ids = current_block.children_ids.as_ref().unwrap();
        let mut cached = self.cache_contents.to_owned();

        for child_id in lk_children_ids.into_iter() {
            let child_blk_result = cached.get_mut(child_id);
            if let Some(child_blk) = child_blk_result {
                if child_blk.children_ids.is_some() {
                    self.resolve_internal_children_pipeline(
                        child_blk,
                        text_sebering,
                        current_layer + 1,
                    );
                }

                let layout_item = LayoutItem::from_content_block(child_blk);

                if let Err(e) = attr.push_item(layout_item) {
                    println!("error throw in {:?}", e);
                }
            }
        }
    }
}
